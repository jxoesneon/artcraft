//! Artcraft unified theme.

pub struct Color(pub u8, pub u8, pub u8);

pub struct Theme {
    pub bg: Color,
    pub card: Color,
    pub accent: Color,
}

impl Theme {
    pub fn hub_dark() -> Self {
        Self {
            bg: Color(18, 20, 26),
            card: Color(26, 30, 38),
            accent: Color(0, 229, 255),
        }
    }
}
