use rand::{thread_rng, Rng};

pub trait GeneticAlgorithm<T> {
    fn crossover(a: T, b: T, mask: u32) -> T;
}

pub trait Crossover {
    fn crossover(a: &Vec<f32>, b: &Vec<f32>, mutation_rate: f64) -> Vec<f32>;
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

impl BitMask for u32 {
    /**
     * Given a number of intersections of partitions of zeros and ones, this function produces a bit mask
     * e. g:
     *      u32::create_bit_mask(2) ->
     *          00001111111111111111111111111111
     *       or 00000000000000001111111111111111 etc.
     */
    fn create_bit_mask(intersections: u8) -> u32 {
        let mut remaining_capacity = 32;
        let mut partitions: Vec<u8> = vec![0; (intersections).into()]
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

impl Crossover for Vec<f32> {
    fn crossover(a: &Vec<f32>, b: &Vec<f32>, mutation_rate: f64) -> Vec<f32> {
        let mut c: Vec<f32> = Vec::with_capacity(a.capacity());
        for i in 0..a.len() {
            let gene_mutation_occurred = thread_rng().gen_bool(mutation_rate);
            let x = if gene_mutation_occurred {
                thread_rng().gen_range(-3.0..3.0)
            } else {
                f32::crossover(a[i], b[i], u32::create_bit_mask(2))
            };
            c.push(x);
        }
        c
    }
}
