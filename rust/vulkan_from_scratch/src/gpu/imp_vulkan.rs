use std::any::Any;
use std::fmt::Debug;

use ash::vk;

use crate::gpu::{ApiArc, ApiImpl, ApiParams, DeviceImpl, DeviceParams};

extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = unsafe { *p_callback_data };
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        std::borrow::Cow::from("")
    } else {
        unsafe { std::ffi::CStr::from_ptr(callback_data.p_message_id_name) }.to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        std::borrow::Cow::from("")
    } else {
        unsafe { std::ffi::CStr::from_ptr(callback_data.p_message) }.to_string_lossy()
    };

    println!(
        "{message_severity:?}: {message_type:?} [{message_id_name} ({message_id_number})] : \
         {message}"
    );

    vk::FALSE
}

struct DesiredQueueItem {
    family_index: u32,
    count: u32,
}
struct PhysicalDeviceWrapper {
    physical_device: vk::PhysicalDevice,
    props: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    // mem_props: vk::PhysicalDeviceMemoryProperties,
    queue_family_props: Vec<vk::QueueFamilyProperties>,
}

impl PhysicalDeviceWrapper {
    pub fn new(vk_instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        unsafe {
            Self {
                physical_device,
                props: vk_instance.get_physical_device_properties(physical_device),
                features: vk_instance.get_physical_device_features(physical_device),
                // mem_props: vk_instance.get_physical_device_memory_properties(physical_device),
                queue_family_props: vk_instance
                    .get_physical_device_queue_family_properties(physical_device),
            }
        }
    }
    pub fn required_queues(&self) -> Vec<DesiredQueueItem> {
        let mut out = Vec::with_capacity(1);
        for (i, it) in self.queue_family_props.iter().enumerate() {
            if it.queue_count == 0 {
                // ???
                continue;
            }
            if it
                .queue_flags
                .contains(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE)
            {
                out.push(DesiredQueueItem {
                    family_index: i as _,
                    count: 1,
                });
                break;
            }
        }
        out
    }
}

pub struct VulkanApi {
    #[allow(dead_code)] // !!! ash::Entry must not be dropped, otherwise further calls will crash
    vk: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: ash::ext::debug_utils::Instance,
    debug_utils_messenger: vk::DebugUtilsMessengerEXT,
    allocator: Option<vk::AllocationCallbacks<'static>>,
}

impl Debug for VulkanApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VulkanApi")
            .field("instance", &self.instance.handle())
            .finish_non_exhaustive()
    }
}

impl Drop for VulkanApi {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_utils_messenger, self.allocator.as_ref());
            self.instance.destroy_instance(self.allocator.as_ref());
        }
    }
}

impl VulkanApi {
    pub fn create(_params: &ApiParams) -> Result<Self, std::io::Error> {
        let allocator = None;
        let vk = unsafe { ash::Entry::load() }.expect("Failed to load Vulkan API");
        unsafe {
            let instance = {
                // TODO: GPU API: instance layers + extensions
                let layer_names = [c"VK_LAYER_KHRONOS_validation"];
                let layer_names_raw: Vec<_> = layer_names.iter().map(|x| x.as_ptr()).collect();
                let mut extension_names = ash_window::enumerate_required_extensions(
                    raw_window_handle::WindowsDisplayHandle::new().into(),
                )
                .unwrap()
                .to_vec();
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

                vk.create_instance(&create_info, allocator.as_ref())
                    .expect("Failed to create Vulkan Instance")
            };

            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));

            let debug_utils_loader = ash::ext::debug_utils::Instance::new(&vk, &instance);
            let debug_utils_messenger = debug_utils_loader
                .create_debug_utils_messenger(&debug_info, allocator.as_ref())
                .unwrap();

            Ok(Self {
                vk,
                instance,
                debug_utils_loader,
                debug_utils_messenger,
                allocator,
            })
        }
    }
}

impl ApiImpl for VulkanApi {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn create_device(
        &self,
        api_arc: &ApiArc,
        _params: &DeviceParams,
    ) -> Result<Box<dyn DeviceImpl>, std::io::Error> {
        // TODO: GPU API: better device selector + command-line/env options
        let mut physical_devices: Vec<PhysicalDeviceWrapper> =
            unsafe { self.instance.enumerate_physical_devices() }
                .unwrap()
                .into_iter()
                .map(|physical_device| PhysicalDeviceWrapper::new(&self.instance, physical_device))
                .collect();

        let physical_device_types_sorted = [
            vk::PhysicalDeviceType::DISCRETE_GPU,
            vk::PhysicalDeviceType::INTEGRATED_GPU,
            vk::PhysicalDeviceType::VIRTUAL_GPU,
            vk::PhysicalDeviceType::CPU,
            vk::PhysicalDeviceType::OTHER,
        ];

        physical_devices.sort_by(|a, b| {
            let a_type_score = physical_device_types_sorted
                .iter()
                .position(|x| *x == a.props.device_type);
            let b_type_score = physical_device_types_sorted
                .iter()
                .position(|x| *x == b.props.device_type);
            a_type_score.cmp(&b_type_score)
            // TODO: among these, try to find the best GPU. taking into account required+desired
            // features+extensions
        });

        let chosen_physical_device = physical_devices
            .iter()
            .find(|x| !x.required_queues().is_empty())
            .unwrap();

        let device = {
            // TODO: GPU API trade-offs:
            // - Multiple queues?
            // - Priority per queue?
            // - Multiple devices?
            // - Multiple command buffers?
            let queue_priorities = [1.; 64];
            let queue_create_infos: Vec<_> = chosen_physical_device
                .required_queues()
                .into_iter()
                .map(|x| {
                    assert!(queue_priorities.len() >= x.count as _);
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(x.family_index)
                        .queue_priorities(&queue_priorities[..x.count as _])
                })
                .collect();
            // TODO: GPU API: device extensions + features
            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&[])
                .enabled_features(&chosen_physical_device.features); // PERF: Vulkan book recommends not enable all features blindly because that may cause unnecessary allocations
            unsafe {
                self.instance.create_device(
                    chosen_physical_device.physical_device,
                    &device_create_info,
                    self.allocator.as_ref(),
                )
            }
            .unwrap()
        };

        Ok(Box::new(VulkanDevice {
            api: api_arc.clone(),
            device,
        }))
    }
}

pub struct VulkanDevice {
    api: ApiArc,
    device: ash::Device,
}

impl Debug for VulkanDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VulkanDevice")
            .field("handle", &self.device.handle())
            .finish_non_exhaustive()
    }
}

impl VulkanDevice {
    fn api(&self) -> &VulkanApi {
        self.api.0.imp.as_any().downcast_ref::<VulkanApi>().unwrap()
    }
}

impl Drop for VulkanDevice {
    fn drop(&mut self) {
        let allocator = self.api().allocator;
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(allocator.as_ref());
        }
    }
}

impl DeviceImpl for VulkanDevice {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
