extern crate winapi;
extern crate xinput_dyn;

use xinput_dyn::{LoadedXInputApi, XINPUT_DLL_NAMES};
use winapi::winerror::*;
use winapi::xinput::*;

use std::mem::uninitialized;
//use std::mem::zeroed as zero;
use std::mem::transmute;
use std::time::Duration;
use std::thread::sleep;
use std::ffi::CStr;

mod fix {
    use winapi::*;
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    #[repr(C)]
    pub struct XINPUT_KEYSTROKE {
        pub VirtualKey: WORD,
        pub Unicode: WCHAR,
        pub Flags: WORD,
        pub UserIndex: BYTE,
        pub HidCode: BYTE,
    }    
}

fn main() {
    let result = LoadedXInputApi::load_best();
    if result.is_err() {
        eprintln!("Could not load any XInput DLL!");
        std::process::exit(1);
    };
    let (api, i) = result.unwrap();
    println!("Settled on {}", 
        CStr::from_bytes_with_nul(XINPUT_DLL_NAMES[i])
            .unwrap().to_string_lossy()
    );

    if api.XInputGetState.is_none() {
        eprintln!("XInputGetState is not available!");
        std::process::exit(1);
    }

    if api.XInputGetKeystroke.is_none() {
        eprintln!("XInputGetKeystroke is not available!");
        std::process::exit(1);
    }

    'main_event_loop: loop {
        let player_idx = 0;
        let mut event: fix::XINPUT_KEYSTROKE = unsafe { uninitialized() };

        'poll: loop {
            let status = unsafe {
                api.XInputGetKeystroke.unwrap()(player_idx, 0, transmute(&mut event as *mut _))
	        };
            match status {
                // XXX This returns ERROR_SUCCESS and zeroes the struct 
                // when used prior to plugging a controller in.
                // After unplugging a controller, it returns ERROR_EMPTY.
                // See https://stackoverflow.com/q/23669238/7972165
                // XInput 1.4
                ERROR_SUCCESS => {
		    print_keystroke(&event);
            //sleep(Duration::from_millis(500));
		    continue 'poll;
                },
                ERROR_EMPTY => {
                    eprintln!("No event left for this frame!");
                },
                ERROR_DEVICE_NOT_CONNECTED => {
                    eprintln!("Xbox Controller n°{} is not connected!", player_idx);
                },
                _ => {
                    eprintln!("Unhandled error {:X}", status);
                },
            }
	    break 'poll;
	}
        sleep(Duration::from_millis(100));
    }
}

fn print_keystroke(keystroke: &fix::XINPUT_KEYSTROKE) {
    let mut code_string = String::new();
    let vkey = match keystroke.VirtualKey {
        VK_PAD_A => "A",
        VK_PAD_B => "B",
        VK_PAD_X => "X",
        VK_PAD_Y => "Y", 
        VK_PAD_RSHOULDER => "Right shoulder",
        VK_PAD_LSHOULDER => "Left shoulder",
        VK_PAD_LTRIGGER	=> "Left trigger",
        VK_PAD_RTRIGGER => "Right trigger",
        VK_PAD_DPAD_UP => "Dpad up ↑",
        VK_PAD_DPAD_DOWN => "Dpad down ↓",
        VK_PAD_DPAD_LEFT => "Dpad left ←",
        VK_PAD_DPAD_RIGHT => "Dpad right →",
        VK_PAD_START => "Start",
        VK_PAD_BACK	=> "Back",
        VK_PAD_LTHUMB_PRESS => "Left thumbstick click",
        VK_PAD_RTHUMB_PRESS => "Right thumbstick click",
        VK_PAD_LTHUMB_UP        => "Left thumbstick up ↑",
        VK_PAD_LTHUMB_DOWN      => "Left thumbstick down ↓",
        VK_PAD_LTHUMB_RIGHT     => "Left thumbstick right →",
        VK_PAD_LTHUMB_LEFT      => "Left thumbstick left ←",
        VK_PAD_LTHUMB_UPLEFT    => "Left thumbstick up-left ↖",
        VK_PAD_LTHUMB_UPRIGHT   => "Left thumbstick up-right ↗",
        VK_PAD_LTHUMB_DOWNRIGHT => "Left thumbstick down-right ↘",
        VK_PAD_LTHUMB_DOWNLEFT  => "Left thumbstick down-left ↙",
        VK_PAD_RTHUMB_UP        => "Right thumbstick up ↑",
        VK_PAD_RTHUMB_DOWN      => "Right thumbstick down ↓",
        VK_PAD_RTHUMB_RIGHT     => "Right thumbstick right →",
        VK_PAD_RTHUMB_LEFT      => "Right thumbstick left ←",
        VK_PAD_RTHUMB_UPLEFT    => "Right thumbstick up-left ↖",
        VK_PAD_RTHUMB_UPRIGHT   => "Right thumbstick up-right ↗",
        VK_PAD_RTHUMB_DOWNRIGHT => "Right thumbstick down-right ↘",
        VK_PAD_RTHUMB_DOWNLEFT  => "Right thumbstick down-left ↙",
        code @ _ => {
            code_string = format!("{}", code);
            ""
        },
    };
    // TODO: document that it reports joystick if only halfway through:
    // Can't count on this for accurate joystick events
    // XInput 1.4
    println!("n°{}, VKey: {}, down: {}, up: {}, repeat: {}", 
        keystroke.UserIndex,
        if vkey.is_empty() { &code_string } else { vkey } ,
        keystroke.Flags & XINPUT_KEYSTROKE_KEYDOWN != 0,
        keystroke.Flags & XINPUT_KEYSTROKE_KEYUP != 0,
        keystroke.Flags & XINPUT_KEYSTROKE_REPEAT != 0,
    );
}


/*
    loop {
        let player_idx = 0;
        let (state, status) = unsafe {
            let mut state: XINPUT_STATE = uninitialized();
            let status = api.XInputGetState.unwrap()(player_idx, &mut state);
            (state, status)
        };
        match status {
            ERROR_SUCCESS => {
                let pad = state.Gamepad;
                let vib: u16;
                if (pad.wButtons & XINPUT_GAMEPAD_A) != 0 {
                    println!("Pressing A!");
                    vib = std::u16::MAX;
                } else {
                    println!("Not pressing A");
                    vib = 0;
                }
                let mut vibration = XINPUT_VIBRATION {
                    wLeftMotorSpeed: vib,
                    wRightMotorSpeed: vib,
                };
                unsafe {
                    api.XInputSetState.unwrap()(player_idx, &mut vibration);
                }
            },
            ERROR_DEVICE_NOT_CONNECTED => {
                eprintln!("Xbox Controller n°{} is not connected!", player_idx);
            },
            _ => {
                eprintln!("Unhandled error {:X}", status);
            },
        }
        sleep(Duration::from_millis(100));
    }
    */
