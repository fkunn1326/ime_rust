use windows::Win32::UI::TextServices::{ITfContext, ITfEditSession, ITfEditSession_Impl, ITfInsertAtSelection, TF_ANCHOR_END, TF_ES_READWRITE, TF_ES_SYNC, TF_IAS_QUERYONLY, TF_ST_CORRECTION};
use windows_core::{implement, Result, Interface};

use crate::utils::winutils::to_wide_16;

#[implement(ITfEditSession)]
pub struct EditSession {
    context: ITfContext,
}

impl EditSession {
    pub fn new(
        context: ITfContext,
    ) -> Self {
        EditSession {
            context,
        }
    }

    pub fn handle(
        client_id: u32,
        context: ITfContext,
    ) -> Result<()> {
        let session: ITfEditSession = EditSession::new(
            context.clone(),
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
            range.SetText(cookie, TF_ST_CORRECTION, &to_wide_16("Test String\n"))?;
            range.Collapse(cookie, TF_ANCHOR_END)?;
        }

        Ok(())
    }
}
