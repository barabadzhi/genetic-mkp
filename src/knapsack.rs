use rsgenetic::sim::{*, select::TournamentSelector, seq::Simulator};
// use rsgenetic::sim::{*, select::UnstableMaximizeSelector, seq::Simulator};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

use chromosome::Chromosome;
use item::Item;
use statistics::Statistics;

#[derive(Debug)]
pub struct Knapsack {
    pub capacity: Vec<u64>,
    pub items: Vec<Item>,
}

impl Knapsack {
    fn new() -> Knapsack {
        Knapsack {
            capacity: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn from(file: &str) -> Knapsack {
        let file = File::open(file).expect("Input file is not specified");
        let reader = BufReader::new(file);

        let mut m = 0;
        let mut values = Vec::new();
        let mut weights = Vec::new();

        let mut knapsack = Knapsack::new();

        for (line_number, contents) in reader.lines().enumerate() {
            let mut contents: Vec<u64> = contents
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            match line_number {
                0 => {
                    // _n m q opt
                    knapsack.items = Vec::with_capacity(contents[1] as usize);

                    m = contents[1] as usize + 1;
                }
                1 => {
                    // a line with the n obj. func. coefficients
                    values = contents;
                }
                _ if m >= line_number => {
                    // a line for each m; n coefficients for <= constraints
                    weights.push(contents);
                }
                _ => {
                    // a line with rhs of <= constraints
                    knapsack.capacity = contents;
                }
            }
        }

        for (index, value) in values.into_iter().enumerate() {
            let mut item_weight = Vec::with_capacity(weights.len());

            for weight in &weights {
                item_weight.push(weight[index]);
            }

            knapsack.items.push(Item {
                id: index as u64 + 1,
                value,
                weight: item_weight,
            });
        }

        knapsack
    }

    pub fn chromosome_total_value(&self, chromosome: &Chromosome) -> u64 {
        self.items
            .iter()
            .zip(chromosome.items.iter())
            .map(|(a, b)| {
                if *b {
                    return a.value;
                }

                0
            })
            .sum::<u64>()
    }

    fn capacity_left(&self, chromosome: &Chromosome) -> Vec<i64> {
        let mut capacity_left: Vec<i64> = self.capacity.iter().map(|x| *x as i64).collect();

        for (index, value) in chromosome.items.iter().enumerate() {
            if *value {
                self.items[index]
                    .weight
                    .iter()
                    .enumerate()
                    .for_each(|(i, x)| capacity_left[i] -= *x as i64);
            }
        }

        capacity_left
    }

    fn compare_items(&self, first: &Item, second: &Item, chromosome: &Chromosome) -> Ordering {
        let capacity_left = self.capacity_left(chromosome);

        let first_profit = first.value as f64
            / first
                .weight
                .iter()
                .zip(capacity_left.iter())
                .map(|(weight, left)| *weight as f64 / *left as f64)
                .sum::<f64>();

        let second_profit = second.value as f64
            / second
                .weight
                .iter()
                .zip(capacity_left.iter())
                .map(|(weight, left)| *weight as f64 / *left as f64)
                .sum::<f64>();

        first_profit
            .partial_cmp(&second_profit)
            .unwrap_or(Ordering::Equal)
    }

    pub fn is_chromosome_feasible(&self, chromosome: &Chromosome) -> bool {
        self.capacity_left(chromosome).iter().all(|x| *x >= 0)
    }

    pub fn will_item_fit(&self, chromosome: &Chromosome, item: &Item) -> bool {
        let capacity_left = self.capacity_left(chromosome);

        capacity_left
            .iter()
            .zip(item.weight.iter())
            .all(|(a, b)| *a >= *b as i64)
    }

    pub fn repair_chromosome(&self, chromosome: &mut Chromosome) {
        let mut items = self.items.clone();

        while !self.is_chromosome_feasible(chromosome) {
            items.sort_unstable_by(|a, b| self.compare_items(b, a, &chromosome));

            // drop less interesting item
            chromosome.items[(items.pop().unwrap().id - 1) as usize] = false;
        }
    }

    fn greedy_chromosome(&self) -> Chromosome {
        let mut chromosome = Chromosome::new(&self);
        let mut items = self.items.clone();

        loop {
            items.sort_unstable_by(|a, b| self.compare_items(b, a, &chromosome));

            if !self.will_item_fit(&chromosome, &items[0]) {
                break;
            }

            chromosome.items[(items.remove(0).id - 1) as usize] = true;
        }

        chromosome
    }

    fn generate_population(&self, random_population_size: usize) -> Vec<Chromosome> {
        let mut population = Vec::with_capacity(random_population_size);

        let greedy_chromosome = self.greedy_chromosome();

        population.push(greedy_chromosome);

        for _ in 0..random_population_size - 1 {
            let mut random_chromosome = Chromosome::new(&self);

            random_chromosome.generate();

            // add random feasible chromosome
            population.push(random_chromosome);
        }

        population
    }

    pub fn run_ga(
        &self,
        random_population_size: usize,
        selection_count: usize,
        iterations_count: u64,
    ) -> Statistics {
        let mut population = self.generate_population(random_population_size);

        let mut simulator = Simulator::builder(&mut population)
            .set_selector(Box::new(
                TournamentSelector::new_checked(16, selection_count).unwrap(),
                // UnstableMaximizeSelector::new(random_population_size / 10),
            ))
            .build();

        let mut best_individual = self.greedy_chromosome();

        for _ in 0..iterations_count {
            simulator.checked_step();

            let chromosome = simulator.get().unwrap();

            if self.chromosome_total_value(&best_individual)
                < self.chromosome_total_value(&chromosome)
            {
                best_individual = chromosome.clone();
            }

            // println!(
            //     "{} {:?}",
            //     self.chromosome_total_value(&chromosome),
            //     self.capacity_left(&chromosome)
            // );
        }

        let mut result = Statistics::new();

        result.duration = simulator.time().unwrap();
        result.iterations = simulator.iterations();

        let mut utilization = vec![0; self.capacity.len()];

        for i in 0..self.items.len() {
            if best_individual.items[i] {
                result.picked_items.push(self.items[i].id);
                result.total_profit += self.items[i].value;

                for (index, weight) in self.items[i].weight.iter().enumerate() {
                    utilization[index] += weight;
                }
            }
        }

        result.picked_items.sort();

        for (index, capacity) in self.capacity.iter().enumerate() {
            if *capacity < utilization[index] {
                panic!(
                    "\nViolated knapsack capacity!\ncapacity:    {:?}\nutilization: {:?}",
                    self.capacity, utilization
                );
            }

            result.utilization.push(format!(
                "{:.2}%",
                (utilization[index] as f64 / *capacity as f64) * 100_f64
            ));
        }

        result
    }
}
