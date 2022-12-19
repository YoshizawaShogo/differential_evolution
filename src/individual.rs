//! 最小限(Minimum)の機能を実装
//! 
//! 例
//! use a::individuals::minimum;
//! type FLOAT = minimum::FLOAT;
//! #[derive(Clone)]
//! struct Individual
//! {
//!     genes: Vec<FLOAT>,
//!     converted_genes: Vec<FLOAT>,
//!     evaluation_values: Vec<FLOAT>,
//! }

use std::fmt::Debug;
use rand::Rng;

pub type FLOAT = f64;

pub trait Base<CONVERTED>
{
    fn new() -> Self;
    fn set_genes(&mut self, genes: Vec<FLOAT>);
    fn get_genes(&self) -> &Vec<FLOAT>;
    fn set_converted_genes(&mut self, converted_genes: Vec<CONVERTED>);
    fn get_converted_genes(&self) -> &Vec<CONVERTED>;
    fn set_evaluation_values(&mut self, evaluation_values: Vec<FLOAT>);
    fn get_evaluation_values(&self) -> &Vec<FLOAT>; 
    /// evaluateのための前処理
    fn convert(&self) -> Vec<CONVERTED>;
    /// 評価値は高ければ高いほど良い、と定義
    fn evaluate(&self) -> Vec<FLOAT>;
}

pub trait Utility<CONVERTED> // 形を揃えるためだけに、"CONVERTED"を使用。
{
    fn new_from_genes(genes: Vec<FLOAT>) -> Self;
    fn new_from_length(genes_length: usize) -> Self;
    fn convert_and_set(&mut self);
    fn evaluate_and_set(&mut self);
    fn show_detail(&self);
}

pub trait EvolutionParts<CONVERTED> // 形を揃えるためだけに、"CONVERTED"を使用。
{
    fn bin_cross(&self, another: Self, crossover_rate: FLOAT) -> Self;
    fn is_better_than(value_1: &Vec<FLOAT>, value_2: &Vec<FLOAT>) -> bool;
    fn compete(&self, another: Self) -> Self;
}

impl<INDIVIDUAL, CONVERTED> Utility<CONVERTED> for INDIVIDUAL
where
    INDIVIDUAL: Base<CONVERTED>,
    CONVERTED: Debug
{
    fn new_from_genes(genes: Vec<FLOAT>) -> Self {
        let mut individual = Self::new();
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
    fn convert_and_set(&mut self) {
        self.set_converted_genes(self.convert());
    }
    fn evaluate_and_set(&mut self) {
        self.set_evaluation_values(self.evaluate());
    }
    fn show_detail(&self) {
        println!("Genes {:?}", self.get_genes());
        println!("Converted {:?}", self.get_converted_genes());
        println!("Evaluation {:?}", self.get_evaluation_values());
    }
}

impl<INDIVIDUAL, CONVERTED> EvolutionParts<CONVERTED> for INDIVIDUAL
where
    INDIVIDUAL: Base<CONVERTED> + Clone
{
    /// 二項交叉
    fn bin_cross(&self, mut another: Self, crossover_rate: FLOAT) -> Self {
        let mut random_generator = rand::thread_rng();

        // 必ず一つは親個体以外(Another)から遺伝
        let necessary_one = random_generator.gen_range(0..self.get_genes().len());

        let genes_len = self.get_genes().len();
        let mut genes = Vec::with_capacity(genes_len);

        for i in 0..genes_len {
            let gene = if random_generator.gen::<FLOAT>() < crossover_rate || i == necessary_one {
                another.get_genes()[i]
            } else {
                self.get_genes()[i]
            };
            genes.push(gene);
        }
        another.set_genes(genes);
        another
    }
    /// 個体を評価し、比較する
    /// 評価値が高い個体の方が良い個体と定義
    fn is_better_than(value_1: &Vec<FLOAT>, value_2: &Vec<FLOAT>) -> bool {
        for (value_1, value_2) in value_1.iter().zip(value_2) {
            if value_1 < value_2 {
                return false;
            } else if value_1 > value_2 {
                return true;
            }
        }
        true // すべてが等しい場合、とりあえずtrue
    }
    fn compete(&self, another: Self) -> Self {
        let self_values = self.get_evaluation_values();
        let another_values = another.get_evaluation_values();
        if Self::is_better_than(&self_values, &another_values) {
            self.clone()
        } else {
            another
        }
    }
}