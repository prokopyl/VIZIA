
pub enum FontFamily {

}

pub struct FontWeight(u16);

impl FontWeight {
    pub const THIN: FontWeight = FontWeight(100);
    pub const HAIRLINE: FontWeight = FontWeight::THIN;

    pub const EXTRA_LIGHT: FontWeight = FontWeight(200);

    pub const LIGHT: FontWeight = FontWeight(300);

    pub const REGULAR: FontWeight = FontWeight(400);
    pub const NORMAL: FontWeight = FontWeight::REGULAR;

    pub const MEDIUM: FontWeight = FontWeight(500);

    pub const SEMI_BOLD: FontWeight = FontWeight(600);

    pub const BOLD: FontWeight = FontWeight(700);

    pub const EXTRA_BOLD: FontWeight = FontWeight(800);

    pub const BLACK: FontWeight = FontWeight(900);
    pub const HEAVY: FontWeight = FontWeight::BLACK;

    pub const EXTRA_BLACK: FontWeight = FontWeight(950);

    /// Create a new `FontWeight` with a custom value.
    ///
    /// Values will be clamped to the range 1..=1000.
    pub fn new(raw: u16) -> FontWeight {
        let raw = raw.clamp(1, 1000);
        FontWeight(raw)
    }

    /// Return the raw value as a u16.
    pub const fn to_raw(self) -> u16 {
        self.0
    }
}

pub enum FontStyle {
    Regular,
    Italic,
}