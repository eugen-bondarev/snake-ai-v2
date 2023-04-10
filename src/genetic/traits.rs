pub trait HasFitness {
    fn get_fitness(&self) -> f32;
}

pub trait HasSensors {
    fn get_sensors(&self) -> Vec<f32>;
}

pub trait HasGenes<T> {
    fn crossover(a: &T, b: &T, mutation_rate: f64) -> T;
}

pub trait HasLife {
    fn is_alive(&self) -> bool;

    fn reborn(&mut self);

    fn kill(&mut self);
}

pub trait HasTimePerception {
    fn tick(&mut self);
}
