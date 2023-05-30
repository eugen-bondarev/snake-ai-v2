use rand::{thread_rng, Rng};

/// A trait with only associated functions is (more or less) just a namespace and should probably be a mod.
/// Converted crossover to a method.
///
/// Defining it as a method allows us to call crossover on any Vec<f32> instance (if the trait is in scope).
pub trait GeneticCrossover {
    fn crossover(&self, b: &Vec<f32>, mutation_rate: f64) -> Vec<f32>;
}

impl GeneticCrossover for Vec<f32> {
    fn crossover(&self, b: &Vec<f32>, mutation_rate: f64) -> Vec<f32> {
        let a = self;
        let mut c: Vec<f32> = Vec::with_capacity(a.capacity());
        for i in 0..a.len() {
            let gene_mutation_occurred = thread_rng().gen_bool(mutation_rate);
            let x = if gene_mutation_occurred {
                thread_rng().gen_range(-3.0..3.0)
            } else {
                a[i].crossover(b[i], u32::create_bit_mask(2))
            };
            c.push(x);
        }
        c
    }
}

pub trait NumericCrossover<T> {
    fn crossover(self, b: T, mask: u32) -> T;
}

impl NumericCrossover<u32> for u32 {
    fn crossover(self, b: u32, mask: u32) -> u32 {
        (self & mask) | (b & !mask)
    }
}

impl NumericCrossover<f32> for f32 {
    fn crossover(self, b: f32, mask: u32) -> f32 {
        f32::from_bits(u32::crossover(self.to_bits(), b.to_bits(), mask))
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
     *          0000 11111111111111111111 00000000
     *              ^                    ^
     *              |                    |
     *              intersection         intersection
     *       or 0000000000000000 11111111111111 00 etc.
     *                          ^              ^
     *                          |              |
     *                          intersection   intersection
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
