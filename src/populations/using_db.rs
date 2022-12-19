//! 最小限(Minimum)の機能を実装
//! 
//! 例

use crate::individual;
use crate::populations;
use crate::populations::common::{GetterSetter, EvolutionParts};
use ys_simple_db::DB;

type FLOAT = individual::FLOAT;

#[derive(Debug, Clone)]
pub struct Population<INDIVIDUAL> {
    individuals: Vec<INDIVIDUAL>,
    db_name: String,
}
pub trait Specific<INDIVIDUAL, CONVERTED> {
    fn new(db_name: &str) -> Self;
    fn new_from_individuals(individuals: Vec<INDIVIDUAL>, db_name: &str) -> Self;
    fn new_from_shape(individuals_len: usize, genes_len: usize, db_name: &str) -> Self;
    fn get_db_name(&self) -> &str;
    // fn convert_and_evaluate_and_db(&mut self, db: &mut DB, individual: &mut INDIVIDUAL);
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT);
    fn get_sorted_db(&self) -> Vec<Vec<String>>;
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
    Self: populations::common::GetterSetter<INDIVIDUAL, CONVERTED> + populations::common::Utility<INDIVIDUAL, CONVERTED>,
    INDIVIDUAL: individual::Base<CONVERTED> + individual::Utility<CONVERTED> + individual::EvolutionParts<CONVERTED> + Clone,
    CONVERTED: ToString
{
    fn new(db_name: &str) -> Self {
        Self { individuals: Vec::new(), db_name: db_name.to_string() }
    }

    fn new_from_individuals(individuals: Vec<INDIVIDUAL>, db_name: &str) -> Self{
        let mut population = Self::new(db_name);
        population.set_individuals(individuals);
        population
    }
    fn new_from_shape(individuals_len: usize, genes_len: usize, db_name: &str) -> Self {
        let mut population = Self::new(db_name);
        let mut individuals = Vec::with_capacity(individuals_len);

        for _ in 0..individuals_len {
            individuals.push(INDIVIDUAL::new_from_length(genes_len));
        }
        population.set_individuals(individuals);

        population
    }
    fn get_db_name(&self) -> &str {
        &self.db_name
    }
    /* trialに対して呼び出すとエラーが発生したため、コメントアウト。二か所でこの処理をしている。*/
    // fn convert_and_evaluate_and_db(&mut self, db: &mut DB, individual: &mut INDIVIDUAL) {
    //     individual.convert_and_set();
    //     if db.contains_key(individual.get_converted_genes()) {
    //         let evaluation_values: Vec<FLOAT> = db.get(individual.get_converted_genes()).unwrap().iter().map(|x| match x.parse::<FLOAT>() {
    //             Ok(x) => x,
    //             Err(_) => panic!()
    //         }).collect();
    //         individual.set_evaluation_values(evaluation_values);
    //     } else {
    //         individual.set_evaluation_values(individual.evaluate());
    //         db.insert(&individual.get_converted_genes(), &individual.get_evaluation_values());
    //         db.to_file(self.get_db_name(), ",");
    //     }
    // }
    fn advance_epoch(&mut self, epoch: usize, best_or_rand: &str, difference_vector_count: usize, f_scale: FLOAT, crossover_rate: FLOAT) {
        // DBっぽいデータ構造をファイルからロードする。
        let mut db = DB::from_file(self.get_db_name(), self.get_individuals()[0].convert().len(), ",");
        
        // 初期個体を一度だけ評価する
        let mut individuals = self.get_individuals().clone();
        for individual in individuals.iter_mut() {
            individual.convert_and_set();
            if db.contains_key(individual.get_converted_genes()) {
                let evaluation_values: Vec<FLOAT> = db.get(individual.get_converted_genes()).unwrap().iter().map(|x| match x.parse::<FLOAT>() {
                    Ok(x) => x,
                    Err(_) => panic!()
                }).collect();
                individual.set_evaluation_values(evaluation_values);
            } else {
                individual.set_evaluation_values(individual.evaluate());
                db.insert(&individual.get_converted_genes(), &individual.get_evaluation_values());
                db.to_file(self.get_db_name(), ",");
            }
        }
        self.set_individuals(individuals);

        // 進化
        for _ in 0..epoch {
            let mut individuals = Vec::with_capacity(self.get_individuals().len());
            for individual in self.get_individuals() {
                let mutant = self.de_generate_mutant(best_or_rand, difference_vector_count, f_scale);
                let mut trial = individual.bin_cross(mutant, crossover_rate);
                trial.convert_and_set();
                if db.contains_key(trial.get_converted_genes()) {
                    let evaluation_values: Vec<FLOAT> = db.get(trial.get_converted_genes()).unwrap().iter().map(|x| match x.parse::<FLOAT>() {
                        Ok(x) => x,
                        Err(_) => panic!()
                    }).collect();
                    trial.set_evaluation_values(evaluation_values);
                } else {
                    trial.set_evaluation_values(trial.evaluate());
                    db.insert(&trial.get_converted_genes(), &trial.get_evaluation_values());
                    db.to_file(self.get_db_name(), ",");
                }
                let winner = individual.compete(trial);
                individuals.push(winner);
            }
            self.set_individuals(individuals);
        }
        db.to_file(self.get_db_name(), ",");
    }

    fn get_sorted_db(&self) -> Vec<Vec<String>>{
        let key_length = self.get_individuals()[0].convert().len();
        let db = DB::from_file(self.get_db_name(), key_length, ",");
        let mut db = db.to_vec();
        db.sort_by(|a, b| {
            let (_ , a_evaluation_values)= a.split_at(key_length);
            let (_ , b_evaluation_values)= b.split_at(key_length);
            let a_evaluation_values: Vec<FLOAT> = a_evaluation_values.iter().map(|str_x| str_x.parse::<FLOAT>().unwrap()).collect();
            let b_evaluation_values: Vec<FLOAT> = b_evaluation_values.iter().map(|str_x| str_x.parse::<FLOAT>().unwrap()).collect();
            if INDIVIDUAL::is_better_than(&a_evaluation_values, &b_evaluation_values) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        db
    }
}