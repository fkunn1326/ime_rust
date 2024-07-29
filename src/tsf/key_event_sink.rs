use windows::Win32::{
    Foundation::{BOOL, LPARAM, WPARAM},
    UI::TextServices::{ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl},
};
use windows_core::{implement, Result};

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink;

impl KeyEventSink {
    pub fn new() -> Self {
        KeyEventSink
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink_Impl {
    fn OnKeyDown(
        &self,
        _pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnKeyUp(
        &self,
        _pic: Option<&windows::Win32::UI::TextServices::ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnPreservedKey(
        &self,
        _pic: Option<&windows::Win32::UI::TextServices::ITfContext>,
        _rguid: *const windows_core::GUID,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnSetFocus(&self, _fforeground: BOOL) -> Result<()> {
        Ok(())
    }

    fn OnTestKeyDown(
        &self,
        _pic: Option<&windows::Win32::UI::TextServices::ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnTestKeyUp(
        &self,
        _pic: Option<&windows::Win32::UI::TextServices::ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }
}
