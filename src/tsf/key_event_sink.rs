use std::rc::Rc;

use windows::Win32::{
    Foundation::{BOOL, LPARAM, WPARAM},
    UI::TextServices::{
        ITfContext, ITfInsertAtSelection, ITfKeyEventSink, ITfKeyEventSink_Impl, TF_ANCHOR_END,
        TF_IAS_QUERYONLY, TF_ST_CORRECTION,
    },
};
use windows_core::{implement, Interface, Result};

use crate::utils::winutils::to_wide_16;

use super::edit_session::EditSession;

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    client_id: u32,
}

impl KeyEventSink {
    pub fn new(client_id: u32) -> Self {
        KeyEventSink { client_id }
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink_Impl {
    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        let insert: ITfInsertAtSelection = pic.unwrap().clone().cast()?;

        EditSession::handle(
            self.client_id,
            pic.unwrap().clone(),
            Rc::new(move |cookie| unsafe {
                let range = insert.InsertTextAtSelection(cookie, TF_IAS_QUERYONLY, &[])?;
                range.SetText(cookie, TF_ST_CORRECTION, &to_wide_16("ABC"))?;
                range.Collapse(cookie, TF_ANCHOR_END)?;
                Ok(())
            }),
        )?;
        Ok(BOOL::from(true))
    }

    fn OnKeyUp(&self, _pic: Option<&ITfContext>, _wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnPreservedKey(
        &self,
        _pic: Option<&ITfContext>,
        _rguid: *const windows_core::GUID,
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
