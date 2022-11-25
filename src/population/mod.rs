pub mod individual;

use std::{fmt::Debug, ops::AddAssign};
use num::{Float, NumCast};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use self::individual::*;

pub struct PopulationMinimum<INDIVIDUAL> {
    individuals: Vec<INDIVIDUAL>,
}

trait PopulationMinimumBase<INDIVIDUAL, FLOAT> {
    fn new() -> Self;
    fn get_individuals(&self) -> &Vec<INDIVIDUAL>;
    fn get_individuals_as_mut(&mut self) -> &mut Vec<INDIVIDUAL>;
    fn set_individuals(&mut self, individuals: Vec<INDIVIDUAL>);
}

pub trait PopulationMinimumInterface<INDIVIDUAL, FLOAT> {
    fn new_from_individuals(individuals: Vec<INDIVIDUAL>) -> Self;
    fn new_from_shape(individuals_len: usize, genes_len: usize) -> Self;
    fn show_all(&self);
    fn show_at(&self, index: usize);
    fn show_best_individual(&self);
    fn get_index_best(&self) -> usize;
    fn get_individual_best(&self) -> &INDIVIDUAL;
    fn set_evaluation_function(&mut self, evaluation_function: EvaluationFunctionType<INDIVIDUAL, FLOAT>);
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT);
}

trait PopulationMinimumEvolution<INDIVIDUAL, FLOAT> {
    fn choice_factor_indexes(&self, count: usize) -> Vec<usize>;
    fn de_generate_mutant(&self, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT) -> INDIVIDUAL;
}

impl<INDIVIDUAL, FLOAT> PopulationMinimumBase<INDIVIDUAL, FLOAT> for PopulationMinimum<INDIVIDUAL>
where
    INDIVIDUAL: IndividualMinimumBase<FLOAT>,
    FLOAT: Float,
{
    fn new() -> Self {
        Self { individuals: Vec::new() }
    }

    fn get_individuals(&self) -> &Vec<INDIVIDUAL> {
        &self.individuals
    }

    fn set_individuals(&mut self, individuals: Vec<INDIVIDUAL>) {
        self.individuals = individuals;
    }

    fn get_individuals_as_mut(&mut self) -> &mut Vec<INDIVIDUAL> {
        &mut self.individuals
    }
}

impl<POPULATION, INDIVIDUAL, FLOAT> PopulationMinimumInterface<INDIVIDUAL, FLOAT> for POPULATION
where
    Standard: Distribution<FLOAT>,
    POPULATION: PopulationMinimumBase<INDIVIDUAL, FLOAT>,
    INDIVIDUAL: IndividualMinimumBase<FLOAT> + IndividualMinimumInterface<FLOAT> + IndividualMinimumEvolution<FLOAT>,
    FLOAT: Float + AddAssign + Debug,
{
    fn new_from_individuals(individuals: Vec<INDIVIDUAL>) -> Self{
        let mut population = POPULATION::new();
        population.set_individuals(individuals);
        population
    }

    fn new_from_shape(individuals_len: usize, genes_len: usize) -> Self {
        let mut population = POPULATION::new();
        let mut individuals = Vec::with_capacity(individuals_len);

        for _ in 0..individuals_len {
            individuals.push(INDIVIDUAL::new_from_length(genes_len));
        }
        population.set_individuals(individuals);

        population
    }

    fn get_index_best(&self) -> usize {
        let mut best_index = 0;
        let mut best_value = self.get_individuals()[0].get_evaluation_values();

        for (index, individual) in self.get_individuals().iter().enumerate() {
            if individual.get_evaluation_values() < best_value {
                best_index = index;
                best_value = individual.get_evaluation_values();
            }
        }
        best_index
    }

    fn show_all(&self) {
        for individual in self.get_individuals() {
            individual.show_detail();
        }
    }

    fn show_at(&self, index: usize) {
        self.get_individuals()[index].show_detail();
    }

    fn show_best_individual(&self) {
        self.get_individuals()[self.get_index_best()].show_detail();
    }

    fn get_individual_best(&self) -> &INDIVIDUAL {
        &self.get_individuals()[self.get_index_best()]
    }

    fn set_evaluation_function(&mut self, evaluation_function: EvaluationFunctionType<INDIVIDUAL, FLOAT>) {
        for individual in self.get_individuals_as_mut() {
            individual.set_evaluation_function(evaluation_function)
        }
    }

    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT) {
        for individual in self.get_individuals_as_mut() {
            if individual.get_evaluation_values().len() == 0 {
                individual.set_evaluation_values(individual.evaluate());
            }
        }
        for _ in 0..epoch {
            let mut individuals = Vec::with_capacity(self.get_individuals().len());
            for individual in self.get_individuals() {
                let mutant = self.de_generate_mutant(best_or_rand, difference_vector_count, f_scale);
                let trial = individual.bin_cross(mutant, crossover_rate);
                individuals.push(individual.compete(trial));
            }
            self.set_individuals(individuals);
        }
    }
}

impl<POPULATION, INDIVIDUAL, FLOAT> PopulationMinimumEvolution<INDIVIDUAL, FLOAT> for POPULATION
where
    Standard: Distribution<FLOAT>,
    POPULATION: PopulationMinimumBase<INDIVIDUAL, FLOAT>,
    INDIVIDUAL: IndividualMinimumBase<FLOAT> + IndividualMinimumInterface<FLOAT> + IndividualMinimumEvolution<FLOAT>,
    FLOAT: Float + NumCast + AddAssign + Debug,
{
    fn choice_factor_indexes(&self, count: usize) -> Vec<usize> {
        // 集団全体から個体を選ぶ意味は無いため
        assert!(count < self.get_individuals().len());

        let mut random_generator = rand::thread_rng();
        let mut factor_indexes = Vec::with_capacity(count);

        while factor_indexes.len() != count {
            let index = random_generator.gen_range(0..self.get_individuals().len());
            factor_indexes.push(index);
        }

        factor_indexes
    }
    fn de_generate_mutant(&self, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT) -> INDIVIDUAL
    {
        
        assert!(best_or_rand == "best" || best_or_rand == "rand");
        let mut factor_indexes = self.choice_factor_indexes(1 + 2 * difference_vector_count);

        // bestとrandの両方に対応するための記述
        if best_or_rand == "best" {
            let best_individual_index = self.get_index_best();
            if factor_indexes.contains(&best_individual_index) {
                let duplicated_index = factor_indexes.iter().position(|&r| r == best_individual_index).unwrap();
                factor_indexes.remove(duplicated_index);
                factor_indexes.insert(0, best_individual_index);
            } else {
                factor_indexes.remove(0);
                factor_indexes.insert(0, best_individual_index);
            }
            factor_indexes.insert(0, self.get_index_best());
        }

        
        let genes_len = self.get_individuals()[0].get_genes().len();
        let mut genes = Vec::with_capacity(genes_len);
        let individuals = &self.get_individuals();

        for i in 0..genes_len{
            let mut gene = individuals[factor_indexes[0]].get_genes()[i];
            for j in 0..difference_vector_count {
                let gene1 = individuals[factor_indexes[1 + 2 * j]].get_genes()[i];
                let gene2 = individuals[factor_indexes[1 + 2 * j + 1]].get_genes()[i];

                gene += f_scale * (gene1 - gene2);
            }
            // [0.0, 1.0]がパラメータの範囲なため、超えていた場合は範囲内に収まるように修正する
            if gene > num::cast(1.0).unwrap() {
                gene = num::cast(1.0).unwrap()
            } else if gene < num::cast(0.0).unwrap() {
                gene = num::cast(0.0).unwrap()
            }
            genes.push(gene);
        }
        INDIVIDUAL::new_from_genes(genes)
    }
}