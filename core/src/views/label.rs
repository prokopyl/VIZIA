use better_any::{Tid, TidAble};

use crate::{Context, Handle, LocalizedStringKey, View};

#[derive(Tid)]
pub struct Label;

impl Label {
    pub fn new<'a>(cx: &'a mut Context<'a>, text: impl LocalizedStringKey<'a>) -> Handle<'a, Self> {
        Self {}.build2(cx, |_| {}).text(text.key())
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }
}
