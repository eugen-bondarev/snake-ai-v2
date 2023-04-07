mod cell;
mod direction;

use std::collections::HashMap;

use lazy_static::lazy_static;

pub use crate::snake::cell::{Point, FIELD_HEIGHT, FIELD_WIDTH};
pub use crate::snake::direction::Direction;

use self::cell::Cell;

pub struct Snake {
    cells: Vec<Cell>,
    apple: Cell,
    direction: Direction,
    is_alive: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            cells: vec![Cell::init_random()],
            apple: Cell::init_random(),
            direction: Direction::Up,
            is_alive: true,
        }
    }

    pub fn tick(&mut self) {
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

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_apple(&self) -> &Cell {
        &self.apple
    }

    pub fn get_score(&self) -> usize {
        self.cells.len() - 1
    }

    pub fn print(&self) {
        for cell in &self.cells {
            cell.print();
        }
    }
}
