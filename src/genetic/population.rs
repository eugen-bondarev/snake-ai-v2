use std::sync::{Arc, Mutex};

use super::traits::{HasFitness, HasLife, HasTimePerception};

use rayon::prelude::*;

pub struct Population<T> {
    capacity: usize,
    genomes: Vec<T>,
    pub alive_genomes_count: Arc<Mutex<usize>>,
    pub max_fitness_current: Arc<Mutex<f32>>,
}

impl<T> Population<T>
where
    T: Sync + Send + Default + HasLife + HasFitness + HasTimePerception,
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

        // for item in &mut self.genomes {
        //     if item.get_fitness() > *self.max_fitness_current.lock().unwrap() {
        //         *self.max_fitness_current.lock().unwrap() = item.get_fitness();
        //     }
        //     if !item.is_alive() {
        //         continue;
        //     }
        //     item.tick();
        //     *self.alive_genomes_count.lock().unwrap() += 1;
        // }
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

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn get_genomes(&mut self) -> &mut Vec<T> {
        &mut self.genomes
    }
}
