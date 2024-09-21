use std::sync::mpsc::Sender;

use windows::core::{implement, Result};
use windows::Win32::{
    Foundation::{BOOL, LPARAM, WPARAM},
    UI::TextServices::{ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl},
};

use ipc::socket::SocketManager;

use crate::ui::{CandidateEvent, UiEvent};

use super::composition_mgr::CompositionMgr;

// キーボードイベントを処理するクラス
#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    composition_mgr: CompositionMgr,
    socket_mgr: SocketManager,
    ui_proxy: Sender<UiEvent>,
}

impl KeyEventSink {
    pub fn new(
        composition_mgr: CompositionMgr,
        socket_mgr: SocketManager,
        ui_proxy: Sender<UiEvent>,
    ) -> Self {
        KeyEventSink {
            composition_mgr,
            socket_mgr,
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

        let response = self.socket_mgr.get(message).unwrap();
        let response: Vec<&str> = response.split(',').collect();

        // let pos = self.composition_mgr.get_pos()?;

        if self.composition_mgr.composition.borrow().clone().is_none() {
            self.composition_mgr
                .start_composition(pic.unwrap().clone())?;
        }

        self.composition_mgr
            .set_text(&format!("{}", &response[0]))?;

        let pos = self.composition_mgr.get_pos()?;

        self.ui_proxy.send(UiEvent::Locate(pos)).unwrap();

        self.ui_proxy
            .send(UiEvent::Candidate(CandidateEvent {
                candidates: response.iter().map(|s| s.to_string()).collect(),
            }))
            .unwrap();

        self.ui_proxy.send(UiEvent::Show).unwrap();

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
