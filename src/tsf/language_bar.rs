use windows::core::Error;
use windows::Win32::Foundation::E_INVALIDARG;
use windows::Win32::UI::TextServices::{
    ITfLangBarItemSink, GUID_LBI_INPUTMODE, TF_LBI_STYLE_BTN_BUTTON, TF_LBI_STYLE_TEXTCOLORICON,
};
use windows::Win32::{
    Foundation::{BOOL, POINT, RECT},
    System::Ole::CONNECT_E_CANNOTCONNECT,
    UI::{
        TextServices::{
            ITfLangBarItem, ITfLangBarItemButton, ITfLangBarItemButton_Impl, ITfLangBarItemMgr,
            ITfLangBarItem_Impl, ITfMenu, ITfSource, ITfSource_Impl, ITfThreadMgr, TfLBIClick,
            TF_LANGBARITEMINFO,
        },
        WindowsAndMessaging::{LoadImageW, HICON, IMAGE_ICON, LR_DEFAULTCOLOR},
    },
};
use windows_core::{implement, IUnknown, Interface, Result, BSTR, GUID, PCWSTR};

use crate::utils::globals::GUID_TEXT_SERVICE;
use crate::{dll::DllModule, utils::globals::TEXTSERVICE_LANGBARITEMSINK_COOKIE};

// https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/TSF/language-bar.md
// https://github.com/microsoft/Windows-classic-samples/blob/main/Samples/Win7Samples/winui/input/tsf/textservice/textservice-step04/LanguageBar.cpp

#[implement(ITfSource, ITfLangBarItem, ITfLangBarItemButton)]
pub struct LanguageBar {
    thread_mgr: ITfThreadMgr,
}

// これを代入しないと表示されない
static INFO: TF_LANGBARITEMINFO = TF_LANGBARITEMINFO {
    clsidService: GUID_TEXT_SERVICE,
    guidItem: GUID_LBI_INPUTMODE,
    dwStyle: TF_LBI_STYLE_BTN_BUTTON | TF_LBI_STYLE_TEXTCOLORICON,
    ulSort: 0,
    szDescription: [0; 32],
};

impl LanguageBar {
    pub fn new(thread_mgr: ITfThreadMgr) -> Result<ITfLangBarItemButton> {
        let this = LanguageBar {
            thread_mgr: thread_mgr.clone(),
        };
        let item: ITfLangBarItemButton = this.into();
        LanguageBar::add_item(thread_mgr.clone(), item.clone())?;
        Ok(item)
    }

    pub fn deactivate(&self, item: ITfLangBarItemButton) -> Result<()> {
        LanguageBar::remove_item(self, item)
    }

    fn add_item(thread_mgr: ITfThreadMgr, item: ITfLangBarItemButton) -> Result<()> {
        let langbar_mgr: ITfLangBarItemMgr = thread_mgr.cast()?;
        unsafe { langbar_mgr.AddItem(&item)? }

        Ok(())
    }

    fn remove_item(&self, item: ITfLangBarItemButton) -> Result<()> {
        let langbar_mgr: ITfLangBarItemMgr = self.thread_mgr.cast()?;
        unsafe { langbar_mgr.RemoveItem(&item)? }

        Ok(())
    }
}

impl ITfLangBarItem_Impl for LanguageBar_Impl {
    fn GetInfo(&self, p_info: *mut TF_LANGBARITEMINFO) -> Result<()> {
        unsafe {
            *p_info = INFO;
        }
        Ok(())
    }

    fn GetStatus(&self) -> Result<u32> {
        Ok(0)
    }

    fn Show(&self, _f_show: BOOL) -> Result<()> {
        Ok(())
    }

    fn GetTooltipString(&self) -> Result<BSTR> {
        Ok(BSTR::from("GetTooltipString"))
    }
}

impl ITfLangBarItemButton_Impl for LanguageBar_Impl {
    fn OnClick(&self, _click: TfLBIClick, _pt: &POINT, _prcarea: *const RECT) -> Result<()> {
        Ok(())
    }

    fn InitMenu(&self, _pmenu: Option<&ITfMenu>) -> windows_core::Result<()> {
        Ok(())
    }

    fn OnMenuSelect(&self, _w_id: u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn GetIcon(&self) -> Result<HICON> {
        unsafe {
            let handle = LoadImageW(
                DllModule::global().lock().unwrap().hinst,
                PCWSTR(102 as *mut u16),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTCOLOR,
            )?;

            Ok(HICON(handle.0))
        }
    }

    fn GetText(&self) -> Result<BSTR> {
        Ok(BSTR::from("GetText"))
    }
}

impl ITfSource_Impl for LanguageBar_Impl {
    fn AdviseSink(&self, riid: *const GUID, punk: Option<&IUnknown>) -> Result<u32> {
        let riid = unsafe { *riid };

        if riid != ITfLangBarItemSink::IID {
            return Err(Error::from(E_INVALIDARG));
        }

        if punk.is_none() {
            return Err(Error::from(E_INVALIDARG));
        }

        Ok(TEXTSERVICE_LANGBARITEMSINK_COOKIE)
    }

    fn UnadviseSink(&self, dw_cookie: u32) -> Result<()> {
        if dw_cookie != TEXTSERVICE_LANGBARITEMSINK_COOKIE {
            return Err(Error::from(CONNECT_E_CANNOTCONNECT));
        }

        Ok(())
    }
}
