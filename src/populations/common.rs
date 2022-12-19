//! 共通(common)の機能を実装
//! 
//! 例
use rand::Rng;

use crate::individual;
type FLOAT = individual::FLOAT;

pub trait GetterSetter<INDIVIDUAL, CONVERTED> {
    fn get_individuals(&self) -> &Vec<INDIVIDUAL>;
    fn set_individuals(&mut self, individuals: Vec<INDIVIDUAL>);
}

pub trait Utility<INDIVIDUAL, CONVERTED> {
    fn get_index_best(&self) -> usize;
    fn get_individual_best(&self) -> &INDIVIDUAL;
    fn get_length_of_gene(&self) -> usize;
    fn get_length_of_converted_gene(&self) -> usize;
    fn show_all(&self);
    fn show_at(&self, index: usize);
    fn show_best_individual(&self);
}

pub(crate) trait EvolutionParts<INDIVIDUAL, CONVERTED> {
    fn choice_factor_indexes(&self, count: usize) -> Vec<usize>;
    fn de_generate_mutant(&self, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT) -> INDIVIDUAL;
}

impl<POPULATION, INDIVIDUAL, CONVERTED> Utility<INDIVIDUAL, CONVERTED> for POPULATION
where
    POPULATION: GetterSetter<INDIVIDUAL, CONVERTED>,
    INDIVIDUAL: individual::Base<CONVERTED> + individual::Utility<CONVERTED>,
{
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
    fn get_individual_best(&self) -> &INDIVIDUAL {
        &self.get_individuals()[self.get_index_best()]
    }
    fn get_length_of_gene(&self) -> usize {
        self.get_individuals()[0].get_genes().len()
    }
    fn get_length_of_converted_gene(&self) -> usize {
        self.get_individuals()[0].get_converted_genes().len()
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

}

impl<POPULATION, INDIVIDUAL, CONVERTED> EvolutionParts<INDIVIDUAL, CONVERTED> for POPULATION
where
    POPULATION: GetterSetter<INDIVIDUAL, CONVERTED>,
    INDIVIDUAL: individual::Base<CONVERTED> + individual::Utility<CONVERTED> + individual::EvolutionParts<CONVERTED> + Clone,
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
            if gene > 1.0 {
                gene = 1.0
            } else if gene < 0.0 {
                gene = 0.0
            }
            genes.push(gene);
        }
        INDIVIDUAL::new_from_genes(genes)
    }
}
