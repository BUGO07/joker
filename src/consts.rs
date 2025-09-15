pub const CARD_WIDTH: f32 = 290.0;
pub const CARD_HEIGHT: f32 = 400.0;
pub const CARD_SCALE: f32 = 1.0 / 5.0;
pub const CSW: f32 = CARD_WIDTH * CARD_SCALE;
pub const CSH: f32 = CARD_HEIGHT * CARD_SCALE;
pub const ASSETS: &[&str] = &[
    "JR", "JB", "S7", "S8", "S9", "S1", "SJ", "SQ", "SK", "SA", "D6", "D7", "D8", "D9", "D1", "DJ",
    "DQ", "DK", "DA", "C7", "C8", "C9", "C1", "CJ", "CQ", "CK", "CA", "H6", "H7", "H8", "H9", "H1",
    "HJ", "HQ", "HK", "HA",
];

pub const DEFAULT_FONT_WIDTH: f32 = 20.0;
pub const HALF_FONT_HEIGHT: f32 = DEFAULT_FONT_WIDTH * 1.2 * 0.5;
