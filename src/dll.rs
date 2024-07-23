use std::ffi::c_void;

use windows::core::GUID;
use windows::Win32::Foundation::{CLASS_E_CLASSNOTAVAILABLE, E_UNEXPECTED};
use windows::Win32::System::Com::IClassFactory;
use windows::{core::HRESULT, Win32::Foundation::S_OK};
use windows_core::Interface;

use crate::check_err;
use crate::factory::IMEClassFactory;
use crate::register::*;
use crate::utils::globals::GUID_TEXT_SERVICE;
use crate::utils::winutils::get_module_path;

#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    return S_OK;
}

#[no_mangle]
pub extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let rclsid = &unsafe { *rclsid };
    let riid = &unsafe { *riid };
    let ppv = unsafe { &mut *ppv };

    *ppv = std::ptr::null_mut();

    if *rclsid != GUID_TEXT_SERVICE {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    if *riid != IClassFactory::IID {
        return E_UNEXPECTED;
    }

    let factory: IMEClassFactory = IMEClassFactory::new().into();
    let factory: IClassFactory = factory.into();

    *ppv = unsafe { std::mem::transmute(factory) };

    return S_OK;
}

#[no_mangle]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    let result = ProfileMgr::register(get_module_path().as_str());
    check_err!(result);

    let result = ClsidMgr::register(get_module_path().as_str());
    check_err!(result);

    let result = CategiryMgr::register();
    check_err!(result);

    return S_OK;
}

#[no_mangle]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    let result = ProfileMgr::unregister();
    check_err!(result);

    let result = ClsidMgr::unregister();
    check_err!(result);

    let result = CategiryMgr::unregister();
    check_err!(result);

    return S_OK;
}
