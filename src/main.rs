use std::vec;

use rand::Rng;

static FIELD_WIDTH: u8 = 16;
static FIELD_HEIGHT: u8 = 16;

struct Cell {
    x: u8,
    y: u8,
}

impl Cell {
    fn init_random() -> Cell {
        Cell {
            x: rand::thread_rng().gen_range(0..FIELD_WIDTH),
            y: rand::thread_rng().gen_range(0..FIELD_HEIGHT),
        }
    }

    fn print(&self) {
        println!("{0} {1}", self.x, self.y);
    }
}

struct Snake {
    cells: Vec<Cell>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            cells: vec![Cell::init_random()],
        }
    }

    fn print(&self) {
        for cell in &self.cells {
            cell.print();
        }
    }
}

#[derive(Copy, Clone)]
enum CellState {
    Empty,
    Snake,
    Apple,
}

struct Field {
    cells: Vec<Vec<i32>>,
}

fn fill<T>(size: usize, val: T) -> Vec<T>
where
    T: Copy,
{
    let mut zero_vec: Vec<T> = Vec::with_capacity(size);
    for i in 0..size {
        zero_vec.push(val);
    }
    zero_vec
}

impl Field {
    fn new() -> Field {
        let foo = fill(FIELD_HEIGHT.into(), 3);
        Field {
            cells: fill(FIELD_WIDTH.into(), foo),
        }
    }
}

// fn render_field(snakes: &Vec<Snake>) {
//     for _ in 0..FIELD_HEIGHT {
//         for _ in 0..FIELD_WIDTH {
//             print!(" ");
//         }
//         print!("\n");
//     }
// }

fn main() {
    // let mut vec_2d: Vec<Vec<CellState>> = vec![vec![CellState::Empty]];
    // vec_2d.push(vec![CellState::Apple]);
    let field = Field::new();
}
