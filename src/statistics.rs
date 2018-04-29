use std::fmt;

use colored::*;

#[derive(Default, Debug)]
pub struct Statistics {
    pub total_profit: u64,
    pub picked_items: Vec<u64>,
    pub utilization: Vec<String>,
    pub iterations: u64,
    pub duration: i64,
}

impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            ..Default::default()
        }
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut run_time = String::new();

        if self.duration / 1_000_000_000 > 0 {
            run_time += &format!("{} s", (self.duration / 1_000_000_000).to_string().green());
        }

        if self.duration - (self.duration / 1_000_000_000) > 0 {
            if run_time.is_empty() {
                run_time += &format!("{} ns", self.duration.to_string().green());
            } else {
                run_time += &format!(
                    " {} ns",
                    (self.duration - (self.duration / 1_000_000_000))
                        .to_string()
                        .green()
                );
            }
        }

        writeln!(
            f,
            r#"
    -> Total profit: {}
    -> Picked items ({}): {}
    -> Utilization: {}
    -> Iterations: {}
    -> Duration: {}"#,
            self.total_profit.to_string().green(),
            self.picked_items.len(),
            format!("{:?}", self.picked_items).yellow(),
            self.utilization.join(" ").blue(),
            self.iterations.to_string().cyan(),
            run_time
        )
    }
}
