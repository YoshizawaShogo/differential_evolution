use serde::{Deserialize, Serialize};
use ys_differential_evolution::group;
use ys_differential_evolution::group::*;
use ys_differential_evolution::individual;
use ys_differential_evolution::method::ExtDefaultDE;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Car {
    genes: Vec<f64>,
    features: Vec<f64>,
    evals: Vec<f64>,
}

impl individual::Minimum for Car {
    type Feature = f64;

    fn new() -> Self {
        Self {
            genes: vec![],
            features: vec![],
            evals: vec![],
        }
    }

    fn set_genes(&mut self, genes: Vec<f64>) {
        self.genes = genes;
    }

    fn get_genes(&self) -> &Vec<f64> {
        &self.genes
    }

    fn set_features(&mut self, features: Vec<Self::Feature>) {
        self.features = features
    }

    fn get_features(&self) -> &Vec<Self::Feature> {
        &self.features
    }

    fn set_evaluations(&mut self, evaluations: Vec<f64>) {
        self.evals = evaluations;
    }

    fn get_evaluations(&self) -> &Vec<f64> {
        &self.evals
    }

    fn identificate(&self) -> Vec<Self::Feature> {
        self.genes
            .iter()
            .clone()
            .map(|x| ((*x - 0.5) * 1000.0).round() / 1000.0)
            .collect()
    }

    fn evaluate(&self) -> Vec<f64> {
        vec![self.features.iter().fold(0.0, |a, b| a - b * b)]
    }
}

#[test]
fn sphere() {
    let mut g = group::Group::<Car>::from_shape(10, 10, 0);
    println!("{:#?}", g.get_best().1);
    g.advance_epoch(100, "rand", 1, 0.5, 0.5);
    println!("{:#?}", g.get_best().1);

    // assert!(false);
}