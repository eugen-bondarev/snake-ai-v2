mod point;

pub use point::Point;
use rand::Rng;

pub static FIELD_WIDTH: u8 = 32;
pub static FIELD_HEIGHT: u8 = 32;

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

    pub fn print(&self) {
        println!("{0}, {1}", self.current.0, self.current.1);
    }
}
