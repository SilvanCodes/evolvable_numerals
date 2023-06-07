use std::ops::Range;

use crate::{BinaryPGA2, EvolvableNumeral};

/// An evolvable f64 on a custom range.
pub struct EvolvableF64 {
    genome: BinaryPGA2,
    range: Range<f64>,
}

impl EvolvableNumeral for EvolvableF64 {
    fn representation(&mut self) -> &mut BinaryPGA2 {
        &mut self.genome
    }
}

impl EvolvableF64 {
    pub fn new(lower: f64, upper: f64) -> Self {
        Self {
            genome: BinaryPGA2::new(),
            range: lower..upper,
        }
    }

    pub fn value(&self) -> f64 {
        self.genome.f64(&self.range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_different_values() {
        // let numeral = EvolvableF64::new(0.0, 1.0);

        // assert!((dbg!(numeral.value()) - 0.0).abs() < f64::EPSILON);

        for _ in 0..20 {
            let numeral = EvolvableF64::new(0.0, 1.0);
            dbg!(numeral.value());
        }
    }
}
