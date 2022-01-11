use parley::{Layout, context::{RangedBuilder, TextSource}, style::StyleProperty};

use super::storage::{TextStorage, Storage};



use std::{rc::Rc, ops::RangeBounds};

use crate::Color;

use parley::*;

pub struct TextLayout {
    pub text: Storage,
    pub layout: Layout<Color>,
}

pub struct TextLayoutBuilder {
    text: Storage,
    builder: RangedBuilder<'static, Color, Storage>,
    alignment: layout::Alignment,
    max_width: f64,
}

impl TextLayoutBuilder {
    fn range_attribute(
        mut self,
        range: impl RangeBounds<usize>,
        attribute: StyleProperty<Color>,
    ) -> Self {
        self.builder.push(&attribute, range);
        self
    }

    fn build(mut self) -> TextLayout {
        let mut layout = self.builder.build();
        layout.break_all_lines(Some(self.max_width as f32), self.alignment);
        TextLayout {
            text: self.text,
            layout,
        }
    }
}

