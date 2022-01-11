use crate::Color;

use super::font::{FontWeight, FontFamily, FontStyle};



pub enum TextAttribute {
    Family(FontFamily),
    Size(f64),
    Weight(FontWeight),
    Color(Color),
    Style(FontStyle),
    Underline(bool),
    Strikethrough(bool),
}