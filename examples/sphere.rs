// この"use"は必須
// This "use" is required
use ys_differential_evolution::population::{individual::*, *};

fn main() {
    let mut population = PopulationMinimum::<IndividualMinimum>::new_from_shape(50, 20);
    population.set_evaluation_function(|individual| {
        let a = individual.get_genes();
        let b = a.iter().fold(0.0, |a, b| a + (b-0.5).powi(2));
        vec![b]
    });
    population.advance_epoch(100, "best", 1, 0.5, 0.5);
    population.show_best_individual();
}