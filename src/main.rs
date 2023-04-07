mod cell;

use std::collections::HashMap;

use cell::Cell;
use cell::Point;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    cells: Vec<Cell>,
    apple: Cell,
    direction: Direction,
    is_alive: bool,
}

use lazy_static::lazy_static;

impl Snake {
    fn new() -> Snake {
        Snake {
            cells: vec![Cell::init_random()],
            apple: Cell::init_random(),
            direction: Direction::Up,
            is_alive: true,
        }
    }

    fn tick(&mut self) {
        if !self.is_alive {
            self.cells = vec![Cell::init_random()];
            self.direction = Direction::Up;
            self.is_alive = true;
            self.apple = Cell::init_random();
        }

        lazy_static! {
            static ref MAP: HashMap<Direction, Point> = vec![
                (Direction::Up, (0, -1)),
                (Direction::Down, (0, 1)),
                (Direction::Left, (-1, 0)),
                (Direction::Right, (1, 0)),
            ]
            .into_iter()
            .collect();
        }

        let matching_point = MAP[&self.direction];

        for i in 0..self.cells.len() {
            self.cells[i].prev = self.cells[i].current;
            if i > 0 {
                self.cells[i].current = self.cells[i - 1].prev;
            }
        }

        self.cells[0].add(&matching_point);

        self.is_alive = self.cells[0].current.0 >= 0
            && self.cells[0].current.0 < FIELD_WIDTH.into()
            && self.cells[0].current.1 >= 0
            && self.cells[0].current.1 < FIELD_HEIGHT.into();

        if self.apple.current == self.cells[0].current {
            self.cells.push(Cell {
                current: self.cells[0].prev,
                prev: (0, 0),
            });
            self.apple = Cell::init_random();
        }
    }

    fn print(&self) {
        for cell in &self.cells {
            cell.print();
        }
    }
}

use console_engine::pixel;
use console_engine::Color;
use console_engine::ConsoleEngine;
use console_engine::KeyCode;

use crate::cell::FIELD_HEIGHT;
use crate::cell::FIELD_WIDTH;

fn draw_borders(canvas: &mut ConsoleEngine) {
    let border_color = Color::DarkRed;

    let corner_pixel = pixel::pxl_bg(' ', border_color);
    let horizontal_pixel = pixel::pxl_bg(' ', border_color);
    let vertical_pixel = pixel::pxl_bg(' ', border_color);

    canvas.set_pxl(0, 0, corner_pixel);
    canvas.set_pxl((FIELD_WIDTH - 1).into(), 0, corner_pixel);
    canvas.set_pxl(
        (FIELD_WIDTH - 1).into(),
        (FIELD_HEIGHT - 1).into(),
        corner_pixel,
    );
    canvas.set_pxl(0, (FIELD_HEIGHT - 1).into(), corner_pixel);

    for x in 1..FIELD_WIDTH - 1 {
        canvas.set_pxl(x.into(), 0, horizontal_pixel);
        canvas.set_pxl(x.into(), (FIELD_HEIGHT - 1).into(), horizontal_pixel);
    }
    for y in 1..FIELD_HEIGHT - 1 {
        canvas.set_pxl(0, y.into(), vertical_pixel);
        canvas.set_pxl((FIELD_WIDTH - 1).into(), y.into(), vertical_pixel);
    }
}

fn main() {
    let mut snakes: Vec<Snake> = vec![Snake::new()];

    let mut engine = ConsoleEngine::init(FIELD_WIDTH.into(), FIELD_HEIGHT.into(), 15).unwrap();

    loop {
        engine.wait_frame();
        engine.clear_screen();

        // draw_borders(&mut engine);

        for snake in &mut snakes {
            snake.tick();
            engine.set_pxl(
                snake.apple.current.0,
                snake.apple.current.1,
                pixel::pxl_bg(' ', Color::Red),
            );
            for cell in &snake.cells {
                engine.set_pxl(
                    cell.current.0.into(),
                    cell.current.1.into(),
                    pixel::pxl_bg(' ', Color::Green),
                );
            }
        }

        if engine.is_key_pressed(KeyCode::Char('d')) {
            snakes[0].direction = Direction::Right;
        }

        if engine.is_key_pressed(KeyCode::Char('a')) {
            snakes[0].direction = Direction::Left;
        }

        if engine.is_key_pressed(KeyCode::Char('w')) {
            snakes[0].direction = Direction::Up;
        }

        if engine.is_key_pressed(KeyCode::Char('s')) {
            snakes[0].direction = Direction::Down;
        }

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        engine.draw();
    }
}
