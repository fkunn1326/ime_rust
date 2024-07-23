use windows::{
    core::{implement, Error},
    Win32::{
        Foundation::{BOOL, E_NOINTERFACE},
        System::Com::{IClassFactory, IClassFactory_Impl},
        UI::TextServices::ITfTextInputProcessor,
    },
};
use windows_core::{IUnknown, Interface, Result, GUID};

use crate::tsf::text_service::TextService;

#[implement(IClassFactory)]
pub struct IMEClassFactory;

impl IMEClassFactory {
    pub fn new() -> Self {
        IMEClassFactory
    }
}

impl IClassFactory_Impl for IMEClassFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut std::ffi::c_void,
    ) -> Result<()> {
        let riid = unsafe { &*riid };
        let ppvobject = unsafe { &mut *ppvobject };

        *ppvobject = std::ptr::null_mut();

        if punkouter.is_some() {
            return Err(Error::from(E_NOINTERFACE));
        }

        if *riid != ITfTextInputProcessor::IID || *riid != IUnknown::IID {
            return Err(Error::from(E_NOINTERFACE));
        }

        let text_service: ITfTextInputProcessor = TextService::new().into();

        *ppvobject = unsafe { core::mem::transmute(text_service) };

        Ok(())
    }

    fn LockServer(&self, flock: BOOL) -> Result<()> {
        if flock.as_bool() {
            // Lock
        } else {
            // Unlock
        }
        Ok(())
    }
}
