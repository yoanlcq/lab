use alloc::sync::Arc;
use std::time::Instant;
use core::time::Duration;
use vek::{Extent2, Vec2};

use crate::gpu::{ApiArc, ApiParams, ApiSpec, DeviceParams};
use crate::windowing::{PumpEventParams, WindowParams};

extern crate alloc;

pub mod gpu;
pub mod windowing;

struct StartupProfiler {
    startup_instant: Instant,
}

impl StartupProfiler {
    pub(crate) fn new() -> Self {
        let this = Self {
            startup_instant: Instant::now(),
        };
        Self::log_exe_startup_time();
        this
    }

    fn log_exe_startup_time() {
        match std::env::current_exe() {
            Err(e) => eprintln!("Getting current exe path failed: {e}"),
            Ok(exe_path) => {
                match std::fs::metadata(exe_path) {
                    Err(e) => eprintln!("Getting current exe metadata failed: {e}"),
                    Ok(metadata) => {
                        match metadata.accessed() {
                            Err(e) => eprintln!("Last access time is not available on this file system: {e}"),
                            Ok(last_access_time) => {
                                match std::time::SystemTime::now().duration_since(last_access_time) {
                                    Err(e) => eprintln!("Failed to get elapsed time between exe last access and current time: {e}"),
                                    Ok(duration) => println!("[Startup] Time since exe last access: {:.3} seconds", duration.as_secs_f32()),
                                }
                            },
                        }
                    },
                }
            }
        }
    }

    pub(crate) fn log_step(&self, description: &str) {
        let elapsed = Instant::now().duration_since(self.startup_instant).as_secs_f32();
        println!("[Startup][{elapsed:.3}] {description}");
    }
}

struct GpuThreadResult {
    device: gpu::DeviceArc,
}

fn gpu_thread_work(startup_profiler: &StartupProfiler) -> gpu::Result<GpuThreadResult> {
    let api = {
        let zone = tracy_client::span!("gpu::api_create");
        zone.emit_color(0x4d0303);
        let api = ApiArc::create(&ApiParams { spec: ApiSpec::Vulkan })?;
        startup_profiler.log_step("GPU API created");
        api
    };

    let device = {
        let zone = tracy_client::span!("gpu::device_create");
        zone.emit_color(0x750505);
        let device = api.create_device(&DeviceParams {})?;
        startup_profiler.log_step("GPU Device created");
        device
    };

    {
        let zone = tracy_client::span!("gpu::test_upload_large_buffer");
        zone.emit_color(0x99321d);
        device.test_upload_large_buffer()?;
        startup_profiler.log_step("GPU Buffer uploaded");
    };

    // let _swapchain0 = device.create_swap_chain(&SwapChainParams { window: &window0 })?;
    // let _swapchain1 = device.create_swap_chain(&SwapChainParams { window: &window1 })?;
    Ok(GpuThreadResult { device })
}

fn make_window_params(index: u32) -> WindowParams {
    let w = 512.;
    let h = 256.;
    WindowParams {
        title: "Vulkan experiment".to_owned(),
        position: Some(Vec2::new(f64::from(index).mul_add(w, 16.), 16.)),
        size: Some(Extent2::new(w, h)),
    }
}

#[expect(clippy::missing_panics_doc, reason = "This function is for experimenting quickly")]
pub fn main() -> gpu::Result<()> {
    // StartupProfiler goes first because Tracy initialization takes some time and we want to capture that fact
    let startup_profiler = Arc::new(StartupProfiler::new());
    let _tracy_client = tracy_client::Client::start();

    let mut gpu_thread = Some({
        let startup_profiler = startup_profiler.clone();
        std::thread::Builder::new().name("GPU API thread".to_owned()).spawn(move || {
            let zone = tracy_client::span!("gpu_thread_work");
            zone.emit_color(0x210101);

            #[expect(clippy::expect_used, reason = "This is desired, we are at the top level in another thread")]
            gpu_thread_work(&startup_profiler).expect("Something failed in the GPU thread work")
        })?
    });

    let display = windowing::DisplayArc::open(&windowing::DisplayParams {})?;
    let window0 = display.create_window(&make_window_params(0))?;
    let window1 = display.create_window(&make_window_params(1))?;
    window0.show()?;
    window1.show()?;
    startup_profiler.log_step("Window showed");

    startup_profiler.log_step("Main event loop starting");

    // At 300FPS, we have ((2^64) / (300 * 60 * 60 * 24 * 365)) = 2e9 years of runtime. So don't even think about std::num::Wrapping
    let mut frame_index = 0_u64;
    let mut gpu_thread_result = None;
    loop {
        if let Some(t) = gpu_thread.take() {
            if t.is_finished() {
                gpu_thread_result = Some(t.join().map_err(|e| std::io::Error::other(format!("GPU thread panicked: {e:?}")))?);
            } else {
                gpu_thread = Some(t);
            }
        }

        let params = PumpEventParams {
            timeout: gpu_thread.is_some().then_some(Duration::from_secs_f32(1. / 144.)),
            max_events: None,
        };

        {
            let zone = tracy_client::span!("pump_events");
            zone.emit_color(0x16033d);
            if display.pump_events(&params).is_some_and(|r| r.exit_requested) {
                break;
            }
        }

        if let Some(gpu_thread_result) = gpu_thread_result.as_ref() {
            result_hole::consume!(gpu_thread_result.device.set_frame_index(frame_index));
        }

        frame_index += 1;
        tracy_client::frame_mark();
    }

    Ok(())
}
