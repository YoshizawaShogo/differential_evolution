use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    str::FromStr,
};

use rand::{rngs::StdRng, Rng};

/// genes --(identificate)--> features --(evaluate)--> evaluations
pub trait Minimum {
    /// intermediate representation
    type Feature: ToString + Display + Debug + PartialEq + FromStr + Clone;

    fn new() -> Self;
    fn set_genes(&mut self, genes: Vec<f64>);
    fn get_genes(&self) -> &Vec<f64>;
    fn set_features(&mut self, features: Vec<Self::Feature>);
    fn get_features(&self) -> &Vec<Self::Feature>;
    fn set_evaluations(&mut self, evaluations: Vec<f64>);
    fn get_evaluations(&self) -> &Vec<f64>;

    fn identificate(&self) -> Vec<Self::Feature>;
    fn evaluate(&self) -> Vec<f64>;
}

pub trait ExtMinimum: Minimum {
    fn from_genes(gene: Vec<f64>) -> Self;
    fn from_length(length: usize, random_generator: &mut StdRng) -> Self;

    fn is_better_than(&self, another: &Self) -> bool;
    fn cross(&self, another: &Self, own_ratio: f64, random_generator: &mut StdRng) -> Self;
}

impl<I> ExtMinimum for I
where
    I: Minimum,
{
    fn from_genes(gene: Vec<f64>) -> Self {
        let mut indi = Self::new();
        indi.set_genes(gene);
        indi
    }
    fn from_length(length: usize, random_generator: &mut StdRng) -> Self {
        let mut gene = vec![];
        for _ in 0..length {
            gene.push(random_generator.gen());
        }
        Self::from_genes(gene)
    }
    fn is_better_than(&self, another: &Self) -> bool {
        let self_eval = self.get_evaluations();
        let another_eval = another.get_evaluations();
        match self_eval.partial_cmp(another_eval).unwrap() {
            Ordering::Greater | Ordering::Equal => true,
            Ordering::Less => false,
        }
    }
    fn cross(&self, another: &Self, another_ratio: f64, random_generator: &mut StdRng) -> Self {
        debug_assert!(0.0 <= another_ratio && another_ratio < 1.0);

        let gene_len = self.get_genes().len();
        let must_choose_another = random_generator.gen_range(0..gene_len);
        let mut new_genes = Vec::with_capacity(gene_len);

        for i in 0..gene_len {
            let gene: f64 = if random_generator.gen::<f64>() <= another_ratio || i == must_choose_another
            {
                another.get_genes()[i]
            } else {
                self.get_genes()[i]
            };
            new_genes.push(gene);
        }

        let mut individual = Self::from_genes(new_genes);
        individual.set_features(individual.identificate());
        individual
    }
}
