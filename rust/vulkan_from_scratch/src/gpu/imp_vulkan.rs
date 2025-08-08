#![allow(
    clippy::undocumented_unsafe_blocks,
    reason = "Using the Vulkan API is necessarily an unsafe fest. Commenting every single one would add too much noise, actually harming \
              readability. Note that we are still able to re-enable this lint in specific places if we'd like"
)]

use alloc::sync::Weak;
use core::fmt::Debug;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use ash::vk;

use crate::delegates::MulticastDelegateResult;
use crate::gpu::{ApiArc, ApiImpl, ApiInner, ApiParams, DeviceImpl, DeviceParams, Result, SwapChainImpl, SwapChainParams};
use crate::result_hole;
use crate::weak_self::sync::WeakSelf;
use crate::windowing::WindowArc;

extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = unsafe { *p_callback_data };
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        alloc::borrow::Cow::from("")
    } else {
        unsafe { core::ffi::CStr::from_ptr(callback_data.p_message_id_name) }.to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        alloc::borrow::Cow::from("")
    } else {
        unsafe { core::ffi::CStr::from_ptr(callback_data.p_message) }.to_string_lossy()
    };

    println!("{message_severity:?}: {message_type:?} [{message_id_name} ({message_id_number})] : {message}");

    vk::FALSE
}

pub struct VulkanApi {
    api_weak: WeakSelf<ApiInner>,
    #[expect(dead_code, reason = "ash::Entry must not be dropped, otherwise further calls will crash")]
    entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::khr::surface::Instance,
    #[cfg(windows)]
    win32_surface_loader: ash::khr::win32_surface::Instance,
    surfaces: Mutex<HashSet<vk::SurfaceKHR>>,
    debug_utils_loader: ash::ext::debug_utils::Instance,
    debug_utils_messenger: vk::DebugUtilsMessengerEXT,
    allocator: Option<vk::AllocationCallbacks<'static>>,
}

impl Debug for VulkanApi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VulkanApi")
            .field("instance", &self.instance.handle())
            .finish_non_exhaustive()
    }
}

impl Drop for VulkanApi {
    fn drop(&mut self) {
        unsafe {
            for surface in self.surfaces.lock().unwrap().iter() {
                self.surface_loader.destroy_surface(*surface, self.allocator.as_ref());
            }
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_utils_messenger, self.allocator.as_ref());
            self.instance.destroy_instance(self.allocator.as_ref());
        }
    }
}

impl VulkanApi {
    fn api_arc(&self) -> ApiArc {
        ApiArc(self.api_weak.upgrade_unwrap())
    }
    pub fn create(_params: &ApiParams) -> Result<Self> {
        let allocator = None;
        let entry = unsafe { ash::Entry::load() }.map_err(std::io::Error::other)?;
        unsafe {
            let instance = {
                // TODO: GPU API: instance layers + extensions
                let layer_names = [c"VK_LAYER_KHRONOS_validation"];
                let layer_names_raw: Vec<_> = layer_names.iter().map(|x| x.as_ptr()).collect();
                let mut extension_names =
                    ash_window::enumerate_required_extensions(raw_window_handle::WindowsDisplayHandle::new().into())?.to_vec();
                extension_names.push(ash::ext::debug_utils::NAME.as_ptr());
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                {
                    extension_names.push(ash::khr::portability_enumeration::NAME.as_ptr());
                    // Enabling this extension is a requirement when using
                    // `VK_KHR_portability_subset`
                    extension_names.push(ash::khr::get_physical_device_properties2::NAME.as_ptr());
                }
                let application_info = vk::ApplicationInfo::default()
                    .api_version(vk::make_api_version(0, 1, 0, 0))
                    .application_name(c"VulkanFromScratch")
                    .application_version(0)
                    .engine_name(c"NoEngine")
                    .engine_version(0);
                let create_info = vk::InstanceCreateInfo::default()
                    .flags(if cfg!(any(target_os = "macos", target_os = "ios")) {
                        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
                    } else {
                        vk::InstanceCreateFlags::default()
                    })
                    .application_info(&application_info)
                    .enabled_extension_names(&extension_names)
                    .enabled_layer_names(&layer_names_raw);

                entry
                    .create_instance(&create_info, allocator.as_ref())
                    .map_err(std::io::Error::other)?
            };

            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE, // TODO
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));

            let debug_utils_loader = ash::ext::debug_utils::Instance::new(&entry, &instance);
            let debug_utils_messenger = debug_utils_loader.create_debug_utils_messenger(&debug_info, allocator.as_ref())?;

            let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

            #[cfg(windows)]
            let win32_surface_loader = ash::khr::win32_surface::Instance::new(&entry, &instance);

            Ok(Self {
                api_weak: WeakSelf::new(),
                entry,
                instance,
                surface_loader,
                #[cfg(windows)]
                win32_surface_loader,
                debug_utils_loader,
                debug_utils_messenger,
                allocator,
                surfaces: Mutex::new(HashSet::new()),
            })
        }
    }
    #[expect(dead_code, reason = "This is a draft")]
    #[expect(
        clippy::panic,
        reason = "The behavior when creating a surface twice on the same window is not well-defined"
    )]
    fn test_create_surface(&self, window: &WindowArc) {
        let surface = self.create_surface(window).unwrap();
        if self.surfaces.lock().unwrap().insert(surface) {
            let api_weak = self.api_weak.weak().clone();
            window.0.post_destroy_confirmed.lock().unwrap().push(Box::new(move |()| {
                if let Some(api) = api_weak.upgrade() {
                    let this = api.imp.as_any().downcast_ref::<Self>().unwrap();
                    // TODO: should also wait for in-flight work to be idle??
                    // In fact any call to DestroyWindow() is unsafe when there is work in-flight for it...
                    // We should have our own delegate for the "1st chance destroy", and for the "last chance destroy"
                    // (WM_DESTROY)
                    unsafe {
                        this.surface_loader.destroy_surface(surface, this.allocator.as_ref());
                    };
                    this.surfaces.lock().unwrap().remove(&surface);
                }
                MulticastDelegateResult::Remove
            }));
        } else {
            panic!("Surface was already existing??");
        }
    }
    fn create_surface(&self, window: &WindowArc) -> Result<vk::SurfaceKHR> {
        #[cfg(windows)]
        unsafe {
            let surface_create_info = vk::Win32SurfaceCreateInfoKHR {
                hinstance: window.display().hinstance().0.addr().cast_signed(),
                hwnd: window.hwnd().0.addr().cast_signed(),
                ..Default::default()
            };
            Ok(self
                .win32_surface_loader
                .create_win32_surface(&surface_create_info, self.allocator.as_ref())?)
        }
    }
}

#[derive(Default)]
struct UsefulQueueFamilies {
    graphics: Vec<usize>,
    compute: Vec<usize>,
    present: Vec<usize>,
    graphics_and_present: Vec<usize>,
}

impl UsefulQueueFamilies {
    pub fn new(api: &VulkanApi, physical_device: vk::PhysicalDevice, queue_family_props: &[vk::QueueFamilyProperties]) -> Self {
        let mut out = Self::default();
        for (queue_family_index, it) in queue_family_props.iter().enumerate() {
            if it.queue_count == 0 {
                // ??? can this ever happen? I'm too afraid this can be legit
                continue;
            }

            let supports_presentation;

            #[cfg(windows)]
            #[expect(
                clippy::cast_possible_truncation,
                reason = "Let's be realistic, there won't be more than 2^32 queue families"
            )]
            unsafe {
                supports_presentation = api
                    .win32_surface_loader
                    .get_physical_device_win32_presentation_support(physical_device, queue_family_index as u32);
            };

            if it.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                out.graphics.push(queue_family_index);
                if supports_presentation {
                    out.graphics_and_present.push(queue_family_index);
                }
            }

            if it.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                out.compute.push(queue_family_index);
            }

            if supports_presentation {
                out.present.push(queue_family_index);
            }
        }
        out
    }
}

struct PhysicalDeviceWrapper {
    physical_device: vk::PhysicalDevice,
    props: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    useful_queue_families: UsefulQueueFamilies,
}

const MAX_QUEUES_PER_FAMILY: usize = 64; // Waaaaay way more than we'll ever need

impl PhysicalDeviceWrapper {
    pub fn new(api: &VulkanApi, physical_device: vk::PhysicalDevice) -> Self {
        unsafe {
            let queue_family_props = api.instance.get_physical_device_queue_family_properties(physical_device);
            let useful_queue_families = UsefulQueueFamilies::new(api, physical_device, &queue_family_props);
            Self {
                physical_device,
                props: api.instance.get_physical_device_properties(physical_device),
                features: api.instance.get_physical_device_features(physical_device),
                useful_queue_families,
            }
        }
    }
}

struct MinimalQueueCreateInfo {
    queue_family_index: usize,
    count: usize,
}

impl MinimalQueueCreateInfo {
    // TODO: should take into account caller params. Right now very simple, pick the 1st
    // graphics_and_present queue
    pub fn extract(useful_queue_families: &UsefulQueueFamilies) -> Vec<Self> {
        useful_queue_families
            .graphics_and_present
            .first()
            .copied()
            .map_or_else(Vec::new, |queue_family_index| {
                vec![Self {
                    queue_family_index,
                    count: 1,
                }]
            })
    }
}

impl ApiImpl for VulkanApi {
    fn set_weak_api(&self, weak_api: Weak<ApiInner>) {
        self.api_weak.init(weak_api);
    }
    fn create_device(&self, _params: &DeviceParams) -> Result<Box<dyn DeviceImpl>> {
        // TODO: GPU API: better device selector + command-line/env options
        let mut physical_devices: Vec<PhysicalDeviceWrapper> = unsafe { self.instance.enumerate_physical_devices() }?
            .into_iter()
            .map(|physical_device| PhysicalDeviceWrapper::new(self, physical_device))
            .collect();

        let physical_device_types_sorted = [
            vk::PhysicalDeviceType::DISCRETE_GPU,
            vk::PhysicalDeviceType::INTEGRATED_GPU,
            vk::PhysicalDeviceType::VIRTUAL_GPU,
            vk::PhysicalDeviceType::CPU,
            vk::PhysicalDeviceType::OTHER,
        ];

        physical_devices.sort_by(|a, b| {
            let a_type_score = physical_device_types_sorted.iter().position(|x| *x == a.props.device_type);
            let b_type_score = physical_device_types_sorted.iter().position(|x| *x == b.props.device_type);
            a_type_score.cmp(&b_type_score)
            // TODO: among these, try to find the best GPU. taking into account required+desired
            // features+extensions
        });

        let required_queues: HashMap<vk::PhysicalDevice, Vec<MinimalQueueCreateInfo>> = physical_devices
            .iter()
            .map(|physical_device| {
                (
                    physical_device.physical_device,
                    MinimalQueueCreateInfo::extract(&physical_device.useful_queue_families),
                )
            })
            .collect();

        let chosen_physical_device = physical_devices
            .iter()
            .find(|x| !required_queues[&x.physical_device].is_empty())
            .ok_or_else(|| std::io::Error::other("No physical device matching requirements"))?;

        let device = {
            // TODO: GPU API trade-offs:
            // - Multiple queues?
            // - Priority per queue?
            // - Multiple devices?
            // - Multiple command buffers?
            let queue_priorities = [1.; MAX_QUEUES_PER_FAMILY];
            let queue_create_infos: Vec<_> = required_queues[&chosen_physical_device.physical_device]
                .iter()
                .map(|x| {
                    #[expect(
                        clippy::cast_possible_truncation,
                        reason = "Let's be realistic, there won't be more than 2^32 queue families"
                    )]
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(x.queue_family_index as u32)
                        .queue_priorities(&queue_priorities[..core::cmp::min(x.count as _, queue_priorities.len())])
                })
                .collect();
            // TODO: GPU API: device extensions + features
            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&[])
                .enabled_features(&chosen_physical_device.features); // PERF: Vulkan book recommends not enable all features blindly because that may cause unnecessary allocations
            unsafe {
                self.instance
                    .create_device(chosen_physical_device.physical_device, &device_create_info, self.allocator.as_ref())?
            }
        };

        Ok(Box::new(VulkanDevice {
            api: self.api_arc(),
            device,
        }))
    }
}

pub struct VulkanDevice {
    api: ApiArc,
    device: ash::Device,
}

impl Debug for VulkanDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VulkanDevice")
            .field("handle", &self.device.handle())
            .finish_non_exhaustive()
    }
}

impl VulkanDevice {
    fn api(&self) -> &VulkanApi {
        #[expect(clippy::expect_used, reason = "This really cannot fail")]
        self.api
            .0
            .imp
            .as_any()
            .downcast_ref::<VulkanApi>()
            .expect("Getting VulkanApi from VulkanDevice should never fail")
    }
}

impl Drop for VulkanDevice {
    fn drop(&mut self) {
        let allocator = self.api().allocator;
        unsafe {
            result_hole::add(self.device.device_wait_idle());
            self.device.destroy_device(allocator.as_ref());
        }
    }
}

impl DeviceImpl for VulkanDevice {
    fn create_swap_chain(&self, _params: &SwapChainParams) -> Result<Box<dyn SwapChainImpl>> {
        // TODO
        todo!()
    }
}
