use dfdx::{
    prelude::{modules, DeviceBuildExt, Linear, ReLU, Sigmoid},
    tensor::Cpu,
};
use rand::{thread_rng, Rng};

type Activation = Sigmoid;

type Model = (
    (Linear<4, 12>, Activation),
    // (Linear<12, 12>, Activation),
    (Linear<12, 4>, Activation),
    Linear<4, 1>,
);
type InitializedModel = (
    (modules::Linear<4, 12, f32, Cpu>, Activation),
    // (modules::Linear<12, 12, f32, Cpu>, Activation),
    (modules::Linear<12, 4, f32, Cpu>, Activation),
    modules::Linear<4, 1, f32, Cpu>,
);

#[derive(Clone)]
pub struct Genome {
    pub neural_network: InitializedModel,
}

trait GeneticAlgorithm<T> {
    fn crossover(a: T, b: T, mask: u32) -> T;
}

impl GeneticAlgorithm<u32> for u32 {
    fn crossover(a: u32, b: u32, mask: u32) -> u32 {
        (a & mask) | (b & !mask)
    }
}

impl GeneticAlgorithm<f32> for f32 {
    fn crossover(a: f32, b: f32, mask: u32) -> f32 {
        // (a + b) / 2.0
        f32::from_bits(u32::crossover(a.to_bits(), b.to_bits(), mask))
    }
}

trait BitMask {
    fn create_bit_mask(intersections: u8) -> u32;
}

// fn get_time() -> Duration {
//     let start = SystemTime::now();
//     start
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards")
// }

impl BitMask for u32 {
    fn create_bit_mask(intersections: u8) -> u32 {
        let mut remaining_capacity = 32;
        let mut partitions: Vec<u8> = vec![0; (intersections - 1).into()]
            .iter()
            .map(|_| {
                let result = thread_rng().gen_range(0..remaining_capacity);
                remaining_capacity -= result;
                result
            })
            .collect();

        partitions.push(remaining_capacity);

        let mut result = String::from("");
        let mut starting_bit = "0";
        for i in partitions {
            for _ in 0..i {
                result += starting_bit;
            }
            starting_bit = if starting_bit == "0" { "1" } else { "0" };
        }

        u32::from_str_radix(result.as_str(), 2).unwrap()
    }
}

trait Crossover {
    fn crossover(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32>;
}

impl Crossover for Vec<f32> {
    fn crossover(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
        let mut c_0: Vec<f32> = Vec::with_capacity(a.capacity());
        for i in 0..a.len() {
            if thread_rng().gen_bool(0.02) {
                c_0.push(thread_rng().gen_range(-1.0..1.0));
            } else {
                let x = f32::crossover(a[i], b[i], u32::create_bit_mask(3));
                c_0.push(x);
            }
        }
        c_0
    }
}

impl Genome {
    pub fn new() -> Genome {
        let dev: Cpu = Default::default();
        let neural_network = dev.build_module::<Model, f32>();
        Genome { neural_network }
    }

    pub fn crossover(a: &Genome, b: &Genome) -> Genome {
        let dev: Cpu = Default::default();
        let mut neural_network = dev.build_module::<Model, f32>();

        let c_0 = Vec::<f32>::crossover(
            &a.neural_network.0 .0.weight.as_vec(),
            &b.neural_network.0 .0.weight.as_vec(),
        );
        let c_1 = Vec::<f32>::crossover(
            &a.neural_network.1 .0.weight.as_vec(),
            &b.neural_network.1 .0.weight.as_vec(),
        );
        // let c_2 = Vec::<f32>::crossover(
        //     &a.neural_network.2 .0.weight.as_vec(),
        //     &b.neural_network.2 .0.weight.as_vec(),
        // );
        let c_3 = Vec::<f32>::crossover(
            &a.neural_network.2.weight.as_vec(),
            &b.neural_network.2.weight.as_vec(),
        );

        let b_0 = Vec::<f32>::crossover(
            &a.neural_network.0 .0.bias.as_vec(),
            &b.neural_network.0 .0.bias.as_vec(),
        );
        let b_1 = Vec::<f32>::crossover(
            &a.neural_network.1 .0.bias.as_vec(),
            &b.neural_network.1 .0.bias.as_vec(),
        );
        // let b_2 = Vec::<f32>::crossover(
        //     &a.neural_network.2 .0.bias.as_vec(),
        //     &b.neural_network.2 .0.bias.as_vec(),
        // );
        let b_3 = Vec::<f32>::crossover(
            &a.neural_network.2.bias.as_vec(),
            &b.neural_network.2.bias.as_vec(),
        );

        // let b_0 = Vec::<f32>::crossover(
        //     &a.neural_network.0 .0.bias.as_vec(),
        //     &b.neural_network.0 .0.bias.as_vec(),
        // );
        // let b_1 = Vec::<f32>::crossover(
        //     &a.neural_network.1.bias.as_vec(),
        //     &b.neural_network.1.bias.as_vec(),
        // );
        // let mut c_0: Vec<f32> = Vec::with_capacity(a_0.capacity());
        // for i in 0..a_0.len() {
        //     let x = f32::crossover(a_0[i], b_0[i], u32::create_bit_mask(3));
        //     c_0.push(x);
        // }

        neural_network.0 .0.weight.copy_from(&c_0[..]);
        neural_network.1 .0.weight.copy_from(&c_1[..]);
        // neural_network.2 .0.weight.copy_from(&c_2[..]);
        neural_network.2.weight.copy_from(&c_3[..]);

        neural_network.0 .0.bias.copy_from(&b_0[..]);
        neural_network.1 .0.bias.copy_from(&b_1[..]);
        // neural_network.2 .0.bias.copy_from(&b_2[..]);
        neural_network.2.bias.copy_from(&b_3[..]);

        Genome { neural_network }
    }
}
