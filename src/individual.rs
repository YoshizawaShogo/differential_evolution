//! FLOAT: f32 or f64
//! INDIVIDUAL: individual
//! 
//! 適応型(Adaptive)に対して、最小限(Minimum)を定義

use std::{fmt::Debug};
use num::Float;
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[macro_export]
macro_rules! IndividualMinimumImpl {
    () => {
        fn new() -> Self {
            Self { genes: Vec::new(), evaluation_values: Vec::new(), converted_genes: Vec::new()}
        }
        fn set_genes(&mut self, genes: Vec<FLOAT>) {
            self.genes = genes;
        }
        fn get_genes(&self) -> &Vec<FLOAT> {
            &self.genes
        }
        fn get_converted_genes(&self) -> &Vec<FLOAT> {
            &self.converted_genes
        }
        fn get_evaluation_values(&self) -> &Vec<FLOAT> {
            &self.evaluation_values
        }
    };
}

pub trait IndividualMinimumBase<FLOAT>
{
    fn new() -> Self;
    fn set_genes(&mut self, genes: Vec<FLOAT>);
    fn get_genes(&self) -> &Vec<FLOAT>;
    /// evaluateのための前処理
    fn convert(&mut self);
    fn get_converted_genes(&self) -> &Vec<FLOAT>;
    /// 評価値は高い方が良い
    fn evaluate(&mut self);
    fn get_evaluation_values(&self) -> &Vec<FLOAT>;  
}

pub(super) trait IndividualMinimumInterface<FLOAT>
{
    fn new_from_genes(genes: Vec<FLOAT>) -> Self;
    fn new_from_length(genes_length: usize) -> Self;
    fn show_detail(&self);
}

pub(super) trait IndividualMinimumEvolution<FLOAT>
{
    fn bin_cross(&self, another: Self, crossover_rate: FLOAT) -> Self;
    fn compete(&self, another: Self) -> Self;
}

impl<INDIVIDUAL, FLOAT> IndividualMinimumInterface<FLOAT> for INDIVIDUAL
where
    Standard: Distribution<FLOAT>,
    INDIVIDUAL: IndividualMinimumBase<FLOAT>,
    FLOAT: Float + Debug,
{
    fn new_from_genes(genes: Vec<FLOAT>) -> Self {
        let mut individual = INDIVIDUAL::new();
        individual.set_genes(genes);
        individual
    }
    fn new_from_length(genes_length: usize) -> Self {
        let mut random_generator = rand::thread_rng();

        let mut individual = INDIVIDUAL::new();
        let mut genes = Vec::with_capacity(genes_length);
        for _ in 0..genes_length {
            genes.push(random_generator.gen());
        }
        individual.set_genes(genes);
        individual
    }
    fn show_detail(&self) {
        println!("Genes {:?}", self.get_genes());
        println!("Converted {:?}", self.get_converted_genes());
        println!("Evaluation {:?}", self.get_evaluation_values());
    }
}

impl<INDIVIDUAL, FLOAT> IndividualMinimumEvolution<FLOAT> for INDIVIDUAL
where
    Standard: Distribution<FLOAT>,
    INDIVIDUAL: IndividualMinimumBase<FLOAT> + IndividualMinimumInterface<FLOAT> + Clone,
    FLOAT: Float + Debug,
{
    /// 二項交叉
    fn bin_cross(&self, mut another: Self, crossover_rate: FLOAT) -> Self {
        let mut random_generator = rand::thread_rng();

        // 必ず一つは親個体以外(Another)から遺伝
        let necessary_one = random_generator.gen_range(0..self.get_genes().len());

        let genes_len = self.get_genes().len();
        let mut genes = Vec::with_capacity(genes_len);

        for i in 0..genes_len {
            let gene = if random_generator.gen() < crossover_rate || i == necessary_one {
                another.get_genes()[i]
            } else {
                self.get_genes()[i]
            };
            genes.push(gene);
        }
        another.set_genes(genes);
        another.convert();
        another.evaluate();
        another
    }
    /// 個体を評価し、比較する
    /// 評価値が高い個体の方が良い個体と定義
    fn compete(&self, another: Self) -> Self {
        let self_values = self.get_evaluation_values();
        let another_values = another.get_evaluation_values();
        for (self_value, another_value) in self_values.iter().zip(another_values) {
            if self_value < another_value {
                return another;
            } else if self_value > another_value {
                return self.clone();
            }
        }
        self.clone()
    }
    
}