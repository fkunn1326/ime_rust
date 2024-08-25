use core::str;
use std::borrow::BorrowMut;

use windows::Win32::{
    Foundation::{BOOL, HANDLE, LPARAM, WPARAM}, Storage::FileSystem::{ReadFile, WriteFile}, UI::TextServices::{
        ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl
    }
};
use windows::core::{implement, Result};

use super::composition_mgr::CompositionMgr;

// キーボードイベントを処理するクラス
#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    composition_mgr: CompositionMgr,
    handle: HANDLE,
}

impl KeyEventSink {
    pub fn new(composition_mgr: CompositionMgr, handle: HANDLE) -> Self {
        KeyEventSink {
            composition_mgr,
            handle,
        }
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink_Impl {
    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        _wparam: WPARAM,
        _lparam: LPARAM,
    ) -> Result<BOOL> {
        // https://learn.microsoft.com/ja-jp/windows/win32/inputdev/virtual-key-codes
        fn code2char(code: u8) -> String {
            if code >= 0x41 && code <= 0x5A {
                return str::from_utf8(&vec![code]).unwrap().to_string().to_lowercase();
            } else if code == 0xbd {
                return "ー".to_string()
            } else {
                return "".to_string();
            }
        }
        let code: u8 = _wparam.0.try_into().unwrap();

        let message = code2char(code);
        let message_len = message.len();
        if message == "" {
            return Ok(BOOL::from(true));
        }

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

        if self.composition_mgr.composition.borrow().clone().is_none() {
            self.composition_mgr.start_composition(pic.unwrap().clone())?;
        }

        self.composition_mgr.set_text(&response)?;

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
