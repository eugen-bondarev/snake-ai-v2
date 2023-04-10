mod cell;
mod direction;

use std::collections::HashMap;

use dfdx::prelude::Module;
use dfdx::shapes::Rank1;
use dfdx::tensor::{Cpu, Tensor, ZerosTensor};
use lazy_static::lazy_static;

pub use crate::genetic::genome::Genome;
use crate::genetic::traits::{HasFitness, HasGenes, HasLife, HasSensors, HasTimePerception};
pub use crate::snake::cell::{Point, FIELD_HEIGHT, FIELD_WIDTH};
pub use crate::snake::direction::Direction;

use self::cell::Cell;

#[derive(Clone, Default)]
pub struct Snake {
    genome: Genome,

    cells: Vec<Cell>,
    apple: Cell,
    direction: Direction,
    alive: bool,
    moves_made: i32,
}

impl HasFitness for Snake {
    fn get_fitness(&self) -> f32 {
        self.get_length() as f32
    }
}

impl HasSensors for Snake {
    fn get_sensors(&self) -> Vec<f32> {
        vec![
            self.cells[0].current.1 as f32,
            (FIELD_HEIGHT as f32 - self.cells[0].current.1 as f32),
            self.cells[0].current.0 as f32,
            (FIELD_WIDTH as f32 - self.cells[0].current.0 as f32),
            (self.cells[0].current.0 - self.apple.current.0) as f32,
            (self.cells[0].current.1 - self.apple.current.1) as f32,
        ]
    }
}

impl HasLife for Snake {
    fn is_alive(&self) -> bool {
        self.alive
    }

    fn reborn(&mut self) {
        self.cells = vec![Cell::init_random()];
        self.direction = Direction::Up;
        self.alive = true;
        self.apple = Cell::init_random();
        self.moves_made = 0;
    }
}

impl HasTimePerception for Snake {
    fn tick(&mut self) {
        lazy_static! {
            static ref PREDICTION_DIRECTION_MAP: HashMap<usize, Direction> = vec![
                (0, Direction::Up),
                (1, Direction::Down),
                (2, Direction::Left),
                (3, Direction::Right),
            ]
            .into_iter()
            .collect();
        }

        /*
         * This seems kinda unsafe..
         */
        self.direction = PREDICTION_DIRECTION_MAP[&self.get_nn_prediction()];

        lazy_static! {
            static ref DIRECTION_MOVEMENT_MAP: HashMap<Direction, Point> = vec![
                (Direction::Up, (0, -1)),
                (Direction::Down, (0, 1)),
                (Direction::Left, (-1, 0)),
                (Direction::Right, (1, 0)),
            ]
            .into_iter()
            .collect();
        }
        let matching_point = DIRECTION_MOVEMENT_MAP[&self.direction];

        for i in 0..self.cells.len() {
            self.cells[i].prev = self.cells[i].current;
            if i > 0 {
                self.cells[i].current = self.cells[i - 1].prev;
            }
        }

        self.cells[0].add(&matching_point);

        self.alive = self.moves_made < 100
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
}

impl HasGenes<Snake> for Snake {
    fn crossover(a: &Snake, b: &Snake, mutation_rate: f64) -> Snake {
        let mut child = Snake::new();
        child.genome = Genome::crossover(&a.genome, &b.genome, mutation_rate);
        child
    }
}

impl Snake {
    pub fn get_nn_prediction(&mut self) -> usize {
        let input = self.get_sensors();
        let dev: Cpu = Default::default();
        let mut x: Tensor<Rank1<6>, f32, Cpu> = dev.zeros();
        x.copy_from(&input[0..input.len()]);
        match (self.genome.neural_network.forward(x).as_vec())
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
        {
            Some(v) => v,
            None => 0,
        }
    }

    pub fn new() -> Snake {
        Snake {
            genome: Genome::new(),
            cells: vec![Cell::init_random()],
            apple: Cell::init_random(),
            direction: Direction::Up,
            alive: true,
            moves_made: 0,
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

    pub fn get_length(&self) -> usize {
        self.cells.len() - 1
    }
}
