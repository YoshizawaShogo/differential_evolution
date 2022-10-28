// この"use"は必須
// This "use" is required
use ys_differential_evolution::population::{individual::*, *};

// 最適化対象とする構造を定義
// Define structure to be optimized
#[derive(Debug, Clone)]
pub struct Sphere {
    genes: Vec<f32>,
}

// 最低限必要な振舞いを定義
// "f32"は"f64"にも変更可能
// Define minimum required behavior
// "f32" can be changed to "f64"
impl IndividualBaseEach<f32> for Sphere
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
        Self { genes: Vec::new() }
    }

    // "evaluate"はそれぞれの問題に対して実装する必要がある
    // 評価値が低い個体が良個体であると定義
    // 今回は"gene"が全て0.5である時が最適解
    // "evaluate" should be implemented for each problem
    // An individual with a low evaluation value is defined as a good individual.
    // This time, the optimal solution is when all "gene" are 0.5
    fn evaluate(&self) -> f32 {
        let mut sum = 0.0;
        for num in self.get_genes().iter() {
            sum += (num-0.5).powi(2);
        }
        sum
    }
}

fn main() {
    let mut population = BasePopulation::<Sphere>::new_from_shape(10, 20);

    println!("# The initial best individual");
    population.show_best_individual();
    
    // 進化
    // Evolve
    population.advance_epoch(100, "rand", 1, 0.4, 0.9);

    println!("# The final best individual");
    population.show_best_individual();
}