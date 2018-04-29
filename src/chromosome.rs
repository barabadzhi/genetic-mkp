use rand::Rng;
use rsgenetic::pheno::*;

use knapsack::Knapsack;

type FitnessValue = u64;

#[derive(Debug, Clone)]
pub struct Chromosome<'a> {
    pub items: Vec<bool>,
    pub knapsack: &'a Knapsack,
}

impl<'a> Chromosome<'a> {
    pub fn new(knapsack: &'a Knapsack) -> Chromosome {
        Chromosome {
            items: vec![false; knapsack.items.len()],
            knapsack,
        }
    }

    pub fn generate(&mut self) {
        let mut rng = ::rand::thread_rng();

        loop {
            let random_index = rng.gen_range(0, self.items.len());
            let knapsack_item = &self.knapsack.items[random_index];

            if !self.knapsack.will_item_fit(&self, &knapsack_item) {
                break;
            }

            self.items[random_index] = true;
        }
    }
}

impl<'a> Phenotype<FitnessValue> for Chromosome<'a> {
    fn fitness(&self) -> FitnessValue {
        if !self.knapsack.is_chromosome_feasible(&self) {
            FitnessValue::min_value()
        } else {
            self.knapsack.chromosome_total_value(&self)
        }
    }

    fn crossover(&self, other: &Chromosome) -> Chromosome<'a> {
        // uniform crossover
        let mut rng = ::rand::thread_rng();

        let mut chromosome = self.clone();

        for i in 0..self.items.len() {
            if self.items[i] != other.items[i] {
                chromosome.items[i] = rng.gen_weighted_bool(2);
            }
        }

        chromosome
    }

    fn mutate(&self) -> Chromosome<'a> {
        // point mutation
        let mut rng = ::rand::thread_rng();

        let mut chromosome = self.clone();

        // occasional 1st mutation
        if rng.gen_weighted_bool(8) {
            let random_index = rng.gen_range(0, chromosome.items.len());
            chromosome.items[random_index] = !chromosome.items[random_index];
        }

        // occasional 2nd mutation
        if rng.gen_weighted_bool(16) {
            let random_index = rng.gen_range(0, chromosome.items.len());
            chromosome.items[random_index] = !chromosome.items[random_index];
        }

        // repair infeasible chromosomes
        self.knapsack.repair_chromosome(&mut chromosome);

        chromosome
    }
}
