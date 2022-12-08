//! FLOAT: f32 or f64
//! INDIVIDUAL: individual
//! 
//! 適応型(Adaptive)に対して、最小限(Minimum)を定義

use std::{fmt::Debug, ops::AddAssign, str::{self, FromStr}};
use num::{Float, NumCast};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use ys_simple_db::DB;

use crate::individual::minimum::*;

pub struct PopulationUsingDB<INDIVIDUAL> {
    individuals: Vec<INDIVIDUAL>,
    db_name: String,
}


trait PopulationUsingDBBase<INDIVIDUAL, FLOAT> {
    fn new(db_name: &str) -> Self;
    fn get_individuals(&self) -> &Vec<INDIVIDUAL>;
    fn get_individuals_as_mut(&mut self) -> &mut Vec<INDIVIDUAL>;
    fn set_individuals(&mut self, individuals: Vec<INDIVIDUAL>);
    fn get_db_name(&self) -> &str;
}

pub trait PopulationUsingDBInterface<INDIVIDUAL, FLOAT> {
    fn new_from_individuals(individuals: Vec<INDIVIDUAL>, db_name: &str) -> Self;
    fn new_from_shape(individuals_len: usize, genes_len: usize, db_name: &str) -> Self;
    fn show_all(&self);
    fn show_at(&self, index: usize);
    fn show_best_individual(&self);
    fn get_index_best(&self) -> usize;
    fn get_individual_best(&self) -> &INDIVIDUAL;
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT);
}

trait PopulationUsingDBEvolution<INDIVIDUAL, FLOAT> {
    fn choice_factor_indexes(&self, count: usize) -> Vec<usize>;
    fn de_generate_mutant(&self, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT) -> INDIVIDUAL;
}

impl<INDIVIDUAL, FLOAT> PopulationUsingDBBase<INDIVIDUAL, FLOAT> for PopulationUsingDB<INDIVIDUAL>
where
    INDIVIDUAL: IndividualMinimumBase<FLOAT>,
    FLOAT: Float,
{
    fn new(db_name: &str) -> Self {
        Self { individuals: Vec::new(), db_name: db_name.to_string() }
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

    fn get_db_name(&self) -> &str {
        &self.db_name
    }
}

impl<POPULATION, INDIVIDUAL, FLOAT> PopulationUsingDBInterface<INDIVIDUAL, FLOAT> for POPULATION
where
    Standard: Distribution<FLOAT>,
    POPULATION: PopulationUsingDBBase<INDIVIDUAL, FLOAT>,
    INDIVIDUAL: IndividualMinimumBase<FLOAT> + IndividualMinimumInterface<FLOAT> + IndividualMinimumEvolution<FLOAT>,
    FLOAT: Float + AddAssign + Debug + ToString + FromStr,
{
    fn new_from_individuals(individuals: Vec<INDIVIDUAL>, db_name: &str) -> Self{
        let mut population = POPULATION::new(db_name);
        population.set_individuals(individuals);
        population
    }

    fn new_from_shape(individuals_len: usize, genes_len: usize, db_name: &str) -> Self {
        let mut population = POPULATION::new(db_name);
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

    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT) {
        for individual in self.get_individuals_as_mut() {
            if individual.get_evaluation_values().len() == 0 {
                individual.convert();
                individual.evaluate();
            }
        }
        let mut db = DB::from_file(self.get_db_name(), self.get_individuals()[0].get_converted_genes().len(), ",");
        for individual in self.get_individuals_as_mut() {
            db.insert(&individual.get_converted_genes(), &individual.get_evaluation_values());
        }
        for _ in 0..epoch {
            let mut individuals = Vec::with_capacity(self.get_individuals().len());
            for individual in self.get_individuals() {
                let mutant = self.de_generate_mutant(best_or_rand, difference_vector_count, f_scale);
                let mut trial = individual.bin_cross(mutant, crossover_rate);
                trial.convert();
                let converted_genes = trial.get_converted_genes();
                if db.contains_key(converted_genes) {
                    let evaluation_values: Vec<FLOAT> = db.get(converted_genes).unwrap().iter().map(|x| match x.parse::<FLOAT>() {
                        Ok(x) => x,
                        Err(_) => panic!()
                    }).collect();
                    trial.set_evaluation_values(evaluation_values);
                } else {
                    trial.evaluate();
                }
                let winner = individual.compete(trial);
                db.insert(&winner.get_converted_genes(), &winner.get_evaluation_values());
                individuals.push(winner);
            }
            self.set_individuals(individuals);
        }
        db.to_file(self.get_db_name(), ",");
    }
}

impl<POPULATION, INDIVIDUAL, FLOAT> PopulationUsingDBEvolution<INDIVIDUAL, FLOAT> for POPULATION
where
    Standard: Distribution<FLOAT>,
    POPULATION: PopulationUsingDBBase<INDIVIDUAL, FLOAT>,
    INDIVIDUAL: IndividualMinimumBase<FLOAT> + IndividualMinimumInterface<FLOAT> + IndividualMinimumEvolution<FLOAT>,
    FLOAT: Float + NumCast + AddAssign + Debug + ToString + FromStr,
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