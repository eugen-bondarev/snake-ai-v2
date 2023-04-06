use std::{thread::sleep, time::Duration, vec};

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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    cells: Vec<Cell>,
    direction: Direction,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            cells: vec![Cell::init_random()],
            direction: Direction::Up,
        }
    }

    fn tick(&mut self) {
        if matches!(self.direction, Direction::Up) {
            self.cells[0].y -= 1;
        }
        if matches!(self.direction, Direction::Down) {
            self.cells[0].y += 1;
        }
        if matches!(self.direction, Direction::Left) {
            self.cells[0].x -= 1;
        }
        if matches!(self.direction, Direction::Right) {
            self.cells[0].x += 1;
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
    cell_cols: Vec<Vec<CellState>>,
}

impl Field {
    fn new() -> Field {
        let mut cols: Vec<Vec<CellState>> = Vec::with_capacity(FIELD_WIDTH.into());
        for _ in 0..FIELD_WIDTH {
            let mut cells: Vec<CellState> = Vec::with_capacity(FIELD_HEIGHT.into());
            for _ in 0..FIELD_HEIGHT {
                cells.push(CellState::Empty);
            }
            cols.push(cells);
        }
        Field { cell_cols: cols }
    }

    fn clear(&mut self) {
        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGHT {
                self.cell_cols[usize::from(x)][usize::from(y)] = CellState::Empty;
            }
        }
    }

    fn render_snakes(&mut self, snakes: &Vec<Snake>) {
        for snake in snakes {
            for cell in &snake.cells {
                self.cell_cols[usize::from(cell.x)][usize::from(cell.y)] = CellState::Snake;
            }
        }
    }

    fn render(&self) {
        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGHT {
                print!(
                    "{}",
                    if matches!(
                        self.cell_cols[usize::from(y)][usize::from(x)],
                        CellState::Empty
                    ) {
                        " "
                    } else {
                        "S"
                    }
                );
            }
            print!("\n");
        }
    }
}

fn main() {
    let mut field = Field::new();
    let mut snakes = vec![Snake::new()];
    loop {
        sleep(Duration::from_millis(500));

        for snake in &mut snakes {
            snake.tick();
        }

        field.clear();
        field.render_snakes(&snakes);
        field.render();
    }
}
