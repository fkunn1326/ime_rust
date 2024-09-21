use windows::core::{implement, Result};
use windows::Win32::UI::TextServices::{
    ITfContext, ITfDocumentMgr, ITfThreadMgrEventSink, ITfThreadMgrEventSink_Impl,
};

use ipc::socket::SocketManager;

use super::composition_mgr::CompositionMgr;
use super::key_event_sink::KeyEvent;

// イベントを受け取るクラス、編集コンテキストを作成したり、破棄したりするときに呼ばれる
#[implement(ITfThreadMgrEventSink)]
pub struct ThreadMgrEventSink {
    composition_mgr: CompositionMgr,
    socket_mgr: SocketManager,
}

impl ThreadMgrEventSink {
    pub fn new(composition_mgr: CompositionMgr, socket_mgr: SocketManager) -> Self {
        ThreadMgrEventSink {
            composition_mgr,
            socket_mgr,
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
        if self.composition_mgr.composition.borrow().is_none() {
            self.composition_mgr.start_composition(context)?;
        }
        let preceding_text = self.composition_mgr.get_preceding_text()?;

        let message = serde_json::to_string(&KeyEvent {
            r#type: "left".to_string(),
            message: preceding_text,
        })
        .unwrap();
        self.socket_mgr.post(message)?;

        Ok(())
    }

    fn OnPushContext(&self, _context: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }

    fn OnPopContext(&self, _context: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
}
