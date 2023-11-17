use serde::{Deserialize, Serialize};
use ys_differential_evolution::group;
use ys_differential_evolution::group::*;
use ys_differential_evolution::individual;
use ys_differential_evolution::method::ExtMemoizationDE;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    value: u64,
    weight: u64,
    stock: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Knapsack {
    capacity: u64,
    items: Vec<Item>,
}

impl Knapsack {
    fn new() -> Self{
        Self { 
            capacity: 11000,
            items: vec![
                Item { value: 10, weight: 1,  stock: 10000 },
                Item { value: 9,  weight: 2,  stock: 10000 },
                Item { value: 8,  weight: 3,  stock: 10000 },
                Item { value: 7,  weight: 4,  stock: 10000 },
                Item { value: 6,  weight: 5,  stock: 10000 },
                Item { value: 5,  weight: 6,  stock: 10000 },
                Item { value: 4,  weight: 7,  stock: 10000 },
                Item { value: 3,  weight: 8,  stock: 10000 },
                Item { value: 2,  weight: 9,  stock: 10000 },
                Item { value: 1,  weight: 10, stock: 10000 },
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Individual {
    pub genes: Vec<f64>,
    pub features: Vec<u64>,
    pub evaluations: Vec<f64>,
}

impl individual::Minimum for Individual {
    type Feature = u64;

    fn new() -> Self {
        Individual { genes: vec![], features: vec![], evaluations: vec![] }
    }

    fn set_genes(&mut self, genes: Vec<f64>) {
        self.genes = genes
    }

    fn get_genes(&self) -> &Vec<f64> {
        &self.genes
    }

    fn set_features(&mut self, features: Vec<Self::Feature>) {
        self.features = features;
    }

    fn get_features(&self) -> &Vec<Self::Feature> {
        &self.features
    }

    fn set_evaluations(&mut self, evaluations: Vec<f64>) {
        self.evaluations = evaluations;
    }

    fn get_evaluations(&self) -> &Vec<f64> {
        &self.evaluations
    }

    fn identificate(&self) -> Vec<Self::Feature> {
        let knapsack = Knapsack::new();
        let len = self.get_genes().len();
        let mut features = Vec::with_capacity(len);
        for i in 0..len {
            let gene = self.get_genes()[i];
            let stock = knapsack.items[i].stock;
            features.push((gene * stock as f64) as Self::Feature);
        }
        features
    }

    fn evaluate(&self) -> Vec<f64> {
        let knapsack = Knapsack::new();
        let capacity = knapsack.capacity;
        let len = self.get_features().len();

        let mut weight_sum = 0.0;
        let mut value_sum = 0.0;

        for i in 0..len {
            let count = self.get_features()[i];
            let weight = knapsack.items[i].weight;
            let value = knapsack.items[i].value;

            weight_sum += (weight * count) as f64;
            value_sum += (value * count) as f64;
        }

        let mut evals = vec![];
        if weight_sum <= capacity as f64{
            evals.push(f64::INFINITY);
        } else {
            evals.push(-weight_sum);
        }
        evals.push(value_sum);
        evals
    }
}

#[test]
fn knapsack() {
    let knapsack = Knapsack::new();
    println!("{:#?}", knapsack);

    let kind_of_item = knapsack.items.len();
    let mut g = group::Group::<Individual>::from_shape(10, kind_of_item, 0);
    g.advance_epoch(100, "rand", 1, 0.8, 0.8);
    g.advance_epoch(100, "rand", 1, 0.5, 0.5);
    g.advance_epoch(5, "best", 1, 0.8, 0.8);
    g.advance_epoch(5, "best", 1, 0.3, 0.3);
    println!("{:#?}", g.get_best().1);
}
