use std::borrow::BorrowMut;

use windows::core::{implement, Result};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::WriteFile;
use windows::Win32::UI::TextServices::{
    ITfContext, ITfDocumentMgr, ITfThreadMgrEventSink, ITfThreadMgrEventSink_Impl,
};

use crate::utils::winutils::debug;

use super::composition_mgr::CompositionMgr;
use super::key_event_sink::KeyEvent;


// イベントを受け取るクラス、編集コンテキストを作成したり、破棄したりするときに呼ばれる
#[implement(ITfThreadMgrEventSink)]
pub struct ThreadMgrEventSink {
    composition_mgr: CompositionMgr,
    handle: HANDLE,
}

impl ThreadMgrEventSink {
    pub fn new(composition_mgr: CompositionMgr, handle: HANDLE) -> Self {
        ThreadMgrEventSink {
            composition_mgr,
            handle,
        }
    }
}

impl ITfThreadMgrEventSink_Impl for ThreadMgrEventSink_Impl {
    fn OnInitDocumentMgr(&self, _doc_mgr: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }

    fn OnUninitDocumentMgr(&self, _doc_mgr: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }

    fn OnSetFocus(
        &self,
        docmgr: Option<&ITfDocumentMgr>,
        _prev_doc_mgr: Option<&ITfDocumentMgr>,
    ) -> Result<()> {
        if docmgr.is_none() {
            return Ok(());
        }
        let context = unsafe { docmgr.unwrap().GetBase() }?;
        self.composition_mgr.start_composition(context.clone())?;
        let preceding_text = self.composition_mgr.get_preceding_text()?;

        let message = serde_json::to_string(&KeyEvent {
            r#type: "left".to_string(),
            message: preceding_text
        })
        .unwrap();
        let message_len = message.len();

        let wide: Vec<u8> = message.to_string().into_bytes();
        unsafe {
            WriteFile(
                self.handle,
                Some(&wide),
                Some((message_len as u32).borrow_mut()),
                None,
            )
        }?;
        
        Ok(())
    }

    fn OnPushContext(&self, _context: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }

    fn OnPopContext(&self, _context: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
}
