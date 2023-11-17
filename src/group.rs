use crate::individual;
use itertools::Itertools;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

pub trait Minimum<I>
where
    I: individual::Minimum,
{
    fn new() -> Self;
    fn set_individuals(&mut self, individuals: Vec<I>);
    fn get_individuals(&self) -> &Vec<I>;
    fn set_random_generator(&mut self, random_generator: StdRng);
    fn borrowed_random_generator(&mut self) -> &mut StdRng;
}

type MEMO = HashMap<String, Vec<f64>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group<I>
where
    I: individual::Minimum,
{
    individuals: Vec<I>,
    #[serde(skip)]
    random_generator: Option<StdRng>,
    #[serde(skip)]
    memo: MEMO,
}

impl<I> Minimum<I> for Group<I>
where
    I: individual::Minimum,
{
    fn new() -> Self {
        Self {
            individuals: vec![],
            random_generator: Some(StdRng::seed_from_u64(0)),
            memo: HashMap::new(),
        }
    }
    fn set_individuals(&mut self, individuals: Vec<I>) {
        self.individuals = individuals;
    }
    fn get_individuals(&self) -> &Vec<I> {
        &self.individuals
    }
    fn set_random_generator(&mut self, random_generator: StdRng) {
        self.random_generator = Some(random_generator);
    }
    fn borrowed_random_generator(&mut self) -> &mut StdRng {
        self.random_generator.as_mut().unwrap()
    }
}

pub trait ExtMinimum<I>: Minimum<I>
where
    I: individual::ExtMinimum,
{
    fn set_random_seed(&mut self, random_seed: u64);
    fn from_individuals(individuals: Vec<I>, random_generator: StdRng) -> Self;
    fn from_shape(individuals_len: usize, gene_len: usize, random_seed: u64) -> Self;
    fn get_best(&self) -> (usize, &I);
    fn get_gene_len(&self) -> usize;
}

impl<I, G> ExtMinimum<I> for G
where
    I: individual::ExtMinimum,
    G: Minimum<I>,
{
    fn set_random_seed(&mut self, random_seed: u64) {
        self.set_random_generator(StdRng::seed_from_u64(random_seed));
    }
    fn from_individuals(individuals: Vec<I>, mut random_generator: StdRng) -> Self {
        let mut g = G::new();
        let rg = random_generator.gen();
        g.set_individuals(individuals);
        g.set_random_seed(rg);
        g
    }
    fn from_shape(individuals_len: usize, gene_len: usize, random_seed: u64) -> Self {
        let mut rg = StdRng::seed_from_u64(random_seed);

        let mut individuals = Vec::with_capacity(individuals_len);
        for _ in 0..individuals_len {
            let mut genes = Vec::with_capacity(gene_len);
            for _ in 0..gene_len {
                genes.push(rg.gen());
            }
            individuals.push(I::from_genes(genes));
        }
        G::from_individuals(individuals, rg)
    }
    fn get_best(&self) -> (usize, &I) {
        self.get_individuals()
            .iter()
            .enumerate()
            .max_by(|a, b| {
                let a_eval = a.1.get_evaluations();
                let b_eval = b.1.get_evaluations();
                a_eval.partial_cmp(b_eval).unwrap()
            })
            .unwrap()
    }
    fn get_gene_len(&self) -> usize {
        self.get_individuals()[0].get_genes().len()
    }
}

pub trait ExtSaveGroup<I> {
    fn save_to_json(&self, output_path: &str);
    fn load_from_json(input_path: &str, random_seed: u64) -> Self;
}

impl<I, G> ExtSaveGroup<I> for G
where
    I: individual::ExtMinimum + Serialize + DeserializeOwned,
    G: Minimum<I> + Serialize + DeserializeOwned,
{
    fn save_to_json(&self, output_path: &str) {
        let mut file = File::create(output_path).unwrap();
        let json = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
    fn load_from_json(input_path: &str, random_seed: u64) -> Self {
        let json = read_to_string(input_path).unwrap();
        let mut group: G = serde_json::from_str(&json).unwrap();
        group.set_random_seed(random_seed);
        group
    }
}

pub trait Memoization<I>
where
    I: individual::ExtMinimum,
{
    fn memo_as_mut(&mut self) -> &mut MEMO;
    fn memo_as_ref(&self) -> &MEMO;
}

impl<I> Memoization<I> for Group<I>
where
    I: individual::ExtMinimum,
{
    fn memo_as_mut(&mut self) -> &mut MEMO {
        &mut self.memo
    }
    fn memo_as_ref(&self) -> &MEMO {
        &self.memo
    }
}

pub trait ExtMemoization<I>: ExtMinimum<I> + Memoization<I>
where
    I: individual::ExtMinimum,
{
    fn save_memo_to_csv(&self, output_path: &str);
    fn load_memo_from_csv(&mut self, input_path: &str);
    fn get_sorted_memo(&self) -> Vec<(String, Vec<f64>)>;
}

impl<I, G> ExtMemoization<I> for G
where
    I: individual::ExtMinimum,
    G: Minimum<I> + Memoization<I>,
{
    fn save_memo_to_csv(&self, output_path: &str) {
        let mut file = File::create(output_path).unwrap();
        let mut buf = String::new();
        for (key, value) in self.memo_as_ref().iter() {
            let key = key;
            let value = value.iter().join(",");
            buf += &format!("{}:{}\n", key, value);
        }
        file.write_all(buf.as_bytes()).unwrap();
    }

    fn load_memo_from_csv(&mut self, input_path: &str) {
        for result in BufReader::new(File::open(input_path).unwrap()).lines() {
            let line = result.unwrap();
            let key_value: Vec<&str> = line.split(":").collect();
            assert_eq!(key_value.len(), 2);
            let key = key_value[0].to_string();
            let value = key_value[1]
                .split(",")
                .map(|x| f64::from_str(x).unwrap())
                .collect();
            self.memo_as_mut().insert(key, value);
        }
    }

    fn get_sorted_memo(&self) -> Vec<(String, Vec<f64>)> {
        let mut vec_memo = Vec::from_iter(self.memo_as_ref().clone());
        vec_memo.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        vec_memo.reverse();
        vec_memo
    }
}

pub trait BaseDE<I>: ExtMinimum<I>
where
    I: individual::ExtMinimum,
{
    /// random_generatorを使用するため、&mut selfにしている
    fn choice_factor_indexes(&mut self, count: usize) -> Vec<usize>;
    /// random_generatorを使用するため、&mut selfにしている
    fn de_mutate(&mut self, best_or_rand: &str, difference_vector_count: usize, f_scale: f64) -> I;
}

impl<I, G> BaseDE<I> for G
where
    I: individual::ExtMinimum + Clone,
    G: Minimum<I>,
{
    fn choice_factor_indexes(&mut self, count: usize) -> Vec<usize> {
        // 集団全体から全個体を選ぶ意味は無いため
        assert!(count < self.get_individuals().len());

        let mut choiced = HashSet::with_capacity(count);
        let mut factor_indexes = Vec::with_capacity(count);
        while choiced.len() != count {
            let len = self.get_individuals().len();
            let index = self.borrowed_random_generator().gen_range(0..len);
            if choiced.insert(index) {
                factor_indexes.push(index);
            }
        }
        factor_indexes
    }

    fn de_mutate(&mut self, best_or_rand: &str, difference_vector_count: usize, f_scale: f64) -> I {
        assert!(best_or_rand == "best" || best_or_rand == "rand");
        let mut factor_indexes = self.choice_factor_indexes(1 + 2 * difference_vector_count);

        // bestとrandの両方に対応するための記述
        if best_or_rand == "best" {
            let best_individual_index = self.get_best().0;
            if factor_indexes.contains(&best_individual_index) {
                let duplicated_index = factor_indexes
                    .iter()
                    .position(|&r| r == best_individual_index)
                    .unwrap();
                factor_indexes.remove(duplicated_index);
            } else {
                factor_indexes.remove(0);
            }
            factor_indexes.insert(0, best_individual_index);
        }

        let genes_len = self.get_gene_len();
        let mut genes = Vec::with_capacity(genes_len);
        let individuals = &self.get_individuals();

        for i in 0..genes_len {
            let mut gene = individuals[factor_indexes[0]].get_genes()[i];
            for j in 0..difference_vector_count {
                let gene1 = individuals[factor_indexes[1 + 2 * j]].get_genes()[i];
                let gene2 = individuals[factor_indexes[1 + 2 * j + 1]].get_genes()[i];

                gene += f_scale * (gene1 - gene2);
            }
            // [0.0, 1.0]がパラメータの範囲なため、超えていた場合は範囲内に収まるように修正する
            if gene > 1.0 {
                gene = 1.0
            } else if gene < 0.0 {
                gene = 0.0
            }
            genes.push(gene);
        }
        I::from_genes(genes)
    }
}
