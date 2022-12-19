//! 最小限(Minimum)の機能を実装
//! 
//! 例

use crate::individual;
use crate::populations;
use crate::populations::common::{GetterSetter, EvolutionParts};
type FLOAT = individual::FLOAT;

#[derive(Debug, Clone)]
pub struct Population<INDIVIDUAL> {
    individuals: Vec<INDIVIDUAL>,
}
pub trait Specific<INDIVIDUAL, CONVERTED> {
    fn new() -> Self;
    fn new_from_individuals(individuals: Vec<INDIVIDUAL>) -> Self;
    fn new_from_shape(individuals_len: usize, genes_len: usize) -> Self;
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT);
}

impl<INDIVIDUAL, CONVERTED> GetterSetter<INDIVIDUAL, CONVERTED> for Population<INDIVIDUAL>
{
    fn get_individuals(&self) -> &Vec<INDIVIDUAL> {
        &self.individuals
    }
    fn set_individuals(&mut self, individuals: Vec<INDIVIDUAL>) {
        self.individuals = individuals;
    }
}

impl<INDIVIDUAL, CONVERTED> Specific<INDIVIDUAL, CONVERTED> for Population<INDIVIDUAL>
where
    Self: populations::common::GetterSetter<INDIVIDUAL, CONVERTED>,
    INDIVIDUAL: individual::Base<CONVERTED> + individual::Utility<CONVERTED> + individual::EvolutionParts<CONVERTED> + Clone,
{
    fn new() -> Self {
        Self { individuals: Vec::new() }
    }
    fn new_from_individuals(individuals: Vec<INDIVIDUAL>) -> Self{
        let mut population = Self::new();
        population.set_individuals(individuals);
        population
    }
    fn new_from_shape(individuals_len: usize, genes_len: usize) -> Self {
        let mut population = Self::new();
        let mut individuals = Vec::with_capacity(individuals_len);

        for _ in 0..individuals_len {
            individuals.push(INDIVIDUAL::new_from_length(genes_len));
        }
        population.set_individuals(individuals);
        population
    }
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT) {
        let mut individuals = self.get_individuals().clone();
        for individual in individuals.iter_mut() {
            if individual.get_evaluation_values().len() == 0 {
                individual.convert_and_set();
                individual.evaluate_and_set();
            }
        }
        self.set_individuals(individuals);

        for _ in 0..epoch {
            let mut individuals = Vec::with_capacity(self.get_individuals().len());
            for individual in self.get_individuals() {
                let mutant = self.de_generate_mutant(best_or_rand, difference_vector_count, f_scale);
                let mut trial = individual.bin_cross(mutant, crossover_rate);
                trial.convert_and_set();
                trial.evaluate_and_set();
                let winner = individual.compete(trial);
                individuals.push(winner);
            }
            self.set_individuals(individuals);
        }
    }
}