use windows::core::{implement, Result};
use windows::Win32::UI::TextServices::{
    ITfContext, ITfEditSession, ITfEditSession_Impl, TF_ES_READWRITE, TF_ES_SYNC,
};

use std::rc::Rc;

// テキスト編集に必要なクッキーを受け取り、編集処理を行うクラス
#[implement(ITfEditSession)]
pub struct EditSession {
    callback: Rc<dyn Fn(u32) -> Result<()>>,
}

impl EditSession {
    pub fn new(callback: Rc<dyn Fn(u32) -> Result<()>>) -> Self {
        EditSession { callback }
    }

    pub fn handle(
        client_id: u32,
        context: ITfContext,
        callback: Rc<dyn Fn(u32) -> Result<()>>,
    ) -> Result<()> {
        let session: ITfEditSession = EditSession::new(callback).into();

        let result = unsafe {
            context.RequestEditSession(client_id, &session, TF_ES_SYNC | TF_ES_READWRITE)
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl ITfEditSession_Impl for EditSession_Impl {
    fn DoEditSession(&self, cookie: u32) -> Result<()> {
        (self.callback)(cookie)?;
        Ok(())
    }
}
