use std::cell::RefCell;
use std::collections::HashMap;

use windows::core::{Interface, Result, implement, AsImpl, w};
use windows::Win32::Foundation::{CloseHandle, BOOL, GENERIC_READ, GENERIC_WRITE, HANDLE};
use windows::Win32::Storage::FileSystem::{CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING};
use windows::Win32::UI::TextServices::{
    CLSID_TF_CategoryMgr, ITfCategoryMgr, ITfCompositionSink, ITfCompositionSink_Impl, ITfKeyEventSink, ITfKeystrokeMgr, ITfLangBarItemButton, ITfSource, ITfTextInputProcessor, ITfTextInputProcessor_Impl, ITfThreadMgr, ITfThreadMgrEventSink
};

use crate::utils::globals::{
    GUID_DISPLAY_ATTRIBUTE_CONVERTED, GUID_DISPLAY_ATTRIBUTE_FOCUSED, GUID_DISPLAY_ATTRIBUTE_INPUT,
};
use crate::utils::winutils::co_create_inproc;

use super::composition_mgr::CompositionMgr;
use super::key_event_sink::KeyEventSink;
use super::language_bar::LanguageBar;
use super::thread_mgr_event_sink::ThreadMgrEventSink;

// すべてを取りまとめるメインのクラス
// Activate()とDeactivate()を実装しておけばいい
#[implement(
    ITfTextInputProcessor,
    ITfCompositionSink
)]
pub struct TextService {
    this: RefCell<Option<ITfTextInputProcessor>>,
    client_id: RefCell<u32>,
    // thread manager
    thread_mgr: RefCell<Option<ITfThreadMgr>>,
    thread_mgr_event_sink: RefCell<Option<ITfThreadMgrEventSink>>,
    thread_mgr_event_sink_cookie: RefCell<u32>,

    // category manager
    category_mgr: RefCell<Option<ITfCategoryMgr>>,

    // language bar
    language_bar: RefCell<Option<ITfLangBarItemButton>>,

    // key event sink
    key_event_sink: RefCell<Option<ITfKeyEventSink>>,

    // display attribute
    display_attribute_atom: RefCell<HashMap<&'static str, u32>>,

    // composition manager
    composition_mgr: RefCell<Option<CompositionMgr>>,

    // pipe handle
    pipe_handle: RefCell<Option<HANDLE>>,
}

impl TextService {
    pub fn new() -> Self {
        TextService {
            this: RefCell::new(None),
            client_id: RefCell::new(0),

            thread_mgr: RefCell::new(None),
            thread_mgr_event_sink: RefCell::new(None),
            thread_mgr_event_sink_cookie: RefCell::new(0),

            category_mgr: RefCell::new(None),

            language_bar: RefCell::new(None),

            key_event_sink: RefCell::new(None),

            display_attribute_atom: RefCell::new(HashMap::new()),

            composition_mgr: RefCell::new(None),

            pipe_handle: RefCell::new(None),
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

        self.category_mgr
            .replace(Some(co_create_inproc::<ITfCategoryMgr>(
                &CLSID_TF_CategoryMgr,
            )?));
        self.client_id.replace(tid);

        self.activate_thread_mgr_event_sink()?;
        self.activate_language_bar()?;
        self.activate_display_attribute()?;
        self.activate_pipe()?;
        self.activate_composition_mgr()?;
        self.activate_key_event_sink()?;
        Ok(())
    }

    // deactivate()
    fn deactivate(&self) -> Result<()> {
        self.deactivate_thread_mgr_event_sink()?;
        self.deactivate_language_bar()?;
        self.deactivate_display_attribute()?;
        self.deactivate_composition_mgr()?;
        self.deactivate_key_event_sink()?;
        self.deactivate_pipe()?;
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

    // Display attribute (表示属性、下線入れたり色変えたり)
    fn activate_display_attribute(&self) -> Result<()> {
        let category_mgr = self.category_mgr.borrow().clone().unwrap();
        let mut atom_map = HashMap::new();

        unsafe {
            let input_atom = category_mgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_INPUT)?;
            let focused_atom = category_mgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_FOCUSED)?;
            let converted_atom = category_mgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_CONVERTED)?;

            atom_map.insert("input", input_atom);
            atom_map.insert("focused", focused_atom);
            atom_map.insert("converted", converted_atom);
        }

        // alert(&format!("input: {:?}", atom_map));

        self.display_attribute_atom.replace(atom_map);

        Ok(())
    }

    fn deactivate_display_attribute(&self) -> Result<()> {
        self.display_attribute_atom.borrow_mut().clear();
        Ok(())
    }

    fn activate_composition_mgr(&self) -> Result<()> {
        let client_id = self.client_id.borrow().clone();

        let this: ITfTextInputProcessor = self.this.borrow().clone().unwrap();
        let sink: ITfCompositionSink = this.cast()?;

        let composition_mgr = CompositionMgr::new(client_id, sink);
        self.composition_mgr.replace(Some(composition_mgr));

        Ok(())
    }

    fn deactivate_composition_mgr(&self) -> Result<()> {
        let composition_mgr = self.composition_mgr.borrow_mut().take().unwrap();
        composition_mgr.end_composition()?;
        Ok(())
    }

    // Key event sink (キーボードイベント関連)
    fn activate_key_event_sink(&self) -> Result<()> {
        let sink: ITfKeyEventSink = KeyEventSink::new(
            self.composition_mgr.borrow().clone().unwrap(),
            self.pipe_handle.borrow().clone().unwrap(),
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

    fn activate_pipe(&self) -> Result<()> {
        let handle = unsafe {
            CreateFileW(
                w!("\\\\.\\pipe\\azookey_service"),
                GENERIC_WRITE.0 | GENERIC_READ.0,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        }?;

        self.pipe_handle.replace(Some(handle));
        Ok(())
    }

    fn deactivate_pipe(&self) -> Result<()> {
        let handle = self.pipe_handle.borrow().clone().unwrap();
        unsafe {
            CloseHandle(handle)?;
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

impl ITfCompositionSink_Impl for TextService_Impl {
    fn OnCompositionTerminated(&self,_ecwrite:u32,_pcomposition:Option<&windows::Win32::UI::TextServices::ITfComposition>) -> windows_core::Result<()> {
        Ok(())
    }
}