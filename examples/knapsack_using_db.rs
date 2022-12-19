use ys_differential_evolution::individual;
use ys_differential_evolution::populations;
use ys_differential_evolution::populations::common::*;
use ys_differential_evolution::populations::using_db::Specific;

type FLOAT = individual::FLOAT;
type CONVERTED = usize;

#[derive(Clone)]
struct Individual
{
    genes: Vec<FLOAT>,
    converted_genes: Vec<CONVERTED>,
    evaluation_values: Vec<FLOAT>,
}

struct Item {
    value: usize,
    weight: usize,
    stock: usize,
}

struct Knapsack {
    capacity: usize,
    items: Vec<Item>,
}

impl Knapsack {
    fn new() -> Self{
        Self { 
            capacity: 11111,
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

impl individual::Base<CONVERTED> for Individual{
    fn new() -> Self {
        Self { genes: Vec::new(), converted_genes: Vec::new(), evaluation_values: Vec::new() }
    }
    fn set_genes(&mut self, genes: Vec<FLOAT>) {
        self.genes = genes;
    }
    fn get_genes(&self) -> &Vec<FLOAT> {
        &self.genes
    }
    fn set_converted_genes(&mut self, converted_genes: Vec<CONVERTED>) {
        self.converted_genes = converted_genes;
    }
    fn get_converted_genes(&self) -> &Vec<CONVERTED> {
        &self.converted_genes
    }
    fn set_evaluation_values(&mut self, evaluation_values: Vec<FLOAT>) {
        self.evaluation_values = evaluation_values;
    }
    fn get_evaluation_values(&self) -> &Vec<FLOAT> {
        &self.evaluation_values
    }
    fn convert(&self) -> Vec<CONVERTED> {
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
            converted.push(count);
        }
        converted
    }
    fn evaluate(&self) -> Vec<FLOAT> {
        let knapsack = Knapsack::new();
        // 今回は、引数"genes_len"は"item"の個数と等しくする必要がある
        // This time, the argument "genes_len" must equal the number of "items"
        assert_eq!(knapsack.items.len(), self.get_genes().len());
        let mut sum_value = 0;
        let mut sum_weight = 0;
        for (item, count) in knapsack.items.iter().zip(&self.converted_genes) {
            sum_value += item.value * count;
            sum_weight += item.weight * count;
        }
        let mut evaluation_values = Vec::new();

        // "weight"制約
        // "weight" constraint
        if sum_weight > knapsack.capacity{
            evaluation_values.push(- (sum_weight as FLOAT));
        } else {
            evaluation_values.push(FLOAT::MAX);
        }
        // "value"
        evaluation_values.push(sum_value as FLOAT);
        evaluation_values
    }
}


fn main() {
    let mut population = populations::using_db::Population::<Individual>::new_from_shape(20, 10, "for_knapsack.csv");
    population.advance_epoch(1000, "rand", 1, 0.5, 0.5);
    population.show_best_individual();
    let db =  population.get_sorted_db();
    let len = db.len();
    for i in 0..10 {
        println!("{}-th best:\t{:?}",i+1, db[len - 1 - i]);
    }
}