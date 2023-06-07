use std::ops::Range;

use bitvec::vec::BitVec;
use rand::Rng;

mod f64;

pub use crate::f64::EvolvableF64;

/// BinaryPGA2 is an implementation of the **P**roportional **G**enetic **A**lgorithm variant 2 introduced in ["The Proportional Genetic Algorithm: Gene Expression in a Genetic Algorithm"][0]  with an alphabet size of two.
///
/// An interesting observation the paper to my knowledge did not make is the interpretation of the expressed value as a sum of random variables.
/// In the case of the binary alphabet summing over the zeroes and ones will give us a value between zero and the length of the underlying bit vector.
/// If we assume each bit to be a i.i.d. random variable we may conclude that for sufficiently many bits the distribution of different sums over the bits will converge to a normal distribution via the central limit theorem.
/// This is interesting in the regard that mutations, i.e. flipping each bit with some chance will usually yield small changes in the resulting sum.
/// When sampling weights e.g. for an artificial neural network this behavior is very desirable as it gradually explores the weight space with few extreme changes to the weights.
/// By projecting the sum of bits in the BinaryPGA2 into an appropriate range (0.0 to 1.0 or -1.0 to 1.0) it becomes an ineresting candidate for evolving weights of ANNs.
/// Further in makes thw weights space discrete with adaptable resolution reducing the search space over weights significantly.
///
/// [0]: https://www.semanticscholar.org/paper/The-Proportional-Genetic-Algorithm-Wu-Garibay/856fb5784da01a72c01ee8ba7ce133c81ffdded5
struct BinaryPGA2(BitVec);

impl BinaryPGA2 {
    /// Create a new instance of BinaryPGA2 with one bit initial resolution.
    pub fn new() -> Self {
        let mut data = BitVec::EMPTY;

        data.push(rand::thread_rng().gen());

        Self(data)
    }

    /// Create a new instance of BinaryPGA2 with one bit initial resolution.
    pub fn with_resolution(resolution: usize) -> Self {
        let mut data = BitVec::with_capacity(resolution);
        let mut rng = rand::thread_rng();

        for _ in 0..resolution {
            data.push(rng.gen());
        }

        Self(data)
    }

    /// Adds a random bit to the underlying BitVec thereby increasing the resolution.
    fn increase_resolution(&mut self) {
        self.0.push(rand::thread_rng().gen())
    }

    /// Removes a bit from the underlying BitVec by popping a bit.
    fn decrease_resolution(&mut self) {
        if self.0.len() > 1 {
            self.0.pop();
        }
    }

    /// Flips every bit in the underlying BitVec with given `mutation_rate`.
    ///
    /// `mutation_rate` needs to be in the range `0.0..=1.0`.
    fn mutate(&mut self, mutation_rate: f64, rng: &mut impl Rng) {
        for bit in &mut self.0 {
            if rng.gen_bool(mutation_rate) {
                let current = bit.clone();
                bit.commit(!current);
            }
        }
    }

    /// Returns the PGA interpretet as an f64 in the given range.
    ///
    /// For sufficiently many bits the mean of values over the course of mutating the instance will be the center of the range.
    pub fn f64(&self, range: &Range<f64>) -> f64 {
        (self.0.count_ones() as f64 / self.0.len() as f64) * (range.end - range.start) + range.start
    }

    /// Returns the PGA interpretet as an f32 in the given range.
    ///
    /// For sufficiently many bits the mean of values over the course of mutating the instance will be the center of the range.
    pub fn f32(&self, range: &Range<f32>) -> f32 {
        (self.0.count_ones() as f32 / self.0.len() as f32) * (range.end - range.start) + range.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_one_bit_resolution() {
        let pga2 = BinaryPGA2::new();

        assert_eq!(pga2.0.len(), 1);
    }

    #[test]
    fn specify_initial_resolution() {
        let pga2 = BinaryPGA2::with_resolution(10);

        assert_eq!(pga2.0.len(), 10);
    }

    #[test]
    fn increase_resolution() {
        let mut pga2 = BinaryPGA2::with_resolution(1);

        pga2.increase_resolution();

        assert_eq!(pga2.0.len(), 2);
    }

    #[test]
    fn decrease_resolution() {
        let mut pga2 = BinaryPGA2::with_resolution(2);

        pga2.decrease_resolution();

        assert_eq!(pga2.0.len(), 1);
    }

    #[test]
    fn resolution_is_at_least_one() {
        let mut pga2 = BinaryPGA2::with_resolution(1);

        pga2.decrease_resolution();

        assert_eq!(pga2.0.len(), 1);
    }

    #[test]
    fn flips_all_bits() {
        let mut pga2 = BinaryPGA2::with_resolution(10);

        let initial_state = pga2.0.clone();

        pga2.mutate(1.0, &mut rand::thread_rng());

        assert_eq!((initial_state & pga2.0).count_ones(), 0);
    }
}
