mod cell;
mod direction;

use std::collections::VecDeque;

use dfdx::prelude::Module;
use dfdx::shapes::Rank1;
use dfdx::tensor::{Cpu, Tensor, ZerosTensor};

pub use crate::genetic::genome::Genome;
use crate::genetic::traits::{HasFitness, HasGenes, HasLife, HasSensors, HasTimePerception};
pub use crate::snake::cell::{Point, FIELD_HEIGHT, FIELD_WIDTH};
pub use crate::snake::direction::Direction;

use self::cell::Cell;

#[derive(Clone, Default)]
pub struct Snake {
    genome: Genome,

    cells: VecDeque<Cell>,
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
        self.cells = VecDeque::from([Cell::init_random()]);
        self.direction = Direction::Up;
        self.alive = true;
        self.apple = Cell::init_random();
        self.moves_made = 0;
    }

    fn kill(&mut self) {
        self.moves_made = 100;
    }
}

impl HasTimePerception for Snake {
    fn tick(&mut self) {
        self.direction = match &self.get_nn_prediction() {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!(
                "Cannot convert {} to a direction.",
                &self.get_nn_prediction()
            ),
        };

        // I actually tried to refactor the for loop into an iterator, but with a Vec it requires a streaming/lending iterator I think.
        // While I tried to use that I somehow changed the datastructure to a deque which turns out to not need the loop at all.
        // If you are really motivated I can really recommend looking into rust iterators as they are quite powerful.
        // Iterators also usually result in more efficient code than loops, because the compiler is better at optimizing them.
        // The rust course at my university had some great workshop exercises on iterators, I will attach them to my email.

        let new_head = self.cells[0].add(&self.direction.movement_vector());
        self.cells.push_front(new_head);

        if self.apple.current == self.cells[0].current {
            self.apple = Cell::init_random();
            self.moves_made = 0;
        } else {
            self.cells.pop_back();
        }

        self.alive = self.moves_made < 100
            && self.cells[0].current.0 >= 0
            && self.cells[0].current.0 < FIELD_WIDTH.into()
            && self.cells[0].current.1 >= 0
            && self.cells[0].current.1 < FIELD_HEIGHT.into();

        self.moves_made += 1;
    }
}

impl HasGenes<Snake> for Snake {
    fn crossover(a: &Snake, b: &Snake, mutation_rate: f64) -> Snake {
        let mut child = Snake::new();
        child.genome = a.genome.crossover(&b.genome, mutation_rate);
        child
    }
}

impl Snake {
    pub fn get_nn_prediction(&mut self) -> usize {
        let input = self.get_sensors();
        let dev: Cpu = Default::default();
        let mut x: Tensor<Rank1<6>, f32, Cpu> = dev.zeros();
        // Added as_slice and unwrap_or
        x.copy_from(input.as_slice());
        self.genome
            .neural_network
            .forward(x)
            .as_vec()
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap_or(0)
    }

    pub fn new() -> Snake {
        Snake {
            genome: Genome::new(),
            cells: VecDeque::from([Cell::init_random()]),
            apple: Cell::init_random(),
            direction: Direction::Up,
            alive: true,
            moves_made: 0,
        }
    }

    pub fn get_cells(&self) -> &VecDeque<Cell> {
        &self.cells
    }

    pub fn get_apple(&self) -> &Cell {
        &self.apple
    }

    pub fn get_length(&self) -> usize {
        self.cells.len()
    }
}
