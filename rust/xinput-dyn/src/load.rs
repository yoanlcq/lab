#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::transmute;
use winapi::*;
use kernel32::{
    LoadLibraryA,
    GetProcAddress,
    FreeLibrary
};

/// List of DLL names, sorted from best to worst.
pub static XINPUT_DLL_NAMES: &[&'static [u8]] = &[
    b"xinput1_4.dll\0",
    b"xinput1_3.dll\0",
    b"xinput9_1_0.dll\0",
    b"xinput1_2.dll\0",
    b"xinput1_1.dll\0",
];

pub mod types {

    use super::*;

    pub type XInputEnable = unsafe extern "system" fn(enable: BOOL);

    pub type XInputGetAudioDeviceIds = 
        unsafe extern "system" fn(
            dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, 
            pRenderCount: *mut UINT,
            pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT
        ) -> DWORD;

    pub type XInputGetBatteryInformation = 
        unsafe extern "system" fn(
            dwUserIndex: DWORD, devType: BYTE, 
            pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION
        ) -> DWORD;

    pub type XInputGetCapabilities = 
        unsafe extern "system" fn(
            dwUserIndex: DWORD, dwFlags: DWORD, 
            pCapabilities: *mut XINPUT_CAPABILITIES
        ) -> DWORD;

    pub type XInputGetDSoundAudioDeviceGuids = 
        unsafe extern "system" fn(
            dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, 
            pDSoundCaptureGuid: *mut GUID
        ) -> DWORD;

    pub type XInputGetKeystroke = 
        unsafe extern "system" fn(
            dwUserIndex: DWORD, dwReserved: DWORD, pKeystroke: fix::PXINPUT_KEYSTROKE
        ) -> DWORD;

    pub type XInputGetState = 
        unsafe extern "system" fn(
            dwUserIndex: DWORD, pState: *mut XINPUT_STATE
        ) -> DWORD;

    pub type XInputSetState = 
        unsafe extern "system" fn(
            dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION
        ) -> DWORD;
}

pub mod fix {
    use super::*;
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
    type PXINPUT_KEYSTROKE = *mut XINPUT_KEYSTROKE;
}

// XXX instead of Option, shouldn't we also provide error codes for funcs
// which failed to load ?

pub struct LoadedXInputApi {
    pub(crate) dll: HMODULE, // Prevents POD initialization
    pub XInputEnable: Option<types::XInputEnable>,
    pub XInputGetAudioDeviceIds: Option<types::XInputGetAudioDeviceIds>,
    pub XInputGetBatteryInformation: Option<types::XInputGetBatteryInformation>,
    pub XInputGetCapabilities: Option<types::XInputGetCapabilities>,
    pub XInputGetDSoundAudioDeviceGuids: Option<types::XInputGetDSoundAudioDeviceGuids>,
    pub XInputGetKeystroke: Option<types::XInputGetKeystroke>,
    pub XInputGetState: Option<types::XInputGetState>,
    pub XInputSetState: Option<types::XInputSetState>,
}

macro_rules! help_me {
    ($dll:ident, $($name:ident $bstr:tt),+) => {
        {
            $(
                let $name = GetProcAddress($dll, $bstr.as_ptr() as LPCSTR);
                let $name: Option<types::$name> = if $name.is_null() {
                    None
                } else {
                    Some(transmute($name))
                };
            )+
            Self {
                dll: $dll,
                $(
                    $name,
                )+
            }
        }
    }
}

impl Drop for LoadedXInputApi {
    fn drop(&mut self) {
        unsafe {
            FreeLibrary(self.dll);
        }
    }
}

impl LoadedXInputApi {
    pub fn load_best() -> Result<(Self, usize),()> {
        for (i, name) in XINPUT_DLL_NAMES.iter().enumerate() {
            if let Ok(loaded) = Self::load_dll(name) {
                return Ok((loaded, i));
            }
        }
        Err(())
    }

    pub fn load_dll(path: &[u8]) -> Result<Self, ()> {
        unsafe {
            let dll = LoadLibraryA(path.as_ptr() as LPCSTR);
            if dll.is_null() {
                return Err(()); // TODO: Error code ?
            }
            Ok(help_me!{
                dll,
                XInputEnable b"XInputEnable\0",
                XInputGetAudioDeviceIds b"XInputGetAudioDeviceIds\0",
                XInputGetBatteryInformation b"XInputGetBatteryInformation\0",
                XInputGetCapabilities b"XInputGetCapabilities\0",
                XInputGetDSoundAudioDeviceGuids b"XInputGetDSoundAudioDeviceGuids\0",
                XInputGetKeystroke b"XInputGetKeystroke\0",
                XInputGetState b"XInputGetState\0",
                XInputSetState b"XInputSetState\0"
            })
        }
    }
}
