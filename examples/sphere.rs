// この"use"は必須
// This "use" is required
use ys_differential_evolution::{*, population::minimum::*, individual::minimum::*};
type FLOAT = f32;

#[derive(Clone)]
struct IndividualMinimum
{
    genes: Vec<FLOAT>,
    converted_genes: Vec<FLOAT>,
    evaluation_values: Vec<FLOAT>,
}

impl IndividualMinimumBase<FLOAT> for IndividualMinimum {
    IndividualMinimumImpl!();
    fn convert(&mut self) {
        self.converted_genes = self.genes.clone(); // そのまま
    }
    fn evaluate(&mut self) {
        self.evaluation_values = vec![self.converted_genes.iter().fold(0.0, |a, b| a - (b-0.5).powi(2))]
    }
}

fn main() {
    let mut population = PopulationMinimum::<IndividualMinimum>::new_from_shape(10, 10);
    population.advance_epoch(100, "rand", 1, 0.5, 0.5);
    population.show_best_individual();
}