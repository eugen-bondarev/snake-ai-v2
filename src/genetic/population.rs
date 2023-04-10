use super::traits::HasLife;

pub struct Population<T> {
    capacity: usize,
    genomes: Vec<T>,
}

impl<T> Population<T>
where
    T: Default,
    T: HasLife,
{
    pub fn new(capacity: usize) -> Self {
        let mut genomes: Vec<T> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            genomes.push(Default::default());
        }
        Population { genomes, capacity }
    }

    pub fn reborn(&mut self) {
        for genome in &mut self.genomes {
            genome.reborn();
        }
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn get_genomes(&mut self) -> &mut Vec<T> {
        &mut self.genomes
    }
}
