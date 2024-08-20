use pumpkin_macros::packet;
use pumpkin_text::TextComponent;
use serde::Serialize;

#[derive(Serialize)]
#[packet(0x4C)]
pub struct CActionBar<'a> {
    action_bar: TextComponent<'a>,
}

impl<'a> CActionBar<'a> {
    pub fn new(action_bar: TextComponent<'a>) -> Self {
        Self { action_bar }
    }
}
