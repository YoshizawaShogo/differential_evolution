pub mod individual;

use std::{fmt::Debug, ops::AddAssign};
use num::{Float, NumCast};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use self::individual::*;

// TF: f32 or f64
// TI: individual
// TP: population
// 
// 適応型(Adaptive)に対して、普通(Base)を定義

#[derive(Debug, Clone)]
pub struct BasePopulation<TI> {
    individuals: Vec<TI>,
}

pub trait PopulationBaseEach<TI, TF> {
    fn new() -> Self;
    fn get_individuals(&self) -> &Vec<TI>;
    fn set_individuals(&mut self, individuals: Vec<TI>);
}

pub trait PopulationBaseInterface<TI, TF> {
    fn new_from_individuals(individuals: Vec<TI>) -> Self;
    fn new_from_shape(individuals_len: usize, genes_len: usize) -> Self;
    fn show_all(&self);
    fn show_at(&self, index: usize);
    fn show_best_individual(&self);
    fn get_index_best(&self) -> usize;
    fn get_individual_best(&self) -> &TI;
}

trait PopulationBaseEvolutionParts<TI, TF> {
    fn choice_factor_indexes(&self, count: usize) -> Vec<usize>;
    fn de_generate_mutant(&self, best_or_rand: &str, difference_vector_count: usize, f_scale: TF) -> TI;
}

pub trait PopulationBaseEvolution<TI, TF> {
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: TF, crossover_rate: TF);
}

impl<TI: IndividualBaseEach<TF>, TF: Float> PopulationBaseEach<TI, TF> for BasePopulation<TI> {
    fn new() -> Self {
        Self { individuals: Vec::new() }
    }

    fn get_individuals(&self) -> &Vec<TI> {
        &self.individuals
    }

    fn set_individuals(&mut self, individuals: Vec<TI>) {
        self.individuals = individuals;
    }
}

impl<TP: PopulationBaseEach<TI, TF>, 
     TI: IndividualBaseEach<TF> + IndividualBaseInterface<TF>, 
     TF: Float>
     PopulationBaseInterface<TI, TF> for TP
{
    fn new_from_individuals(individuals: Vec<TI>) -> Self{
        let mut population = TP::new();
        population.set_individuals(individuals);
        population
    }

    fn new_from_shape(individuals_len: usize, genes_len: usize) -> Self {
        let mut population = TP::new();
        let mut individuals = Vec::with_capacity(individuals_len);

        for _ in 0..individuals_len {
            individuals.push(TI::new_from_len(genes_len));
        }
        population.set_individuals(individuals);

        population
    }

    fn get_index_best(&self) -> usize {
        let mut best_index: usize = 0;
        let mut best_value = self.get_individuals()[0].evaluate();

        for (index, individual) in self.get_individuals().iter().enumerate() {
            if individual.evaluate() < best_value {
                best_index = index;
                best_value = individual.evaluate();
            }
        }
        best_index
    }

    fn show_all(&self) {
        for individual in self.get_individuals() {
            individual.show();
        }
    }

    fn show_at(&self, index: usize) {
        self.get_individuals()[index].show();
    }

    fn show_best_individual(&self) {
        self.get_individuals()[self.get_index_best()].show();
    }

    fn get_individual_best(&self) -> &TI {
        &self.get_individuals()[self.get_index_best()]
    }
}

impl<TP: PopulationBaseEach<TI, TF> + PopulationBaseInterface<TI, TF>, 
     TI: IndividualBaseEach<TF> + IndividualBaseInterface<TF>, 
     TF: Float + NumCast + AddAssign + Debug>
     PopulationBaseEvolutionParts<TI, TF> for TP
where Standard: Distribution<TF>
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
    fn de_generate_mutant(&self, best_or_rand: &str, difference_vector_count: usize, f_scale: TF) -> TI
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

        TI::new_from_genes(genes)

    }
}



impl<TP: PopulationBaseEach<TI, TF> + PopulationBaseEvolutionParts<TI, TF>, 
     TI: IndividualBaseEach<TF> + IndividualBaseInterface<TF> + IndividualBaseEvolutionParts<TF>, 
     TF: Float>
     PopulationBaseEvolution<TI, TF> for TP
{
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: TF, crossover_rate: TF) {
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