#![allow(
    clippy::undocumented_unsafe_blocks,
    reason = "Using the Vulkan API is necessarily an unsafe fest. Commenting every single one would add too much noise, actually harming \
              readability. Note that we are still able to re-enable this lint in specific places if we'd like"
)]

use alloc::sync::Weak;
use vk_mem::Alloc;
use core::fmt::Debug;
use std::collections::{HashSet};
use core::num::{NonZeroU32};
use core::mem::ManuallyDrop;
use core::ffi::CStr;
use std::sync::Mutex;

use ash::vk;

use crate::debugger::breakpoint;
use crate::delegates::MulticastDelegateResult;
use crate::gpu::{ApiArc, ApiImpl, ApiInner, ApiParams, DeviceImpl, DeviceParams, Result, SwapChainImpl, SwapChainParams};
use crate::result_hole;
use crate::weak_self::sync::WeakSelf;
use crate::windowing::WindowArc;

// https://registry.khronos.org/vulkan/specs/latest/man/html/PFN_vkDebugReportCallbackEXT.html
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

    if message_id_name == "Loader Message" && message_id_number == 0 {
        return vk::FALSE;
    }

    if message_severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
        // https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/general_considerations.html#general_considerations_validation_layer_warnings

        // It happens when VK_KHR_dedicated_allocation extension is enabled. vkGetBufferMemoryRequirements2KHR function is used instead, while validation layer seems to be unaware of it.
        if message.contains("vkBindBufferMemory(): Binding memory to buffer ") && message.contains(" but vkGetBufferMemoryRequirements() has not been called on that buffer") {
            return vk::FALSE;
        }

        // It happens when you map a buffer or image, because the library maps entire VkDeviceMemory block, where different types of images and buffers may end up together, especially on GPUs with unified memory like Intel.
        if message.contains("Mapping an image with layout VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL can result in undefined behavior if this memory is used by the device. Only GENERAL or PREINITIALIZED should be used") {
            return vk::FALSE;
        }

        // It may happen when you use defragmentation.
        if message.contains("Non-linear image ") && message.contains(" is aliased with linear buffer ") && message.contains(" which may indicate a bug") {
            return vk::FALSE;
        }
    }

    if message_severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
        eprintln!("{message_severity:?}: {message_type:?} [{message_id_name} (0x{message_id_number:08x})] : {message}");
        if message_severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
            breakpoint!();
        }
    } else {
        println!("{message_severity:?}: {message_type:?} [{message_id_name} (0x{message_id_number:08x})] : {message}");
    }

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
    api_version: u32,
    supports_get_physical_device_properties2: bool,
    has_debug_utils: bool,
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
        let api_version = vk::make_api_version(0, 1, 0, 0);
        let entry = unsafe { ash::Entry::load() }.map_err(std::io::Error::other)?;
        unsafe {
            let supports_get_physical_device_properties2;
            let has_debug_utils;
            let instance = {
                // TODO: GPU API: instance layers + extensions
                let layer_names = [c"VK_LAYER_KHRONOS_validation"];
                let layer_names_raw: Vec<_> = layer_names.iter().map(|x| x.as_ptr()).collect();

                let ash_window_required_extension_names = ash_window::enumerate_required_extensions(raw_window_handle::WindowsDisplayHandle::new().into())?;
                let required_extension_names: HashSet<&CStr> = ash_window_required_extension_names.iter().map(|x| CStr::from_ptr(*x)).collect();
                let desired_extension_names: HashSet<&CStr> = [
                    ash::ext::debug_utils::NAME,
                    ash::khr::get_physical_device_properties2::NAME,
                ].into_iter().collect();

                #[cfg(any(target_os = "macos", target_os = "ios"))]
                {
                    required_extension_names.insert(ash::khr::portability_enumeration::NAME.as_ptr());
                    // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
                    required_extension_names.insert(ash::khr::get_physical_device_properties2::NAME.as_ptr());
                }

                let extension_properties = entry.enumerate_instance_extension_properties(None)?;
                let supported_extension_names: HashSet<&CStr> = extension_properties.iter().filter_map(|x| x.extension_name_as_c_str().ok()).collect();

                let enabled_extension_names: HashSet<&CStr> = required_extension_names.intersection(&supported_extension_names).copied().collect();
                if enabled_extension_names.len() < required_extension_names.len() {
                    return Err(std::io::Error::other("Some required instance extensions are not supported").into());
                }

                let enabled_extension_names: HashSet<&CStr> = enabled_extension_names.into_iter().chain(desired_extension_names.intersection(&supported_extension_names).copied()).collect();

                supports_get_physical_device_properties2 = enabled_extension_names.contains(ash::khr::get_physical_device_properties2::NAME);
                has_debug_utils = enabled_extension_names.contains(ash::ext::debug_utils::NAME);

                let enabled_extension_names: Vec<*const _> = enabled_extension_names.into_iter().map(CStr::as_ptr).collect();

                let application_info = vk::ApplicationInfo::default()
                    .api_version(api_version)
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
                    .enabled_extension_names(&enabled_extension_names)
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
                api_version,
                supports_get_physical_device_properties2,
                has_debug_utils,
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
    graphics: Vec<u32>,
    compute: Vec<u32>,
    present: Vec<u32>,
    graphics_and_present: Vec<u32>,
}

impl UsefulQueueFamilies {
    pub fn new(api: &VulkanApi, physical_device: vk::PhysicalDevice, queue_family_props: &[vk::QueueFamilyProperties]) -> Self {
        let mut out = Self::default();
        for (queue_family_index, it) in queue_family_props.iter().enumerate() {
            if it.queue_count == 0 {
                // ??? can this ever happen? I'm too afraid this can be legit
                continue;
            }

            #[expect(clippy::cast_possible_truncation, reason = "Safe")]
            let queue_family_index = queue_family_index as u32;

            let supports_presentation;

            #[cfg(windows)]
            unsafe {
                supports_presentation = api
                    .win32_surface_loader
                    .get_physical_device_win32_presentation_support(physical_device, queue_family_index);
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
    properties: vk::PhysicalDeviceProperties,
    #[expect(dead_code, reason = "This may be useful later. Not so certain since I added VMA, but let's re-evaluate this later")]
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    features: vk::PhysicalDeviceFeatures,
    useful_queue_families: UsefulQueueFamilies,
    requested_queues: Vec<MinimalQueueCreateInfo>,
}

const MAX_QUEUES_PER_FAMILY: usize = 64; // Waaaaay way more than we'll ever need

impl PhysicalDeviceWrapper {
    pub fn new(api: &VulkanApi, physical_device: vk::PhysicalDevice) -> Self {
        unsafe {
            let queue_family_props = api.instance.get_physical_device_queue_family_properties(physical_device);
            let useful_queue_families = UsefulQueueFamilies::new(api, physical_device, &queue_family_props);
            Self {
                physical_device,
                properties: api.instance.get_physical_device_properties(physical_device),
                features: api.instance.get_physical_device_features(physical_device),
                memory_properties: api.instance.get_physical_device_memory_properties(physical_device),
                useful_queue_families,
                requested_queues: vec![],
            }
        }
    }
}

struct MinimalQueueCreateInfo {
    queue_family_index: u32,
    count: NonZeroU32,
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
                    count: NonZeroU32::new(1).unwrap(),
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
            .map(|physical_device| {
                let mut wrapper = PhysicalDeviceWrapper::new(self, physical_device);
                // TODO: This should depend on params. This is why I'm keeping it seaparate for now
                wrapper.requested_queues = MinimalQueueCreateInfo::extract(&wrapper.useful_queue_families);
                wrapper
            })
            .collect();

        let physical_device_types_sorted = [
            vk::PhysicalDeviceType::DISCRETE_GPU,
            vk::PhysicalDeviceType::INTEGRATED_GPU,
            vk::PhysicalDeviceType::VIRTUAL_GPU,
            vk::PhysicalDeviceType::CPU,
            vk::PhysicalDeviceType::OTHER,
        ];

        physical_devices.sort_by(|a, b| {
            let a_type_score = physical_device_types_sorted.iter().position(|x| *x == a.properties.device_type);
            let b_type_score = physical_device_types_sorted.iter().position(|x| *x == b.properties.device_type);
            a_type_score.cmp(&b_type_score)
            // TODO: among these, try to find the best GPU. taking into account required+desired
            // features+extensions
        });

        let physical_device = physical_devices
            .into_iter()
            .find(|x| !x.requested_queues.is_empty())
            .ok_or_else(|| std::io::Error::other("No physical device matching requirements"))?;

        let supports_dedicated_allocation;
        let supports_ext_memory_budget;
        let device = {
            // TODO: GPU API trade-offs:
            // - Multiple queues?
            // - Priority per queue?
            // - Multiple devices?
            // - Multiple command buffers?
            let queue_priorities = [1.; MAX_QUEUES_PER_FAMILY];
            let queue_create_infos: Vec<_> = physical_device.requested_queues
                .iter()
                .map(|x| {
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(x.queue_family_index)
                        .queue_priorities(&queue_priorities[..core::cmp::min(x.count.get() as _, queue_priorities.len())])
                })
                .collect();

            let desired_extension_names: HashSet<&CStr> = [
                // https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/vk_khr_dedicated_allocation.html
                c"VK_KHR_get_memory_requirements2",
                c"VK_KHR_dedicated_allocation",
                // https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/staying_within_budget.html
                c"VK_EXT_memory_budget",
            ].into_iter().collect();

            let extension_properties = unsafe { self.instance.enumerate_device_extension_properties(physical_device.physical_device) }?;
            let supported_extension_names: HashSet<&CStr> = extension_properties.iter().map(|x| x.extension_name_as_c_str().unwrap_or_default()).collect();
            let enabled_extension_names: HashSet<&CStr> = desired_extension_names.intersection(&supported_extension_names).copied().collect();
            let enabled_extension_names_pointers: Vec<*const _> = enabled_extension_names.iter().map(|x| x.as_ptr()).collect();

            supports_dedicated_allocation = enabled_extension_names.contains(c"VK_KHR_get_memory_requirements2") && enabled_extension_names.contains(c"VK_KHR_dedicated_allocation");
            supports_ext_memory_budget = self.supports_get_physical_device_properties2 && enabled_extension_names.contains(c"VK_EXT_memory_budget");

            // TODO: GPU API: device extensions + features
            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&enabled_extension_names_pointers)
                .enabled_features(&physical_device.features); // PERF: Vulkan book recommends not enable all features blindly because that may cause unnecessary allocations

            unsafe {
                self.instance
                    .create_device(physical_device.physical_device, &device_create_info, self.allocator.as_ref())?
            }
        };

        let debug_utils_loader = self.has_debug_utils.then(|| ash::ext::debug_utils::Device::new(&self.instance, &device));

        let vma_allocator = {
            let mut create_info = vk_mem::AllocatorCreateInfo::new(&self.instance, &device, physical_device.physical_device);
            create_info.allocation_callbacks = self.allocator.as_ref();

            // HACK due to vk_mem looking for the device's 1_1 function table instead of allowing us to provide the extension function loader.
            // Loading "vkGetBufferMemoryRequirements2KHR" (from the extension) is not the same as "vkGetBufferMemoryRequirements2" (core 1.1), and vk_mem is not aware of that. I should perhaps fork it or submit a PR.
            let is_1_1 = vk::api_version_major(self.api_version) > 1 || (vk::api_version_major(self.api_version) == 1 && vk::api_version_minor(self.api_version) >= 1);

            if supports_dedicated_allocation && is_1_1 {
                create_info.flags |= vk_mem::AllocatorCreateFlags::KHR_DEDICATED_ALLOCATION;
            }
            if supports_ext_memory_budget && is_1_1 {
                create_info.flags |= vk_mem::AllocatorCreateFlags::EXT_MEMORY_BUDGET;
            }
            create_info.vulkan_api_version = self.api_version;
            ManuallyDrop::new(unsafe { vk_mem::Allocator::new(create_info) }?)
        };

        Ok(Box::new(VulkanDevice {
            api: self.api_arc(),
            device,
            physical_device,
            vma_allocator,
            debug_utils_loader,
        }))
    }
}

pub struct VulkanDevice {
    api: ApiArc,
    device: ash::Device,
    #[expect(dead_code, reason = "This will certainly be useful later")]
    physical_device: PhysicalDeviceWrapper,
    vma_allocator: ManuallyDrop<vk_mem::Allocator>,
    debug_utils_loader: Option<ash::ext::debug_utils::Device>,
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
            ManuallyDrop::drop(&mut self.vma_allocator);
            self.device.destroy_device(allocator.as_ref());
        }
    }
}

impl DeviceImpl for VulkanDevice {
    fn create_swap_chain(&self, _params: &SwapChainParams) -> Result<Box<dyn SwapChainImpl>> {
        // TODO
        todo!()
    }
    fn test_upload_large_buffer(&self) -> Result<()> {
        let size = 16 * 1024_usize;
        unsafe {
            let (buffer, mut allocation) = self.vma_allocator.create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(size as u64)
                    .usage(vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::VERTEX_BUFFER)
                ,
                &vk_mem::AllocationCreateInfo {
                    usage: vk_mem::MemoryUsage::Auto,
                    flags: vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
                    user_data: 0_usize, // TODO
                    ..Default::default()
                }
            )?;

            self.debug_utils_loader.as_ref().inspect(|debug_utils_loader| {
                let name_info = vk::DebugUtilsObjectNameInfoEXT {
                    object_type: vk::ObjectType::BUFFER,
                    ..Default::default()
                }
                    .object_handle(buffer)
                    .object_name(c"Test buffer");

                result_hole::add(debug_utils_loader.set_debug_utils_object_name(&name_info));
            });

            // TODO: vk_mem doesn't expose this function supported by VMA. Consider adding it?
            // self.vma_allocator.set_allocation_name(allocation, c"TODO");

            let ptr = self.vma_allocator.map_memory(&mut allocation).inspect_err(|_| {
                self.vma_allocator.destroy_buffer(buffer, &mut allocation);
            })?;

            // TODO: do this on multiple threads?
            core::ptr::write_bytes(ptr, 0xbe, size);

            self.vma_allocator.unmap_memory(&mut allocation);

            result_hole::add(self.vma_allocator.flush_allocation(&allocation, 0, size as u64));

            self.vma_allocator.destroy_buffer(buffer, &mut allocation);

        };
        Ok(())
    }
    fn set_frame_index(&self, frame_index: u64) -> Result<()> {
        #[expect(clippy::cast_possible_truncation, reason = "This is unfortunate, but we're not about to change VMA's API")]
        unsafe { self.vma_allocator.set_current_frame_index(frame_index as u32); };
        Ok(())
    }
}
