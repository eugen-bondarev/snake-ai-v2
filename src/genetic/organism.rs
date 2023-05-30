pub trait Organism: Clone + Send + Default {
    /// Get the current fitness of an organism
    fn get_fitness(&self) -> f32;

    /// Retrieves the current state about the environment of the organism
    fn get_sensors(&self) -> Vec<f32>;

    /// Create a new organism from two parents
    fn crossover(&self, b: &Self, mutation_rate: f64) -> Self;

    /// Check if this organism is alive
    fn is_alive(&self) -> bool;

    /// Reset this organism to its initial state
    fn reborn(&mut self);

    /// Kill the organism on the next tick
    fn kill(&mut self);

    /// Process the next tick
    fn tick(&mut self);
}
