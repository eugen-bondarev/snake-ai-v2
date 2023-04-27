mod point;

pub use point::Point;
use rand::Rng;

// const is usually better than static because it will always get inlined.
// https://stackoverflow.com/a/65475478
pub const FIELD_WIDTH: u8 = 32;
pub const FIELD_HEIGHT: u8 = 32;

#[derive(Clone, Default)]
pub struct Cell {
    pub current: Point,
    pub prev: Point,
}

impl Cell {
    pub fn init_random() -> Cell {
        Cell {
            current: (
                rand::thread_rng().gen_range(0..FIELD_WIDTH).into(),
                rand::thread_rng().gen_range(0..FIELD_HEIGHT).into(),
            ),
            prev: (0, 0),
        }
    }

    pub fn add(&mut self, point: &Point) {
        self.current.0 += point.0;
        self.current.1 += point.1;
    }
}
