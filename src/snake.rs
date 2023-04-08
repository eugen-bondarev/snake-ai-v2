mod cell;
mod direction;

use std::collections::HashMap;

use dfdx::prelude::{DeviceBuildExt, ModuleMut};
use dfdx::shapes::Rank1;
use dfdx::tensor::{Cpu, Tensor, ZerosTensor};
use lazy_static::lazy_static;

pub use crate::genetic::genome::Genome;
pub use crate::snake::cell::{Point, FIELD_HEIGHT, FIELD_WIDTH};
pub use crate::snake::direction::Direction;

use self::cell::Cell;

#[derive(Clone)]
pub struct Snake {
    genome: Genome,

    cells: Vec<Cell>,
    apple: Cell,
    direction: Direction,
    is_alive: bool,
    moves_made: i32,
}

impl Snake {
    fn get_nn_input(&self) -> Vec<f32> {
        vec![
            (self.cells[0].current.0 as f32) / (FIELD_WIDTH as f32),
            FIELD_WIDTH as f32 - self.cells[0].current.0 as f32 / FIELD_WIDTH as f32,
            (self.cells[0].current.1 as f32) / (FIELD_HEIGHT as f32),
            FIELD_HEIGHT as f32 - self.cells[0].current.1 as f32 / FIELD_HEIGHT as f32,
            (self.cells[0].current.0 - self.apple.current.0) as f32 / (FIELD_WIDTH as f32),
            (self.cells[0].current.1 - self.apple.current.1) as f32 / (FIELD_HEIGHT as f32),
        ]
    }

    pub fn get_nn_prediction(&mut self) -> f32 {
        let input = self.get_nn_input();
        let dev: Cpu = Default::default();
        let mut x: Tensor<Rank1<6>, f32, Cpu> = dev.zeros();
        x.copy_from(&input[0..input.len()]);
        (self.genome.neural_network.forward_mut(x).as_vec()[0])
        // .iter()
        // .enumerate()
        // .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        // .map(|(i, _)| i)
        // .unwrap()
    }

    pub fn new() -> Snake {
        Snake {
            genome: Genome::new(),
            cells: vec![Cell::init_random()],
            apple: Cell::init_random(),
            direction: Direction::Up,
            is_alive: true,
            moves_made: 0,
        }
    }

    pub fn crossover(a: &Snake, b: &Snake) -> Snake {
        Snake {
            genome: Genome::crossover(&a.genome, &b.genome),
            cells: vec![Cell::init_random()],
            apple: Cell::init_random(),
            direction: Direction::Up,
            is_alive: true,
            moves_made: 0,
        }
    }

    pub fn reborn(&mut self) {
        self.cells = vec![Cell::init_random()];
        self.direction = Direction::Up;
        self.is_alive = true;
        self.apple = Cell::init_random();
    }

    pub fn get_is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn tick(&mut self) {
        // if !self.is_alive {
        //     self.reborn();
        // }

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

        self.direction = match self.get_nn_prediction() as usize {
            x if x == 0 => Direction::Up,
            x if x == 1 => Direction::Down,
            x if x == 2 => Direction::Left,
            x if x == 3 => Direction::Right,
            _ => Direction::Up,
        };

        let matching_point = MAP[&self.direction];

        for i in 0..self.cells.len() {
            self.cells[i].prev = self.cells[i].current;
            if i > 0 {
                self.cells[i].current = self.cells[i - 1].prev;
            }
        }

        self.cells[0].add(&matching_point);

        self.is_alive = self.moves_made < 100
            && self.cells[0].current.0 >= 0
            && self.cells[0].current.0 < FIELD_WIDTH.into()
            && self.cells[0].current.1 >= 0
            && self.cells[0].current.1 < FIELD_HEIGHT.into();

        if self.apple.current == self.cells[0].current {
            self.cells.push(Cell {
                current: self.cells[0].prev,
                prev: (0, 0),
            });
            self.apple = Cell::init_random();
            self.moves_made = 0;
        }

        self.moves_made += 1;
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
