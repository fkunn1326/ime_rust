use windows::core::Result;
use windows::Win32::UI::TextServices::{
    ITfTextInputProcessor, ITfTextInputProcessor_Impl, ITfThreadMgr,
};
use windows_core::implement;

#[implement(ITfTextInputProcessor)]
pub struct TextService;

impl TextService {
    pub fn new() -> Self {
        Self
    }
}

impl ITfTextInputProcessor_Impl for TextService_Impl {
    fn Activate(&self, _ptim: Option<&ITfThreadMgr>, _tid: u32) -> Result<()> {
        Ok(())
    }

    fn Deactivate(&self) -> Result<()> {
        Ok(())
    }
}
