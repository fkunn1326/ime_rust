use core::str;

use windows::Win32::UI::TextServices::{ITfContext, ITfEditSession, ITfEditSession_Impl, ITfInsertAtSelection, TF_ANCHOR_END, TF_ES_READWRITE, TF_ES_SYNC, TF_IAS_QUERYONLY, TF_ST_CORRECTION};
use windows_core::{implement, Result, Interface};

use crate::utils::winutils::{alert, to_wide_16};

#[implement(ITfEditSession)]
pub struct EditSession {
    context: ITfContext,
    code: u8,
}

impl EditSession {
    pub fn new(
        context: ITfContext,
        code: u8,
    ) -> Self {
        EditSession {
            context,
            code
        }
    }

    pub fn handle(
        client_id: u32,
        context: ITfContext,
        code: u8,
    ) -> Result<()> {
        let session: ITfEditSession = EditSession::new(
            context.clone(),
            code
        ).into();
        
        let result = unsafe { context.RequestEditSession(
            client_id,
            &session,
            TF_ES_SYNC | TF_ES_READWRITE
        ) };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl ITfEditSession_Impl for EditSession_Impl {
    fn DoEditSession(&self, cookie: u32) -> Result<()> {
        let insert: ITfInsertAtSelection = self.context.cast()?;
        unsafe {
            let range = insert.InsertTextAtSelection(cookie, TF_IAS_QUERYONLY, &[])?;

            if self.code >= 0x41 && self.code <= 0x5A {
                let chars = vec![0xD835, 0xDC00 - 0x41 + self.code as u16];
                let string = String::from_utf16(&chars).unwrap();
    
                range.SetText(cookie, TF_ST_CORRECTION, &to_wide_16(&string))?;
                range.Collapse(cookie, TF_ANCHOR_END)?;
            } else {
                range.SetText(cookie, TF_ST_CORRECTION, &to_wide_16(&char::from(self.code).to_string()))?;
                range.Collapse(cookie, TF_ANCHOR_END)?;
            }

        }

        Ok(())
    }
}
