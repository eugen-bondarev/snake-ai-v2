mod point;

pub use point::Point;

// const is usually better than static because it will always get inlined.
// https://stackoverflow.com/a/65475478
pub const FIELD_WIDTH: u8 = 32;
pub const FIELD_HEIGHT: u8 = 32;
