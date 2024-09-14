use std::borrow::BorrowMut;
use std::sync::mpsc;

use windows::core::{implement, Result};
use windows::Win32::{
    Foundation::{BOOL, HANDLE, LPARAM, WPARAM},
    Storage::FileSystem::{ReadFile, WriteFile},
    UI::TextServices::{ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl},
};

use crate::ui::ui::{CandidateEvent, UiEvent};

use super::composition_mgr::CompositionMgr;

// キーボードイベントを処理するクラス
#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    composition_mgr: CompositionMgr,
    handle: HANDLE,
    ui_proxy: mpsc::Sender<UiEvent>,
}

impl KeyEventSink {
    pub fn new(
        composition_mgr: CompositionMgr,
        handle: HANDLE,
        ui_proxy: mpsc::Sender<UiEvent>,
    ) -> Self {
        KeyEventSink {
            composition_mgr,
            handle,
            ui_proxy,
        }
    }
}

#[derive(serde::Serialize)]
pub struct KeyEvent {
    pub r#type: String,
    pub message: String,
}

impl ITfKeyEventSink_Impl for KeyEventSink_Impl {
    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        // https://learn.microsoft.com/ja-jp/windows/win32/inputdev/virtual-key-codes
        let code: u8 = _wparam.0.try_into().unwrap();

        let message = serde_json::to_string(&KeyEvent {
            r#type: "key".to_string(),
            message: code.to_string(),
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

        // サーバーからの応答を読み取り
        let mut buffer = [0; 1024];
        let buffer_len = buffer.len();
        unsafe {
            ReadFile(
                self.handle,
                Some(&mut buffer),
                Some((buffer_len as u32).borrow_mut()),
                None,
            )
        }?;

        let response = String::from_utf8_lossy(&buffer[..]);
        let response: Vec<&str> = response.split(',').collect();

        let pos = self.composition_mgr.get_pos()?;

        self.ui_proxy.send(UiEvent::Locate(pos)).unwrap();

        self.ui_proxy
            .send(UiEvent::Candidate(CandidateEvent {
                candidates: response.iter().map(|s| s.to_string()).collect(),
            }))
            .unwrap();

        self.composition_mgr.set_text(&response[0])?;

        Ok(BOOL::from(true))
    }

    fn OnKeyUp(&self, _pic: Option<&ITfContext>, _wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        Ok(BOOL::from(true))
    }

    fn OnPreservedKey(
        &self,
        _pic: Option<&ITfContext>,
        _rguid: *const windows::core::GUID,
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
