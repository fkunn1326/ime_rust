use std::cell::RefCell;

use windows::core::{Interface, Result};
use windows::Win32::Foundation::BOOL;
use windows::Win32::UI::TextServices::{
    ITfKeyEventSink, ITfKeystrokeMgr, ITfLangBarItemButton, ITfSource, ITfTextInputProcessor, ITfTextInputProcessor_Impl, ITfThreadMgr, ITfThreadMgrEventSink
};
use windows_core::{implement, AsImpl};

use super::key_event_sink::KeyEventSink;
use super::language_bar::LanguageBar;
use super::thread_mgr_event_sink::ThreadMgrEventSink;

// すべてを取りまとめるメインのクラス
// Activate()とDeactivate()を実装しておけばいい
#[implement(ITfTextInputProcessor)]
pub struct TextService {
    this: RefCell<Option<ITfTextInputProcessor>>,
    client_id: RefCell<u32>,
    // thread manager
    thread_mgr: RefCell<Option<ITfThreadMgr>>,
    thread_mgr_event_sink: RefCell<Option<ITfThreadMgrEventSink>>,
    thread_mgr_event_sink_cookie: RefCell<u32>,

    // language bar
    language_bar: RefCell<Option<ITfLangBarItemButton>>,

    // key event sink
    key_event_sink: RefCell<Option<ITfKeyEventSink>>,
}

impl TextService {
    pub fn new() -> Self {
        TextService {
            this: RefCell::new(None),
            client_id: RefCell::new(0),

            // thread manager
            thread_mgr: RefCell::new(None),
            thread_mgr_event_sink: RefCell::new(None),
            thread_mgr_event_sink_cookie: RefCell::new(0),

            // language bar
            language_bar: RefCell::new(None),

            // key event sink
            key_event_sink: RefCell::new(None),
        }
    }

    pub fn set_this(&self, this: ITfTextInputProcessor) {
        self.this.replace(Some(this));
    }

    // activate()
    fn activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        match ptim {
            Some(ptim) => {
                self.thread_mgr.replace(Some(ptim.clone()));
            }
            None => {}
        }

        self.client_id.replace(tid);

        self.activate_thread_mgr_event_sink()?;
        self.activate_language_bar()?;
        self.activate_key_event_sink()?;
        Ok(())
    }

    // deactivate()
    fn deactivate(&self) -> Result<()> {
        self.deactivate_thread_mgr_event_sink()?;
        self.deactivate_language_bar()?;
        self.deactivate_key_event_sink()?;
        Ok(())
    }

    // ThreadMgrEventSink
    fn activate_thread_mgr_event_sink(&self) -> Result<()> {
        let sink: ITfThreadMgrEventSink = ThreadMgrEventSink::new().into();
        let source: ITfSource = self.thread_mgr.borrow().clone().unwrap().cast()?;

        let cookie = unsafe { source.AdviseSink(&ITfThreadMgrEventSink::IID, &sink) }?;

        self.thread_mgr_event_sink_cookie.replace(cookie);
        self.thread_mgr_event_sink
            .borrow_mut()
            .replace(ThreadMgrEventSink::new().into());

        Ok(())
    }

    fn deactivate_thread_mgr_event_sink(&self) -> Result<()> {
        let source: ITfSource = self.thread_mgr.borrow().clone().unwrap().cast()?;
        let cookie = *self.thread_mgr_event_sink_cookie.borrow();
        unsafe {
            source.UnadviseSink(cookie)?;
        }
        Ok(())
    }

    // language bar ("あ"とか"A"とかのやつ)
    fn activate_language_bar(&self) -> Result<()> {
        let language_bar = LanguageBar::new(self.thread_mgr.borrow().clone().unwrap()).unwrap();
        self.language_bar.replace(Some(language_bar));

        Ok(())
    }

    fn deactivate_language_bar(&self) -> Result<()> {
        let item = self.language_bar.borrow().clone().unwrap();
        let language_bar = unsafe { item.as_impl() };
        language_bar.deactivate(item.clone())?;

        self.language_bar.replace(None);

        Ok(())
    }

    // Key event sink (キーボードイベント関連)
    fn activate_key_event_sink(&self) -> Result<()> {
        let sink: ITfKeyEventSink = KeyEventSink::new(
            self.client_id.borrow().clone(),
        ).into();
        let source: ITfKeystrokeMgr = self.thread_mgr.borrow().clone().unwrap().cast()?;

        unsafe {
            source.AdviseKeyEventSink(self.client_id.borrow().clone(), &sink, BOOL::from(true))?;
        }

        self.key_event_sink.borrow_mut().replace(sink.into());

        Ok(())
    }

    fn deactivate_key_event_sink(&self) -> Result<()> {
        let source: ITfKeystrokeMgr = self.thread_mgr.borrow().clone().unwrap().cast()?;
        unsafe {
            source.UnadviseKeyEventSink(self.client_id.borrow().clone())?;
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
