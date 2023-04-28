mod direction;
mod point;

use std::collections::VecDeque;

use dfdx::prelude::Module;
use dfdx::shapes::Rank1;
use dfdx::tensor::{Cpu, Tensor, ZerosTensor};

pub use crate::genetic::genome::Genome;
use crate::genetic::traits::{HasFitness, HasGenes, HasLife, HasSensors, HasTimePerception};
pub use crate::snake::direction::Direction;
pub use crate::snake::point::{Point, FIELD_HEIGHT, FIELD_WIDTH};

#[derive(Clone, Default)]
pub struct Snake {
    genome: Genome,

    cells: VecDeque<Point>,
    apple: Point,
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
            self.cells[0].y as f32,
            (FIELD_HEIGHT as f32 - self.cells[0].y as f32),
            self.cells[0].x as f32,
            (FIELD_WIDTH as f32 - self.cells[0].x as f32),
            (self.cells[0].x - self.apple.x) as f32,
            (self.cells[0].y - self.apple.y) as f32,
        ]
    }
}

impl HasLife for Snake {
    fn is_alive(&self) -> bool {
        self.alive
    }

    fn reborn(&mut self) {
        self.cells = VecDeque::from([Point::default()]);
        self.direction = Direction::Up;
        self.alive = true;
        self.apple = Point::default();
        self.moves_made = 0;
    }

    fn kill(&mut self) {
        self.moves_made = 100;
    }
}

impl HasTimePerception for Snake {
    fn tick(&mut self) {
        // I actually tried to refactor the for loop into an iterator, but with a Vec it requires a streaming/lending iterator I think.
        // While I tried to use that I somehow changed the datastructure to a deque which turns out to not need the loop at all.
        // If you are really motivated I can really recommend looking into rust iterators as they are quite powerful.
        // Iterators also usually result in more efficient code than loops, because the compiler is better at optimizing them.
        // The rust course at my university had some great workshop exercises on iterators, I will attach them to my email.

        self.direction = self.predict_direction();
        let new_head = self.cells[0] + self.direction.movement_vector();
        self.cells.push_front(new_head);

        if self.apple == self.cells[0] {
            self.apple = Point::default();
            self.moves_made = 0;
        } else {
            self.cells.pop_back();
        }

        self.alive = self.moves_made < 100
            && self.cells[0].x >= 0
            && self.cells[0].x < FIELD_WIDTH.into()
            && self.cells[0].y >= 0
            && self.cells[0].y < FIELD_HEIGHT.into();

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
    /// Use the neural network to predict the direction the snake should move in.
    // This refactoring was purely because I like it more this way, it is not necessary.
    // If this crate were a library one should definitly use Results instead of panicking.
    pub fn predict_direction(&mut self) -> Direction {
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
            .map(|(index, _)| match index {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                _ => panic!("Cannot convert {} to a direction", index),
            })
            .expect("Failed to read neural network output")
    }

    pub fn new() -> Snake {
        Snake {
            genome: Genome::new(),
            cells: VecDeque::from([Point::default()]),
            apple: Point::default(),
            direction: Direction::Up,
            alive: true,
            moves_made: 0,
        }
    }

    pub fn get_cells(&self) -> &VecDeque<Point> {
        &self.cells
    }

    pub fn get_apple(&self) -> &Point {
        &self.apple
    }

    pub fn get_length(&self) -> usize {
        self.cells.len()
    }
}
