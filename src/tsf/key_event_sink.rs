use windows::Win32::{
    Foundation::{BOOL, LPARAM, WPARAM},
    UI::TextServices::{
        ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl
    },
};
use windows::core::{implement, Result};

use super::composition_mgr::CompositionMgr;

// キーボードイベントを処理するクラス
#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    composition_mgr: CompositionMgr,
}

impl KeyEventSink {
    pub fn new(composition_mgr: CompositionMgr) -> Self {
        KeyEventSink {
            composition_mgr,
        }
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink_Impl {
    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        if self.composition_mgr.composition.borrow().clone().is_none() {
            self.composition_mgr.start_composition(pic.unwrap().clone())?;
            self.composition_mgr.set_text("0", pic.unwrap().clone())?;
        } else {
            let preedit = self.composition_mgr.preedit.borrow().clone();
            let preedit_int = preedit.parse::<i32>().unwrap();
            self.composition_mgr.set_text(&(preedit_int+1).to_string(), pic.unwrap().clone())?;
        }
        
        // self.composition_mgr.end_composition(pic.unwrap().clone())?;
        Ok(BOOL::from(true))
    }

    fn OnKeyUp(&self, _pic: Option<&ITfContext>, _wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnPreservedKey(
        &self,
        _pic: Option<&ITfContext>,
        _rguid: *const windows::core::GUID,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnSetFocus(&self, _fforeground: BOOL) -> Result<()> {
        Ok(())
    }

    fn OnTestKeyDown(
        &self,
        _pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnTestKeyUp(
        &self,
        _pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }
}
