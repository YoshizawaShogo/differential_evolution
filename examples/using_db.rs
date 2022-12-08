// この"use"は必須
// This "use" is required
use ys_differential_evolution::{*, population::using_db::*, individual::minimum::*};
type FLOAT = f32;

#[derive(Clone)]
struct IndividualMinimum
{
    genes: Vec<FLOAT>,
    converted_genes: Vec<FLOAT>,
    evaluation_values: Vec<FLOAT>,
}

impl IndividualMinimumBase<FLOAT> for IndividualMinimum
{
    IndividualMinimumImpl!(); // テンプレート
    fn convert(&mut self) {
        
        let knapsack = Knapsack::new();
        let mut converted = Vec::with_capacity(self.get_genes().len());
        
        // [0.0, 1.0]の区間を(item.stock + 1)で均等に割り、
        // それぞれの区間を個数に変換する
        for (item, gene) in knapsack.items.iter().zip(self.get_genes()) {
            let per_count = 1.0 / (item.stock + 1) as FLOAT;
            let mut count = (gene / per_count) as usize;
            if count > item.stock {
                count = item.stock;
            }
            converted.push(count as FLOAT);
        }
        self.converted_genes = converted;
    }
    fn evaluate(&mut self) {
        let knapsack = Knapsack::new();
        // 今回は、引数"genes_len"は"item"の個数と等しくする必要がある
        // This time, the argument "genes_len" must equal the number of "items"
        assert_eq!(knapsack.items.len(), self.get_genes().len());
        let mut sum_value = 0.0;
        let mut sum_weight = 0.0;
        for (item, count) in knapsack.items.iter().zip(&self.converted_genes) {
            sum_value += item.value as FLOAT * count;
            sum_weight += item.weight as FLOAT * count;
        }
        let mut evaluation_values = Vec::new();

        // "weight"制約
        // "weight" constraint
        if sum_weight > knapsack.capacity as FLOAT{
            evaluation_values.push(-sum_weight);
        } else {
            evaluation_values.push(FLOAT::MAX);
        }
        // "value"
        evaluation_values.push(sum_value);
        self.evaluation_values = evaluation_values;
    }
}

// 最適化対象とする構造を定義
// Define structure to be optimized
struct  Item {
    value: usize,
    weight: usize,
    stock: usize,
}

struct Knapsack {
    items: Vec<Item>,
    capacity: usize
}

impl Knapsack {
    fn new() -> Self {
        Self { 
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
            capacity: 111111
        }
    }
}

fn main() {
    let mut population = PopulationUsingDB::<IndividualMinimum>::new_from_shape(20, 10, "test.db");
    population.advance_epoch(500, "rand", 1, 0.5, 0.7);
    population.advance_epoch(3, "best", 1, 0.2, 0.9);
    population.show_best_individual();
}