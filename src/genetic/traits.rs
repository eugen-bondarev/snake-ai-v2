pub trait HasFitness {
    fn get_fitness(&self) -> f32;
}

pub trait HasSensors {
    fn get_sensors(&self) -> Vec<f32>;
}

pub trait HasLife {
    fn is_alive(&self) -> bool;

    fn reborn(&mut self);
}

pub trait HasTimePerception {
    fn tick(&mut self);
}
