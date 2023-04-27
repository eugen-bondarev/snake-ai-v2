use rand::Rng;

// const is usually better than static because it will always get inlined.
// https://stackoverflow.com/a/65475478
pub const FIELD_WIDTH: u8 = 32;
pub const FIELD_HEIGHT: u8 = 32;

/// A point in a 2D space.
//
// It is usually a good idea to derive everything that makes sense for a datatype.
// https://users.rust-lang.org/t/what-traits-should-i-normally-derive/484/9
//
// For libraries it is recommended to eagerly derive or implement common types.
// https://rust-lang.github.io/api-guidelines/interoperability.html#c-common-traits
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Default for Point {
    /// Creates a new point with random coordinates.
    ///
    /// Random coordinates are used, because usually when we need a point and dont know the location it is random.
    ///
    /// I am not sure if Default should provide a random point or something like 0,0.
    ///
    /// The documentation for Default says:
    /// _Default values are often some kind of initial value, identity value, or anything else that may make sense as a default._
    ///
    /// I think a random point somewhat makes sense as a default, but it is definitely not a good initial value.
    ///
    /// Also I am not sure if one would expect `Point::default() == Point::default()` to be true.
    fn default() -> Point {
        Point {
            x: rand::thread_rng().gen_range(0..FIELD_WIDTH).into(),
            y: rand::thread_rng().gen_range(0..FIELD_HEIGHT).into(),
        }
    }
}

impl Point {
    /// Creates a new point with the given coordinates.
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    /// Creates a new point from the coordinates of the given point plus the given offset.
    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}
