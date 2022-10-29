// この"use"は必須
// This "use" is required
use ys_differential_evolution::population::{individual::*, *};

// 最適化対象とする構造を定義
// Define structure to be optimized
#[derive(Debug, Clone)]
pub struct  Item {
    pub value: usize,
    pub weight: usize,
    pub maximum_count: usize,
}

#[derive(Debug, Clone)]
pub struct Knapsack {
    pub genes: Vec<f32>,
    pub items: Vec<Item>,
    pub capacity: usize
}

fn convert(items: &Vec<Item>, genes: &Vec<f32>) -> Vec<usize> {
    let mut converted = Vec::with_capacity(genes.len());
    
    // [0.0, 1.0]の区間を(item.maximum_count + 1)で均等に割り、
    // それぞれの区間を個数に変換する
    for (item, gene) in items.iter().zip(genes) {
        let per_count = 1.0 / (item.maximum_count + 1) as f32;
        let mut count = (gene / per_count) as usize;
        if count > item.maximum_count {
            count = item.maximum_count;
        }
        converted.push(count);
    }

    converted
}

// 最低限必要な振舞いを定義
// "f32"は"f64"にも変更可能だが、intには未対応
// Define minimum required behavior
// "f32" can be changed to "f64", but int is not supported
impl IndividualBaseEach<f32> for Knapsack
{
    /* ここからテンプレート */
    /* the template from here */
    fn get_genes(&self) -> &Vec<f32> {
        &self.genes
    }

    fn set_genes(&mut self, genes: Vec<f32>) {
        self.genes = genes;
    }
    /* ここまではテンプレート */
    /* End of the template */

    // "new"と"evaluate"は最適化対象の問題に応じてカスタマイズする必要がある
    // "new" and "evaluate" should be customized according to the problem to be optimized
    fn new() -> Self {
        Self { 
            genes: Vec::new(),
            items: vec![
                Item { value: 10, weight: 1,  maximum_count: 10000 },
                Item { value: 9,  weight: 2,  maximum_count: 10000 },
                Item { value: 8,  weight: 3,  maximum_count: 10000 },
                Item { value: 7,  weight: 4,  maximum_count: 10000 },
                Item { value: 6,  weight: 5,  maximum_count: 10000 },
                Item { value: 5,  weight: 6,  maximum_count: 10000 },
                Item { value: 4,  weight: 7,  maximum_count: 10000 },
                Item { value: 3,  weight: 8,  maximum_count: 10000 },
                Item { value: 2,  weight: 9,  maximum_count: 10000 },
                Item { value: 1,  weight: 10, maximum_count: 10000 },
            ],
            capacity: 111111
        }
    }

    // "evaluate"はそれぞれの問題に対して実装する必要がある
    // 評価値が低い個体が良個体であると定義
    // 今回は、"gene"に応じた個数の"Item"をKnapsackに入れると定義
    // "evaluate" should be implemented for each problem
    // An individual with a low evaluation value is defined as a good individual.
    // This time, it is defined that the number of "Items" corresponding to "gene" is put into Knapsack 
    fn evaluate(&self) -> Vec<f32> {
        assert_eq!(self.items.len(), self.genes.len());

        let mut sum_value = 0.0;
        let mut sum_weight = 0.0;
        let counts = convert(&self.items, &self.genes);
        for (item, count) in self.items.iter().zip(&counts) {
            sum_value -= (item.value * count) as f32;
            sum_weight += (item.weight * count) as f32;
        }

        let mut evaluation_values = Vec::new();

        // "weight"制約
        // "weight" constraint
        if sum_weight > self.capacity as f32{
            evaluation_values.push(sum_weight);
        } else {
            evaluation_values.push(f32::MIN);
        }

        // "value"
        evaluation_values.push(sum_value);

        evaluation_values
    }
}

fn main() {
    // 今回は、引数"genes_len"は"item"の個数と等しくする必要がある
    // This time, the argument "genes_len" must equal the number of "items"
    let mut population = BasePopulation::<Knapsack>::new_from_shape(100, 10);

    println!("# The initial best individual");
    population.show_best_individual();
    let counts = convert(&population.get_individual_best().items, &population.get_individual_best().genes);
    println!("counts: {:?}", counts);
    
    // 進化
    // Evolve
    population.advance_epoch(1000, "rand", 1, 0.5, 0.5);

    println!("\n# The final best individual");
    population.show_best_individual();
    let counts = convert(&population.get_individual_best().items, &population.get_individual_best().genes);
    println!("counts: {:?}", counts);
}