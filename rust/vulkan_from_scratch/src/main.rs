extern crate ash;
extern crate winapi;

use std::{ffi::OsStr, mem::MaybeUninit};
use std::os::windows::ffi::OsStrExt;
use std::io::Error;

use ash::vk;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{DefWindowProcW, LoadCursorW, PostQuitMessage, RegisterClassW, COLOR_WINDOW, IDC_ARROW, WM_DESTROY, WNDCLASSW};
use winapi::um::{libloaderapi::GetModuleHandleW, winuser::{CreateWindowExW, DispatchMessageW, GetMessageW, ShowWindow, TranslateMessage, CW_USEDEFAULT, SW_SHOW, WS_OVERLAPPEDWINDOW}};

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        msg if msg == WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
                0
            }
        },
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
    mem_props: vk::PhysicalDeviceMemoryProperties,
    queue_family_props: Vec<vk::QueueFamilyProperties>,
}

impl VkPhysicalDeviceWrapper {
    pub fn new(vk_instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        unsafe {
            Self {
                physical_device,
                props: vk_instance.get_physical_device_properties(physical_device),
                features: vk_instance.get_physical_device_features(physical_device),
                mem_props: vk_instance.get_physical_device_memory_properties(physical_device),
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

    let hinstance = unsafe { GetModuleHandleW(std::ptr::null()) }; // Question: maybe not as simple as this? https://stackoverflow.com/a/78906765
    let window_class_name = OsStr::new("VulkanFromScratch\0").encode_wide().collect::<Vec<_>>();
    let window_title = OsStr::new("Window Title\0").encode_wide().collect::<Vec<_>>();
    unsafe {
        let wndclass = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0, 
            hInstance: hinstance,
            hIcon: std::ptr::null_mut(),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as _,
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: window_class_name.as_ptr(),
        };
        let atom = RegisterClassW(&wndclass);
        if atom == 0 {
            std::process::exit(1);
        }
    }
    let hwnd = unsafe {
        CreateWindowExW(
            0, // extended style flags
            window_class_name.as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPEDWINDOW, // style flags
            CW_USEDEFAULT, CW_USEDEFAULT, // x, y
            CW_USEDEFAULT, CW_USEDEFAULT, // width, height
            std::ptr::null_mut(), // parent window
            std::ptr::null_mut(), // menu
            hinstance,
            std::ptr::null_mut() // optional payload passed to WM_CREATE
        )
    };
    unsafe {
        ShowWindow(hwnd, SW_SHOW);
        loop {
            let mut msg = MaybeUninit::uninit();
            let has_message = GetMessageW(msg.as_mut_ptr(), std::ptr::null_mut(), 0, 0);
            if has_message == 0 {
                break;
            }
            TranslateMessage(msg.assume_init_mut());
            DispatchMessageW(msg.assume_init_mut());
        }
    }

    Ok(())
}
