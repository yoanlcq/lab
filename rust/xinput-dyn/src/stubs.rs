
use super::load::types;

use std::mem;
use mem::zeroed as zero;
use winapi::xinput as xi;
use winapi::winerror as err;

// XXX name mangling ?
// XXX What does XInput do when dwUserIndex is invalid ?

pub unsafe extern "system" fn XInputEnable(enable: BOOL) {}

pub unsafe extern "system" fn XInputGetAudioDeviceIds(
    dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, 
    pRenderCount: *mut UINT,
    pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT
    ) -> DWORD
{
    *pRenderCount = 0;
    *pCaptureCount = 0;
    err::ERROR_SUCCESS
}

pub unsafe extern "system" fn XInputGetBatteryInformation(
    dwUserIndex: DWORD, devType: BYTE, 
    pBatteryInformation: *mut XINPUT_BATTERY_INFORMATION
    ) -> DWORD
{
    (*pBatteryInformation).BatteryType = xi::BATTERY_TYPE_DISCONNECTED;
    (*pBatteryInformation).Batterylevel = xi::BATTERY_LEVEL_FULL;
    err::ERROR_SUCCESS
}

pub unsafe extern "system" fn XInputGetCapabilities(
    dwUserIndex: DWORD, dwFlags: DWORD, 
    pCapabilities: *mut XINPUT_CAPABILITIES
    ) -> DWORD 
{
    let caps = XINPUT_CAPABILITIES {
        Type: xi::XINPUT_DEVTYPE_GAMEPAD,
        Subtype: xi::XINPUT_DEVSUBTYPE_UNKNOWN, // XXX report this
        Flags: 0,
        Gamepad: zero(), // XXX doubts about this
        Vibration: zero(),
    };
    *pCapabilities = caps;
    err::ERROR_SUCCESS
}

pub unsafe extern "system" fn XInputGetDSoundAudioDeviceGuids(
    dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, 
    pDSoundCaptureGuid: *mut GUID
    ) -> DWORD 
{
    let GUID_NULL: GUID = zero();
    *pDSoundRenderGuid = GUID_NULL;
    *pDSoundCaptureGuid = GUID_NULL;
    err::ERROR_SUCCESS
}

pub unsafe extern "system" fn XInputGetKeystroke(
    dwUserIndex: DWORD, dwReserved: DWORD, pKeystroke: PXINPUT_KEYSTROKE
    ) -> DWORD
{
    *pKeystroke = zero();
    err::ERROR_EMPTY
}

pub unsafe extern "system" fn XInputGetState(
    dwUserIndex: DWORD, pState: *mut XINPUT_STATE
    ) -> DWORD 
{
    *pState = zero();
    err::ERROR_SUCCESS
}

pub unsafe extern "system" fn XInputSetState(
    dwUserIndex: DWORD, pVibration: *mut XINPUT_VIBRATION
    ) -> DWORD
{
    *pVibration = zero();
    err::ERROR_SUCCESS
}

pub struct StubbedXInputApi {
    pub(crate) dll: HMODULE, // Prevents POD initialization
    pub XInputEnable: types::XInputEnable,
    pub XInputGetAudioDeviceIds: types::XInputGetAudioDeviceIds,
    pub XInputGetBatteryInformation: types::XInputGetBatteryInformation,
    pub XInputGetCapabilities: types::XInputGetCapabilities,
    pub XInputGetDSoundAudioDeviceGuids: types::XInputGetDSoundAudioDeviceGuids,
    pub XInputGetKeystroke: types::XInputGetKeystroke,
    pub XInputGetState: types::XInputGetState,
    pub XInputSetState: types::XInputSetState,
}
impl Drop for StubbedXInputApi {
    fn drop(&mut self) {
        unsafe {
            FreeLibrary(self.dll);
        }
    }
}

use LoadedXInputApi;

impl LoadedXInputApi {
    pub fn into_stubbed(self) -> StubbedXInputApi {
        StubbedXInputApi::from(self)
    }
}

impl From<LoadedXInputApi> for StubbedXInputApi {
    fn from(l: LoadedXInputApi) -> Self {
        unsafe {
            mem::forget(loaded);
        }
        Self {
            dll: l.dll,
            XInputEnable: l.XInputEnable.unwrap_or(XInputEnable),
            XInputGetAudioDeviceIds: l.XInputGetAudioDeviceIds.unwrap_or(XInputGetAudioDeviceIds),
            XInputGetBatteryInformation: l.XInputGetBatteryInformation.unwrap_or(XInputGetBatteryInformation),
            XInputGetCapabilities: l.XInputGetCapabilities.unwrap_or(XInputGetCapabilities),
            XInputGetDSoundAudioDeviceGuids: l.XInputGetDSoundAudioDeviceGuids.unwrap_or(XInputGetDSoundAudioDeviceGuids),
            XInputGetKeystroke: l.XInputGetKeystroke.unwrap_or(XInputGetKeystroke),
            XInputGetState: l.XInputGetState.unwrap_or(XInputGetState),
            XInputSetState: l.XInputSetState.unwrap_or(XInputSetState),
        }
    }
}
