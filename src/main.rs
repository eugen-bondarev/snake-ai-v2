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

use crossterm::{
    cursor::position,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use colored::*;

fn print_events(callback: &mut dyn FnMut() -> ()) -> Result<()> {
    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            callback();
            // Timeout expired, no event for 1s
            // println!("{}", ".\r".on_blue());
        }
    }

    Ok(())
}

use std::io::stdout;

use std::process::Command;

extern crate drawille;

use drawille::Canvas;

fn main() -> Result<()> {
    let mut field = Field::new();
    let mut snakes = vec![Snake::new()];

    enable_raw_mode()?;

    let mut canvas = Canvas::new(10, 10);
    canvas.set(5, 4);
    canvas.line(2, 2, 8, 8);
    assert_eq!(canvas.frame(), [" ⢄    ", "  ⠙⢄  ", "    ⠁ "].join("\n"));

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = print_events(&mut || {
        for snake in &mut snakes {
            snake.tick();
        }
        field.clear();
        field.render_snakes(&snakes);

        // print!("\x1B[2J");
        field.render();
    }) {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}

// fn main() {
//     let mut field = Field::new();
//     let mut snakes = vec![Snake::new()];
//     loop {
//         sleep(Duration::from_millis(500));

//         for snake in &mut snakes {
//             snake.tick();
//         }

//         field.clear();
//         field.render_snakes(&snakes);
//         field.render();
//     }
// }
