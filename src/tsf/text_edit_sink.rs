use windows::Win32::UI::TextServices::{
    ITfContext, ITfEditRecord, ITfTextEditSink, ITfTextEditSink_Impl,
};
use windows_core::{implement, Result};

#[implement(ITfTextEditSink)]
pub struct TextEditSink;

impl TextEditSink {
    pub fn new() -> Self {
        TextEditSink
    }
}

impl ITfTextEditSink_Impl for TextEditSink_Impl {
    fn OnEndEdit(
        &self,
        _pic: Option<&ITfContext>,
        _ec_read_only: u32,
        _edit_record: Option<&ITfEditRecord>,
    ) -> Result<()> {
        Ok(())
    }
}
