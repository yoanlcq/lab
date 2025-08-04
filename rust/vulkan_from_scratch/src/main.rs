extern crate ash;
extern crate windows;

use std::io::Error;

use ash::vk;

use windows::Win32::Graphics::Gdi::COLOR_WINDOW;
use windows::{
    core::{w},
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::*,
    Win32::UI::WindowsAndMessaging::*,
};

extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
                LRESULT(0)
            }
        }
        _ => unsafe {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

struct VkDesiredQueueItem {
    family_index: u32,
    count: u32,
}
struct VkPhysicalDeviceWrapper {
    physical_device: vk::PhysicalDevice,
    props: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    // mem_props: vk::PhysicalDeviceMemoryProperties,
    queue_family_props: Vec<vk::QueueFamilyProperties>,
}

impl VkPhysicalDeviceWrapper {
    pub fn new(vk_instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        unsafe {
            Self {
                physical_device,
                props: vk_instance.get_physical_device_properties(physical_device),
                features: vk_instance.get_physical_device_features(physical_device),
                // mem_props: vk_instance.get_physical_device_memory_properties(physical_device),
                queue_family_props: vk_instance.get_physical_device_queue_family_properties(physical_device),
            }
        }
    }
    pub fn required_queues(&self) -> Vec<VkDesiredQueueItem> {
        let mut out = Vec::with_capacity(1);
        for (i, it) in self.queue_family_props.iter().enumerate() {
            if it.queue_count == 0 { // ???
                continue;
            }
            if it.queue_flags.contains(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE) {
                out.push(VkDesiredQueueItem { family_index: i as _, count: 1 });
                break;
            }
        }
        out
    }
}

fn main() -> Result<(), Error> {
    let allocation_callbacks = None;
    let vk = unsafe { ash::Entry::load() }.expect("Failed to load Vulkan API");
    unsafe {
        let vk_instance = {
            let layer_names = [c"VK_LAYER_KHRONOS_validation"];
            let layer_names_raw: Vec<_> = layer_names.iter().map(|x| x.as_ptr()).collect();
            let application_info = vk::ApplicationInfo::default()
                .api_version(vk::make_api_version(0, 1, 0, 0))
                .application_name(c"VulkanFromScratch")
                .application_version(1)
                .engine_name(c"NoEngine")
                .engine_version(1);
            let create_info = vk::InstanceCreateInfo::default()
                .application_info(&application_info)
                .enabled_extension_names(&[])
                .enabled_layer_names(&layer_names_raw);
            vk.create_instance(&create_info, allocation_callbacks).expect("Failed to create Vulkan Instance")
        };

        let mut physical_devices: Vec<VkPhysicalDeviceWrapper> = vk_instance.enumerate_physical_devices().unwrap().into_iter().map(|physical_device| VkPhysicalDeviceWrapper::new(&vk_instance, physical_device)).collect();

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
            // TODO: among these, try to find the best GPU. taking into account required+desired features+extensions
        });

        // TODO: DebugUtilsMessengerCreateInfoEXT

        let chosen_physical_device = physical_devices.iter().find(|x| {
            !x.required_queues().is_empty()
        }).unwrap();

        let vk_device = {
            let queue_priorities = [1.; 64];
            let queue_create_infos: Vec<_> = chosen_physical_device.required_queues().into_iter().map(|x| {
                assert!(queue_priorities.len() >= x.count as _);
                vk::DeviceQueueCreateInfo::default().queue_family_index(x.family_index).queue_priorities(&queue_priorities[.. x.count as _])
            }).collect();
            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&[])
                .enabled_features(&chosen_physical_device.features); // PERF: Vulkan book recommends not enable all features blindly because that may cause unnecessary allocations
            vk_instance.create_device(chosen_physical_device.physical_device, &device_create_info, allocation_callbacks).unwrap()
        };

        // TODO: dedicated allocation for render targer
        // TODO: generate image via dispatch compute or raytracing

        vk_device.destroy_device(allocation_callbacks);

        vk_instance.destroy_instance(None);
    }

    let hinstance = unsafe { GetModuleHandleW(None) }.unwrap(); // Question: maybe not as simple as this? https://stackoverflow.com/a/78906765
    let window_class_name = w!("VulkanFromScratch");
    let window_title = w!("Window Title");
    unsafe {
        let wndclass = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance.into(),
            hIcon: HICON::default(),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hbrBackground: GetSysColorBrush(COLOR_WINDOW),
            lpszMenuName: windows::core::PCWSTR::null(),
            lpszClassName: window_class_name,
        };
        let atom = RegisterClassW(&wndclass);
        assert_ne!(atom, 0);
    }
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(), // extended style flags
            window_class_name,
            window_title,
            WS_OVERLAPPEDWINDOW, // style flags
            CW_USEDEFAULT, CW_USEDEFAULT, // x, y
            CW_USEDEFAULT, CW_USEDEFAULT, // width, height
            None, // parent window
            None, // menu
            Some(hinstance.into()),
            None // optional payload passed to WM_CREATE
        )
    }.unwrap();
    unsafe {
        _ = ShowWindow(hwnd, SW_SHOW);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}
