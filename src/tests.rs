#[cfg(test)]
mod tests {
    use colored::*;
    use rand::Rng;

    use knapsack::Knapsack;
    use statistics::Statistics;

    use std::time::{Duration, Instant};

    fn test(file: &str) {
        let knapsack = Knapsack::from(&file);

        let mut rng = ::rand::thread_rng();

        let mut result = Statistics::new();

        let mut random_population_size;
        let mut selection_count;
        let mut iterations_count;
        let mut parents_count;

        let mut p_s_p = (0, 0, 0);

        let start = Instant::now();
        let limit = Duration::from_secs(60);

        while start.elapsed() < limit {
            random_population_size = rng.gen_range(50, 1000);
            selection_count = rng.gen_range(random_population_size / 5, random_population_size / 2);
            iterations_count = rng.gen_range(50, 500);
            parents_count = rng.gen_range(5, (random_population_size - 1) / 4) * 2;

            let new_result = knapsack.run_ga(
                random_population_size,
                selection_count,
                parents_count,
                iterations_count,
            );

            if new_result.total_profit > result.total_profit {
                result = new_result;
                p_s_p = (random_population_size, selection_count, parents_count);
            }
        }

        println!("{}", file.green().bold());
        println!(
            "Population: {}, Selection: {}, Parents: {}",
            p_s_p.0, p_s_p.1, p_s_p.2
        );
        println!("{}{}", "GA".cyan().bold(), result);
    }

    #[test]
    fn class1_01() {
        let file = String::from("instances_shared/class1/100-5-01.txt");
        test(&file);
    }

    #[test]
    fn class1_02() {
        let file = String::from("instances_shared/class1/100-5-02.txt");
        test(&file);
    }

    #[test]
    fn class1_03() {
        let file = String::from("instances_shared/class1/100-5-03.txt");
        test(&file);
    }

    #[test]
    fn class1_04() {
        let file = String::from("instances_shared/class1/100-5-04.txt");
        test(&file);
    }

    #[test]
    fn class1_05() {
        let file = String::from("instances_shared/class1/100-5-05.txt");
        test(&file);
    }

    #[test]
    fn class5_01() {
        let file = String::from("instances_shared/class5/250-10-01.txt");
        test(&file);
    }

    #[test]
    fn class5_02() {
        let file = String::from("instances_shared/class5/250-10-02.txt");
        test(&file);
    }

    #[test]
    fn class5_03() {
        let file = String::from("instances_shared/class5/250-10-03.txt");
        test(&file);
    }

    #[test]
    fn class5_04() {
        let file = String::from("instances_shared/class5/250-10-04.txt");
        test(&file);
    }

    #[test]
    fn class5_05() {
        let file = String::from("instances_shared/class5/250-10-05.txt");
        test(&file);
    }

    #[test]
    fn class9_01() {
        let file = String::from("instances_shared/class9/500-30-01.txt");
        test(&file);
    }

    #[test]
    fn class9_02() {
        let file = String::from("instances_shared/class9/500-30-02.txt");
        test(&file);
    }

    #[test]
    fn class9_03() {
        let file = String::from("instances_shared/class9/500-30-03.txt");
        test(&file);
    }

    #[test]
    fn class9_04() {
        let file = String::from("instances_shared/class9/500-30-04.txt");
        test(&file);
    }

    #[test]
    fn class9_05() {
        let file = String::from("instances_shared/class9/500-30-05.txt");
        test(&file);
    }
}
