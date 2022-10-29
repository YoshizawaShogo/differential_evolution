use std::{fmt::Debug};
use num::Float;
use rand::{distributions::Standard, prelude::Distribution, Rng};

// TF: f32 or f64
// TI: individual
// 
// 適応型(Adaptive)に対して、普通(Base)を定義

pub trait IndividualBaseEach<TF> {
    fn new() -> Self;
    fn get_genes(&self) -> &Vec<TF>;
    fn set_genes(&mut self, genes: Vec<TF>);
    /// 評価値が低い個体が良個体であると定義
    fn evaluate(&self) -> Vec<TF>;
}

pub trait IndividualBaseInterface<TF: Float> {
    fn new_from_genes(genes: Vec<TF>) -> Self;
    fn new_from_len(genes_len: usize) -> Self;
    fn show(&self);
}

pub trait IndividualBaseEvolutionParts<TF> {
    fn bin_cross(&self, another: Self, crossover_rate: TF) -> Self;
    fn compete(&self, another: Self) -> Self;
}

impl<TI: IndividualBaseEach<TF> + Debug, TF: Float + Debug> IndividualBaseInterface<TF> for TI
where Standard: Distribution<TF>
{
    fn new_from_genes(genes: Vec<TF>) -> Self {
        let mut individual = TI::new();
        individual.set_genes(genes);
        individual
    }

    fn new_from_len(genes_len: usize) -> Self {
        let mut random_generator = rand::thread_rng();

        let mut individual = TI::new();
        let mut genes = Vec::with_capacity(genes_len);
        for _ in 0..genes_len {
            genes.push(random_generator.gen());
        }
        individual.set_genes(genes);

        individual
    }

    fn show(&self) {
        println!("{:?}", self);
        println!("Evaluation {:?}", self.evaluate());
    }
}

impl<TI: IndividualBaseEach<TF> + IndividualBaseInterface<TF> + Clone, TF: Float> IndividualBaseEvolutionParts<TF> for TI
where Standard: Distribution<TF>
{
    /// 二項交叉
    fn bin_cross(&self, another: Self, crossover_rate: TF) -> Self {
        let mut random_generator = rand::thread_rng();

        // 必ず一つは親個体以外(Another)から遺伝
        let necessary_one = random_generator.gen_range(0..self.get_genes().len());

        let genes_len = self.get_genes().len();
        let mut genes = Vec::with_capacity(genes_len);

        for i in 0..self.get_genes().len() {
            let gene = if random_generator.gen() > crossover_rate || i == necessary_one {
                another.get_genes()[i]
            } else {
                self.get_genes()[i]
            };
            genes.push(gene);
        }

        TI::new_from_genes(genes)
    }

    /// 個体を評価し、比較する
    /// 
    /// 評価値が小さい個体の方が良い個体と定義

    fn compete(&self, another: Self) -> Self {
        let self_values = self.evaluate();
        let another_values = another.evaluate();

        for (self_value, another_value) in self_values.iter().zip(&another_values) {
            if self_value > another_value {
                return another;
            } else if self_value < another_value {
                return self.clone();
            }
        }

        self.clone()
    }
    
}

// TODO: Comment, inlineなどを使った高速化