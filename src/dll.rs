
use std::ffi::c_void;

use windows::{core::HRESULT, Win32::Foundation::S_OK};
use windows::core::GUID;

use crate::register::*;
use crate::check_err;
use crate::utils::winutils::get_module_path;

#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    return S_OK
}

#[no_mangle]
pub extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let _ = &unsafe { *rclsid };
    let _ = &unsafe { *riid };
    let _ = unsafe { &mut *ppv };

    return S_OK
}

#[no_mangle]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    let result = ProfileMgr::register(
        get_module_path().as_str()
    );
    check_err!(result);

    let result = ClsidMgr::register(
        get_module_path().as_str()
    );
    check_err!(result);

    let result = CategiryMgr::register();
    check_err!(result);

    return S_OK
}

#[no_mangle]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    let result = ProfileMgr::unregister();
    check_err!(result);

    let result = ClsidMgr::unregister();
    check_err!(result);

    let result = CategiryMgr::unregister();
    check_err!(result);

    return S_OK
}