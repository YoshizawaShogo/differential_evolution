use std::fmt::Debug;

use itertools::Itertools;

use crate::group;
use crate::individual;
pub trait ExtDefaultDE<I> {
    fn advance_epoch(
        &mut self,
        epoch: usize,
        best_or_rand: &str,
        difference_vector_count: usize,
        f_scale: f64,
        crossover_rate: f64,
    );
}

impl<I, G> ExtDefaultDE<I> for G
where
    I: individual::ExtMinimum + Clone,
    G: group::BaseDE<I>,
{
    fn advance_epoch(
        &mut self,
        epoch: usize,
        best_or_rand: &str,
        difference_vector_count: usize,
        f_scale: f64,
        crossover_rate: f64,
    ) {
        let mut tmp_individuals = self.get_individuals().clone();
        for individual in tmp_individuals.iter_mut() {
            if individual.get_evaluations().len() == 0 {
                individual.set_features(individual.identificate());
                individual.set_evaluations(vec![]);
            }
            if individual.get_evaluations().len() == 0 {
                individual.set_evaluations(individual.evaluate());
            }
        }
        self.set_individuals(tmp_individuals);

        for _ in 0..epoch {
            let pre_individuals = self.get_individuals().clone();
            let mut next_individuals = Vec::with_capacity(self.get_individuals().len());
            for individual in pre_individuals {
                let mutant = self.de_mutate(best_or_rand, difference_vector_count, f_scale);
                let mut trial = individual.cross(
                    &mutant,
                    crossover_rate,
                    &mut self.borrowed_random_generator(),
                );

                trial.set_evaluations(trial.evaluate());

                let winner = if individual.is_better_than(&trial) {
                    individual
                } else {
                    trial
                };
                next_individuals.push(winner);
            }
            self.set_individuals(next_individuals);
        }
    }
}

pub trait ExtMemoizationDE<I> {
    fn advance_epoch(
        &mut self,
        epoch: usize,
        best_or_rand: &str,
        difference_vector_count: usize,
        f_scale: f64,
        crossover_rate: f64,
    );
}

impl<I, G> ExtMemoizationDE<I> for G
where
    I: individual::ExtMinimum + Clone + Debug,
    G: group::BaseDE<I> + group::ExtMemoization<I>,
{
    fn advance_epoch(
        &mut self,
        epoch: usize,
        best_or_rand: &str,
        difference_vector_count: usize,
        f_scale: f64,
        crossover_rate: f64,
    ) {
        let mut tmp_individuals = self.get_individuals().clone();
        for individual in tmp_individuals.iter_mut() {
            if individual.get_features().len() == 0 {
                individual.set_features(individual.identificate());
                individual.set_evaluations(vec![]);
            }
            if individual.get_evaluations().len() == 0 {
                let key = individual.get_features();
                let key = &key.iter().join(",");
                if self.memo_as_ref().contains_key(key) {
                    individual.set_evaluations(self.memo_as_ref()[key].clone());
                } else {
                    let evaluation = individual.evaluate();
                    self.memo_as_mut().insert(key.clone(), evaluation.clone());
                    individual.set_evaluations(evaluation);
                }
            }
        }
        self.set_individuals(tmp_individuals);

        for _ in 0..epoch {
            let pre_individuals = self.get_individuals().clone();
            let mut next_individuals = Vec::with_capacity(self.get_individuals().len());
            for individual in pre_individuals {
                let mutant = self.de_mutate(best_or_rand, difference_vector_count, f_scale);
                let mut trial = individual.cross(
                    &mutant,
                    crossover_rate,
                    &mut self.borrowed_random_generator(),
                );


                let key = trial.get_features();
                let key = &key.iter().join(",");
                if self.memo_as_ref().contains_key(key) {
                    trial.set_evaluations(self.memo_as_ref()[key].clone());
                } else {
                    let evaluation = trial.evaluate();
                    self.memo_as_mut().insert(key.clone(), evaluation.clone());
                    trial.set_evaluations(evaluation);
                }
                let winner = if individual.is_better_than(&trial) {
                    individual
                } else {
                    trial
                };
                next_individuals.push(winner);
            }
            self.set_individuals(next_individuals);
        }
    }
}
