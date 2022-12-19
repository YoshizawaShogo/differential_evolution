use ys_differential_evolution::individual;
use ys_differential_evolution::populations;
use ys_differential_evolution::populations::common::*;
use ys_differential_evolution::populations::using_db::Specific;

type FLOAT = individual::FLOAT;
type CONVERTED = FLOAT;

#[derive(Debug, Clone)]
struct Individual
{
    genes: Vec<FLOAT>,
    converted_genes: Vec<CONVERTED>,
    evaluation_values: Vec<FLOAT>,
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
        // [0.0, 1.0] を [0, 100] に変換
        self.get_genes().iter().map(|x| (x * 100.0).round()).collect()
    }
    fn evaluate(&self) -> Vec<FLOAT> {
        // "y = sum( -( ( x - 50 ) ^ 2 ) )"が最大値となるxを求める。
        // 全てのgeneが50の時が最適解
        // std::thread::sleep(std::time::Duration::from_secs(1)); // 仮想負荷
        vec![self.get_converted_genes().iter().map(|x| -((x-50.0).powi(2))).sum()]
    }
}

fn main() {
    let mut population = populations::using_db::Population::<Individual>::new_from_shape(20, 10, "for_sphere.csv");
    population.advance_epoch(100, "rand", 1, 0.5, 0.5);
    population.show_best_individual();
    let db =  population.get_sorted_db();
    let len = db.len();
    for i in 0..10 {
        println!("{}-th best:\t{:?}",i+1, db[len - 1 - i]);
    }
}