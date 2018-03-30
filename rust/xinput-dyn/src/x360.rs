#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum PlayerIndex {
    Player0 = 0,
    Player1 = 1,
    Player2 = 2,
    Player3 = 3,
}

impl Default for PlayerIndex {
    fn default() -> Self {
        PlayerIndex::Player0
    }
}

// NOTE: It's written "D-pad", so no capital P here.

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(i8)]
pub enum DpadX {
    Left = -1,
    None = 0,
    Right = 1,
}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(i8)]
pub enum DpadY {
    Up = 1,
    None = 0,
    Down = -1,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dpad {
    pub x: XAxisDirection,
    pub y: YAxisDirection,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Thumb {
    x: i16,
    y: i16,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct State {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub back: bool,
    pub start: bool,
    pub dpad: Dpad,
    pub l_trigger: u8,
    pub r_trigger: u8,
    pub l_shoulder: bool,
    pub r_shoulder: bool,
    pub l_thumb: Thumb,
    pub r_thumb: Thumb,
}

use StubbedXInputApi;

pub struct PollIter<'a>(&'a StubbedXInputApi);
// TODO impl Iterator

pub enum Error {
    ControllerNotConnected,
    Code(DWORD),
}

type Result<T> = Result<T, Error>;

pub struct SafeApi {
    api: StubbedXInputApi,
}

use std::mem;
use mem::uninitialized as uninit;
use winapi::xinput as xi;
use winapi::winerror as err;

impl SafeApi {
    pub fn enable(&self) {
        self.XInputEnable(true)
    }
    pub fn disable(&self) {
        self.XInputEnable(false)
    }
    pub fn poll_iter(&'a self) -> PollIter<'a> {
        PollIter(self.api)
    }
    pub fn query_controller_state(i: PlayerIndex) -> Result<State> {
        // FIXME There's the dwPacketNumber which can tell if state has changed
        // since last query or not. Issues: 
        // - Do people really need this, or even as a boolean ?
        // - Be careful with poll_iter(), XInputGetKeystroke().
        let mut state: XINPUT_STATE = unsafe { uninit() };
        match self.api.XInputGetState(i as DWORD, &mut state) {
            err::ERROR_SUCCESS => {
                let xi::XINPUT_GAMEPAD {
                    wButtons, bLeftTrigger, bRightTrigger,
                    sThumbLX, sThumbLY, sThumbRX, sThumbRY,
                } = state.Gamepad;
                Ok(State {
                    a: wButtons & xi::XINPUT_GAMEPAD_A != 0,
                    b: wButtons & xi::XINPUT_GAMEPAD_B != 0,
                    x: wButtons & xi::XINPUT_GAMEPAD_X != 0,
                    y: wButtons & xi::XINPUT_GAMEPAD_Y != 0,
                    start: wButtons & xi::XINPUT_GAMEPAD_START != 0,
                    back: wButtons & xi::XINPUT_GAMEPAD_BACK != 0,
                    l_trigger: bLeftTrigger,
                    r_trigger: bRightTrigger,
                    l_shoulder: wButtons & xi::XINPUT_GAMEPAD_LEFT_SHOULDER != 0,
                    r_shoulder: wButtons & xi::XINPUT_GAMEPAD_RIGHT_SHOULDER != 0,
                    dpad: Dpad {
                        x: if wButtons & xi::XINPUT_GAMEPAD_DPAD_LEFT {
                            DpadX::Left
                        } else if wButtons & xi::XINPUT_GAMEPAD_DPAD_RIGHT {
                            DpadX::Right
                        } else { DpadX::None },
                        y: if wButtons & xi::XINPUT_GAMEPAD_DPAD_UP {
                            DpadY::Up
                        } else if wButtons & xi::XINPUT_GAMEPAD_DPAD_DOWN {
                            DpadY::Down
                        } else { DpadY::None },
                    },
                    l_thumb: Thumb { x: sThumbLX, y: sThumbLY },
                    r_thumb: Thumb { x: sThumbRX, y: sThumbRY },
                })
            },
            err::ERROR_DEVICE_NOT_CONNECTED 
                => Err(Error::ControllerNotConnected),
            code @ _ => Err(Error::Code(code)),
        }
    }
    pub fn set_controller_vibration(
        i: XboxControllerIndex, 
        left_and_right_vibration_speed: (u16, u16)
    ) -> Result<(u16, u16)> 
    {
        let mut vibration = XINPUT_VIBRATION {
            wLeftMotorSpeed: left_and_right_vibration_speed.0,
            wRightMotorSpeed: left_and_right_vibration_speed.1,
        };
        match self.api.XInputSetState(i as DWORD, &mut vibration) {
            ERROR_SUCCESS => 
                Ok((vibration.wLeftMotorSpeed, vibration.wRightMotorSpeed)),
            ERROR_DEVICE_NOT_CONNECTED => Err(Error::ControllerNotConnected),
            code @ _ => Err(Error::Code(code)),
        }
    }
}
