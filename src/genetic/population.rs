use std::sync::{Arc, Mutex};

use super::traits::{HasFitness, HasGenes, HasLife, HasTimePerception};

use rayon::prelude::*;

pub struct Population<T> {
    capacity: usize,
    genomes: Vec<T>,
    pub alive_genomes_count: Arc<Mutex<usize>>,
    pub max_fitness_current: Arc<Mutex<f32>>,

    pub generation: usize,
    pub max_fitness_prev: f32,
    pub mutation_rate: f64,
}

use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};

fn generate_random_number_tending_towards_smaller(n: u32, m: u32, small_likelihood: f64) -> u32 {
    let mean = (n + m) / 2;
    let std_dev = (m - n) / 4;
    let normal = Normal::new(mean as f64, std_dev as f64).unwrap();

    let mut rng = thread_rng();
    let mut num;
    loop {
        num = normal.sample(&mut rng) as u32;
        if num >= n && num <= m {
            break;
        }
    }

    let rand_num = rng.gen_range(0.0..1.0);
    if rand_num <= small_likelihood {
        n + rng.gen_range(0..(num - n).max(1))
    } else {
        num + rng.gen_range(0..(m - num + 1).max(1))
    }
}

impl<T> Population<T>
where
    T: Clone + Sync + Send + Default + HasLife + HasFitness + HasTimePerception + HasGenes<T>,
{
    pub fn new(capacity: usize) -> Self {
        let mut genomes: Vec<T> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            genomes.push(Default::default());
        }
        Population {
            genomes,
            capacity,
            alive_genomes_count: Arc::new(Mutex::new(0)),
            max_fitness_current: Arc::new(Mutex::new(0.0)),
            mutation_rate: 0.01,
            generation: 0,
            max_fitness_prev: 0.0,
        }
    }

    pub fn reborn(&mut self) {
        for genome in &mut self.genomes {
            genome.reborn();
        }
    }

    pub fn tick(&mut self) {
        self.max_fitness_current = Arc::new(Mutex::<f32>::new(0.0));
        self.alive_genomes_count = Arc::new(Mutex::new(0));

        let batch_size = self.genomes.len() / num_cpus::get();
        let batches: Vec<_> = self.genomes.chunks_mut(batch_size).collect();

        batches.into_par_iter().for_each(|batch| {
            for item in batch {
                if item.get_fitness() > *self.max_fitness_current.lock().unwrap() {
                    *self.max_fitness_current.lock().unwrap() = item.get_fitness();
                }
                if !item.is_alive() {
                    continue;
                }
                item.tick();
                *self.alive_genomes_count.lock().unwrap() += 1;
            }
        });
    }

    pub fn is_dead(&self) -> bool {
        *self.alive_genomes_count.lock().unwrap() == 0
    }

    pub fn evolution(&mut self) {
        self.get_genomes()
            .sort_by_key(|snake| (snake.get_fitness() as i32) * -1);
        let capacity = self.get_capacity();
        let slice = self.get_genomes()[0..capacity / 10].to_vec();

        let mut new_population: Vec<T> = vec![];

        let progress = *self.max_fitness_current.lock().unwrap() > self.max_fitness_prev;
        self.max_fitness_prev = *self.max_fitness_current.lock().unwrap();

        if progress {
            self.mutation_rate -= self.mutation_rate * 0.1;
        } else {
            self.mutation_rate += self.mutation_rate * 0.1;
        }
        self.mutation_rate = f64::clamp(self.mutation_rate, 0.00005 as f64, 0.05 as f64);

        for _ in (0..self.get_capacity()).step_by(2) {
            let parent_a = &slice[generate_random_number_tending_towards_smaller(
                0,
                slice.len() as u32 - 1,
                0.9,
            ) as usize];
            let parent_b = &slice[generate_random_number_tending_towards_smaller(
                0,
                slice.len() as u32 - 1,
                0.9,
            ) as usize];
            new_population.push(T::crossover(&parent_a, &parent_b, self.mutation_rate));
        }

        self.get_genomes().clear();
        for snake in new_population {
            self.get_genomes().push(snake);
        }

        self.reborn();
        self.generation += 1;
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn get_genomes(&mut self) -> &mut Vec<T> {
        &mut self.genomes
    }
}
