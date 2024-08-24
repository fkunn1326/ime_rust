use windows::core::{Result, implement};
use windows::Win32::UI::TextServices::{
    ITfContext, ITfDocumentMgr, ITfThreadMgrEventSink, ITfThreadMgrEventSink_Impl,
};

// イベントを受け取るクラス、編集コンテキストを作成したり、破棄したりするときに呼ばれる
#[implement(ITfThreadMgrEventSink)]
pub struct ThreadMgrEventSink;

impl ThreadMgrEventSink {
    pub fn new() -> Self {
        ThreadMgrEventSink
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
        _doc_mgr: Option<&ITfDocumentMgr>,
        _prev_doc_mgr: Option<&ITfDocumentMgr>,
    ) -> Result<()> {
        Ok(())
    }

    fn OnPushContext(&self, _context: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }

    fn OnPopContext(&self, _context: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
}
