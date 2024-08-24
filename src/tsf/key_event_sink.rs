use core::str;
use std::{fs::File, io::{Read, Write}};
// use std::fs::File;

use windows::Win32::{
    Foundation::{BOOL, LPARAM, WPARAM},
    UI::TextServices::{
        ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl
    },
};
use windows::core::{implement, Result};

use super::composition_mgr::CompositionMgr;

// キーボードイベントを処理するクラス
#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    composition_mgr: CompositionMgr,
    file: File,
}

impl KeyEventSink {
    pub fn new(composition_mgr: CompositionMgr) -> Self {
        let file = File::options()
            .read(true)
            .write(true)
            .open("\\\\.\\pipe\\azookey_service")
            .unwrap();
        KeyEventSink {
            composition_mgr,
            file
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
        let mut file = self.file.try_clone().unwrap();

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
        file.write_all(message.as_bytes())?;

        // サーバーからの応答を読み取り
        let mut buffer = [0; 1024];
        let mut response = String::new();

        if self.composition_mgr.composition.borrow().clone().is_none() {
            self.composition_mgr.start_composition(pic.unwrap().clone())?;
        }

        match file.read(&mut buffer) {
            Ok(0) => println!("Server closed the connection"),
            Ok(n) => {
                response.push_str(&String::from_utf8_lossy(&buffer[..n]));
                self.composition_mgr.set_text(&response, pic.unwrap().clone())?;
                println!("Received response: {}", response);
            },
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                println!("Read operation timed out");
            },
            Err(e) => return Err(e.into()),
        }
        
        // self.composition_mgr.end_composition(pic.unwrap().clone())?;
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
