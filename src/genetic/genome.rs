use dfdx::{
    prelude::{modules, DeviceBuildExt, Linear, ReLU},
    tensor::Cpu,
};

use super::algorithms::GeneticCrossover;

type Activation = ReLU;

/**
 * Code smell
 */
type Model = ((Linear<6, 4>, Activation), Linear<4, 4>);
type InitializedModel = (
    (modules::Linear<6, 4, f32, Cpu>, Activation),
    modules::Linear<4, 4, f32, Cpu>,
);

#[derive(Clone)]
pub struct Genome {
    pub neural_network: InitializedModel,
}

impl Default for Genome {
    fn default() -> Self {
        let dev: Cpu = Default::default();
        let neural_network = dev.build_module::<Model, f32>();
        Genome { neural_network }
    }
}

impl Genome {
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * I hate this code.
     */
    pub fn crossover(a: &Genome, b: &Genome, mutation_rate: f64) -> Genome {
        let dev: Cpu = Default::default();
        let mut child_neural_network = dev.build_module::<Model, f32>();

        let c_0 = Vec::<f32>::crossover(
            &a.neural_network.0 .0.weight.as_vec(),
            &b.neural_network.0 .0.weight.as_vec(),
            mutation_rate,
        );
        let c_1 = Vec::<f32>::crossover(
            &a.neural_network.1.weight.as_vec(),
            &b.neural_network.1.weight.as_vec(),
            mutation_rate,
        );

        let b_0 = Vec::<f32>::crossover(
            &a.neural_network.0 .0.bias.as_vec(),
            &b.neural_network.0 .0.bias.as_vec(),
            mutation_rate,
        );
        let b_1 = Vec::<f32>::crossover(
            &a.neural_network.1.bias.as_vec(),
            &b.neural_network.1.bias.as_vec(),
            mutation_rate,
        );

        child_neural_network.0 .0.weight.copy_from(&c_0[..]);
        child_neural_network.1.weight.copy_from(&c_1[..]);

        child_neural_network.0 .0.bias.copy_from(&b_0[..]);
        child_neural_network.1.bias.copy_from(&b_1[..]);

        Genome {
            neural_network: child_neural_network,
        }
    }
}
