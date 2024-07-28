use std::cell::RefCell;

use windows::core::{IUnknown, Interface, Result};
use windows::Win32::UI::TextServices::{
    ITfSource, ITfTextInputProcessor, ITfTextInputProcessor_Impl, ITfThreadMgr,
    ITfThreadMgrEventSink,
};
use windows_core::implement;

use super::thread_mgr_event_sink::ThreadMgrEventSink;

// すべてを取りまとめるメインのクラス
// Activate()とDeactivate()を実装しておけばいい
#[implement(ITfTextInputProcessor)]
pub struct TextService {
    this: RefCell<Option<ITfTextInputProcessor>>,
    thread_mgr: RefCell<Option<ITfThreadMgr>>,
    thread_mgr_event_sink: RefCell<Option<ITfThreadMgrEventSink>>,
    thread_mgr_event_sink_cookie: RefCell<u32>,
}

impl TextService {
    pub fn new() -> Self {
        TextService {
            this: RefCell::new(None),
            thread_mgr: RefCell::new(None),
            thread_mgr_event_sink: RefCell::new(None),
            thread_mgr_event_sink_cookie: RefCell::new(0),
        }
    }

    // activate()
    fn activate(&self, ptim: Option<&ITfThreadMgr>, _tid: u32) -> Result<()> {
        match ptim {
            Some(ptim) => {
                self.thread_mgr.replace(Some(ptim.clone()));
            }
            None => {}
        }
        self.init_thread_mgr_event_sink()?;
        Ok(())
    }

    // deactivate()
    fn deactivate(&self) -> Result<()> {
        self.uninit_thread_mgr_event_sink()?;
        Ok(())
    }

    pub fn set_this(&self, this: ITfTextInputProcessor) {
        self.this.replace(Some(this));
    }

    // ThreadMgrEventSink
    fn init_thread_mgr_event_sink(&self) -> Result<()> {
        self.thread_mgr_event_sink
            .borrow_mut()
            .replace(ThreadMgrEventSink::new().into());

        let source: ITfSource = self.thread_mgr.borrow().clone().unwrap().cast()?;
        let sink: IUnknown = self
            .thread_mgr_event_sink
            .borrow()
            .clone()
            .unwrap()
            .cast()?;

        let cookie = unsafe { source.AdviseSink(&ITfThreadMgrEventSink::IID, &sink) }?;

        *self.thread_mgr_event_sink_cookie.borrow_mut() = cookie;

        Ok(())
    }

    fn uninit_thread_mgr_event_sink(&self) -> Result<()> {
        let source: ITfSource = self.thread_mgr.borrow().clone().unwrap().cast()?;
        let cookie = *self.thread_mgr_event_sink_cookie.borrow();
        unsafe {
            source.UnadviseSink(cookie)?;
        }
        Ok(())
    }
}

impl ITfTextInputProcessor_Impl for TextService_Impl {
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        self.activate(ptim, tid)?;
        Ok(())
    }

    fn Deactivate(&self) -> Result<()> {
        self.deactivate()?;
        Ok(())
    }
}
