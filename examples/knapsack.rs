// この"use"は必須
// This "use" is required
use ys_differential_evolution::population::{individual::*, *};

fn main() {

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

    fn convert(items: &Vec<Item>, genes: &Vec<f32>) -> Vec<usize> {
        let mut converted = Vec::with_capacity(genes.len());
        
        // [0.0, 1.0]の区間を(item.stock + 1)で均等に割り、
        // それぞれの区間を個数に変換する
        for (item, gene) in items.iter().zip(genes) {
            let per_count = 1.0 / (item.stock + 1) as f32;
            let mut count = (gene / per_count) as usize;
            if count > item.stock {
                count = item.stock;
            }
            converted.push(count);
        }
        converted
    }

    // 今回は、引数"genes_len"は"item"の個数と等しくする必要がある
    // This time, the argument "genes_len" must equal the number of "items"
    let mut population = PopulationMinimum::<IndividualMinimum>::new_from_shape(100, 10);
    population.set_evaluation_function(|individual| {
        
        let knapsack = Knapsack::new();
        assert_eq!(knapsack.items.len(), individual.get_genes().len());

        let mut sum_value = 0.0;
        let mut sum_weight = 0.0;
        let counts = convert(&knapsack.items, &individual.get_genes());
        for (item, count) in knapsack.items.iter().zip(&counts) {
            sum_value -= (item.value * count) as f32;
            sum_weight += (item.weight * count) as f32;
        }

        let mut evaluation_values = Vec::new();

        // "weight"制約
        // "weight" constraint
        if sum_weight > knapsack.capacity as f32{
            evaluation_values.push(sum_weight);
        } else {
            evaluation_values.push(f32::MIN);
        }

        // "value"
        evaluation_values.push(sum_value);

        evaluation_values
    });

    let knapsack = Knapsack::new();
    population.advance_epoch(10000, "rand", 1, 0.5, 0.5);
    population.show_best_individual();
    let counts = convert(&knapsack.items, &population.get_individual_best().get_genes());
    println!("{:?}", counts);
}