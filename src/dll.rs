use std::ffi::c_void;

use windows::core::GUID;
use windows::Win32::Foundation::{BOOL, CLASS_E_CLASSNOTAVAILABLE, E_UNEXPECTED, HMODULE, S_FALSE};
use windows::Win32::System::Com::IClassFactory;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::{core::HRESULT, Win32::Foundation::S_OK};
use windows_core::Interface;

use crate::check_err;
use crate::factory::IMEClassFactory;
use crate::register::*;
use crate::utils::globals::GUID_TEXT_SERVICE;
use crate::utils::winutils::get_module_path;

use std::sync::{Mutex, OnceLock};

static DLL_INSTANCE: OnceLock<Mutex<DllModule>> = OnceLock::new();

unsafe impl Sync for DllModule {}
unsafe impl Send for DllModule {}

pub struct DllModule {
    ref_count: u32,
    ref_lock: u32,
    pub hinst: HMODULE,
}

impl DllModule {
    pub fn global() -> &'static Mutex<DllModule> {
        DLL_INSTANCE.get().expect("DllModule is not initialized")
    }

    pub fn new() -> Self {
        Self {
            ref_count: 0,
            ref_lock: 0,
            hinst: HMODULE::default(),
        }
    }

    pub fn add_ref(&mut self) -> u32 {
        self.ref_count += 1;
        self.ref_count
    }

    pub fn release(&mut self) -> u32 {
        self.ref_count -= 1;
        self.ref_count
    }

    pub fn lock(&mut self) -> u32 {
        self.ref_lock += 1;
        self.ref_lock
    }

    pub fn unlock(&mut self) -> u32 {
        self.ref_lock -= 1;
        self.ref_lock
    }

    pub fn can_unload(&self) -> bool {
        self.ref_count == 0 && self.ref_lock == 0
    }
}

#[no_mangle]
pub extern "system" fn DllMain(
    hinst: HMODULE,
    fdw_reason: u32,
    _lpv_reserved: *mut c_void,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            let mut dll_instance = DllModule::new();
            dll_instance.hinst = hinst;
            let _ = DLL_INSTANCE.set(Mutex::new(dll_instance));
        }
        _ => {}
    }

    BOOL::from(true)
}

#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    let dll_instance = DllModule::global().lock().unwrap();
    if dll_instance.can_unload() {
        return S_OK;
    }
    return S_FALSE;
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
