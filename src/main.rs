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

enum CellState {
    Empty,
    Snake,
    Apple,
}

struct Field {
    cells: Vec<Vec<CellState>>,
}

use std::iter;

impl Field {
    fn new() -> Field {
        Field { cells }
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
    // render_field(&vec![Snake::new()]);
    let field = Field::new();
}
