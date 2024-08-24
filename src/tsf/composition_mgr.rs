use std::{cell::RefCell, rc::Rc};

use windows::Win32::UI::TextServices::{
    ITfComposition, ITfCompositionSink, ITfContext, ITfContextComposition, ITfInsertAtSelection, TF_IAS_QUERYONLY
};
use windows::core::{Interface, Result};

use crate::utils::winutils::to_wide_16;

use super::edit_session::EditSession;

#[derive(Clone)]
pub struct CompositionMgr {
    pub composition: Rc<RefCell<Option<ITfComposition>>>,
    sink: ITfCompositionSink,
    client_id: u32,
    pub preedit: RefCell<String>,
}

impl CompositionMgr {
    pub fn new(client_id: u32, sink: ITfCompositionSink) -> Self {
        CompositionMgr {
            composition: Rc::new(RefCell::new(None)),
            sink,
            client_id,
            preedit: RefCell::new(String::new()),
        }
    }

    pub fn start_composition(&self, context: ITfContext) -> Result<()> {
        let insert: ITfInsertAtSelection = context.cast()?;
        let context_composition: ITfContextComposition = context.cast()?;

        EditSession::handle(
            self.client_id,
            context,
            Rc::new(
                {
                    let composition_clone = Rc::clone(&self.composition);
                    let sink = self.sink.clone();
                    move |cookie| unsafe {
                        let range = insert.InsertTextAtSelection(cookie, TF_IAS_QUERYONLY, &[])?;
                        let new_composition = context_composition.StartComposition(cookie, &range, &sink)?;
                        *composition_clone.borrow_mut() = Some(new_composition);
                        Ok(())
                    }
                }
            ),
        )?;

        Ok(())
    }

    pub fn end_composition(&self, context: ITfContext) -> Result<()> {
        let composition = self.composition.borrow().clone().unwrap();
        EditSession::handle(
            self.client_id,
            context,
            Rc::new(move |cookie| unsafe {
                composition.EndComposition(cookie)?;
                Ok(())
            }),
        )?;

        Ok(())
    }

    pub fn set_text(&self, text: &str, context: ITfContext) -> Result<()> {
        self.preedit.replace(text.to_string());
        let composition = self.composition.borrow().clone().unwrap();
        let wide_text = to_wide_16(text);
        EditSession::handle(
            self.client_id,
            context,
            Rc::new(move |cookie| unsafe {
                let range = composition.GetRange()?;
                range.SetText(cookie, 0, &wide_text)?;
                Ok(())
            }),
        )?;

        Ok(())
    }
}
