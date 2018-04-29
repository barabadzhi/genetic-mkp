extern crate clap;
extern crate colored;
extern crate rand;
extern crate rsgenetic;

use clap::{App, Arg};
use colored::*;

mod chromosome;
mod item;
mod knapsack;
mod statistics;

use knapsack::Knapsack;

fn main() {
    let (file, random_population_size, selection_count, iterations_count) = read_cmd_arguments();

    let knapsack = Knapsack::from(&file);

    let ga_result = knapsack.run_ga(random_population_size, selection_count, iterations_count);
    println!("{}{}", "GA".cyan().bold(), ga_result);
}

fn read_cmd_arguments() -> (String, usize, usize, u64) {
    let matches = App::new("Genetic MKP")
        .version("0.5.2")
        .author("Bogdan Arabadzhi <bogdan.today@gmail.com>")
        .about("Simple genetic algorithm for multidimensional knapsack problem (MKP)")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets a custom input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("population")
                .short("p")
                .long("population")
                .value_name("NUMBER")
                .help("Sets a random population size")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("selection")
                .short("s")
                .long("selection")
                .value_name("NUMBER")
                .help("Sets number of participants selected for tournaments")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("iterations")
                .short("n")
                .long("iterations")
                .value_name("NUMBER")
                .help("Sets the maximum number of iterations")
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap_or("input.txt").to_string();

    let random_population_size = matches
        .value_of("population")
        .unwrap_or("100")
        .parse::<usize>()
        .unwrap();

    let selection_count = matches
        .value_of("selection")
        .unwrap_or("25")
        .parse::<usize>()
        .unwrap();

    let iterations_count = matches
        .value_of("iterations")
        .unwrap_or("50")
        .parse::<u64>()
        .unwrap();

    (
        input,
        random_population_size,
        selection_count,
        iterations_count,
    )
}
