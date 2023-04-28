use super::organism::Organism;

use rayon::prelude::*;

pub struct Population<T> {
    capacity: usize,
    genomes: Vec<T>,
    pub alive_genomes_count: usize,
    pub max_fitness_current: f32,

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

    let num = normal
        .sample_iter(thread_rng())
        .map(|x| x as u32)
        .find(|sample| sample >= &n && sample <= &m)
        .unwrap_or(0u32);

    let mut rng = thread_rng();
    let rand_num = rng.gen_range(0.0..1.0);
    if rand_num <= small_likelihood {
        n + rng.gen_range(0..(num - n).max(1))
    } else {
        num + rng.gen_range(0..(m - num + 1).max(1))
    }
}

impl<T: Organism> Population<T> {
    pub fn new(capacity: usize) -> Self {
        let mut genomes: Vec<T> = Vec::with_capacity(capacity);
        (0..capacity).for_each(|_| {
            genomes.push(Default::default());
        });
        Population {
            genomes,
            capacity,
            alive_genomes_count: 0,
            max_fitness_current: 0.0,
            mutation_rate: 0.01,
            generation: 0,
            max_fitness_prev: 0.0,
        }
    }

    pub fn reborn(&mut self) {
        self.genomes.iter_mut().for_each(|genome| {
            genome.reborn();
        });
    }

    pub fn kill(&mut self) {
        self.genomes.iter_mut().for_each(|genome| {
            genome.kill();
        });
    }

    pub fn tick(&mut self) {
        struct TickResult {
            best_fitness: f32,
            survivors: usize,
        }

        // Rayon parallel iter => nice
        //
        // I refactored it to use map so we dont need to use a mutex
        //
        // Removed batches as rayon handles it anyways. I think that sped up the process a bit, but did no real benchmarking
        //
        // Even without batching rayon creates only 20 threads on my computer, so I think we are fine
        let tick_result = self
            .genomes
            .par_iter_mut()
            .map(|organism| {
                if !organism.is_alive() {
                    return TickResult {
                        best_fitness: 0.0,
                        survivors: 0,
                    };
                }
                organism.tick();
                let organism_fitness = organism.get_fitness();
                TickResult {
                    best_fitness: organism_fitness,
                    survivors: 1,
                }
            })
            .reduce(
                || TickResult {
                    best_fitness: 0f32,
                    survivors: 0,
                },
                |result, batch_result| TickResult {
                    best_fitness: result.best_fitness.max(batch_result.best_fitness),
                    survivors: result.survivors + batch_result.survivors,
                },
            );

        self.max_fitness_current = tick_result.best_fitness;
        self.alive_genomes_count = tick_result.survivors;
    }

    pub fn is_dead(&self) -> bool {
        self.alive_genomes_count == 0
    }

    pub fn evolution(&mut self) {
        self.get_genomes()
            .sort_by_key(|snake| (snake.get_fitness() as i32) * -1);
        let capacity = self.get_capacity();
        let slice = self.get_genomes()[0..capacity / 10].to_vec();

        let mut new_population: Vec<T> = vec![];

        let progress = self.max_fitness_current > self.max_fitness_prev;
        self.max_fitness_prev = self.max_fitness_current;

        if progress {
            self.mutation_rate -= self.mutation_rate * 0.1;
        } else {
            self.mutation_rate += self.mutation_rate * 0.1;
        }
        self.mutation_rate = f64::clamp(self.mutation_rate, 0.00005 as f64, 0.05 as f64);

        (0..self.get_capacity()).step_by(2).for_each(|_| {
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
            new_population.push(parent_a.crossover(&parent_b, self.mutation_rate));
        });

        self.get_genomes().clear();
        new_population.into_iter().for_each(|snake| {
            self.get_genomes().push(snake);
        });

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
