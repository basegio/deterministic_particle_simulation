use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionDiagnostic {
    pub times: Vec<Duration>,
    pub max_samples: usize,
}

#[derive(Resource)]
pub struct GridUpdateDiagnostic {
    pub times: Vec<Duration>,
    pub max_samples: usize,
}

impl CollisionDiagnostic {
    pub fn new(max_samples: usize) -> Self {
        Self {
            times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn print_result(&self) {
        if self.times.is_empty() {
            return;
        }

        let total: Duration = self.times.iter().sum();
        let avg_ns = total.as_nanos() / self.times.len() as u128;
        let min_ns = self.times.iter().min().unwrap().as_nanos();
        let max_ns = self.times.iter().max().unwrap().as_nanos();
        let avg_mc = Duration::from_nanos(avg_ns as u64).as_micros();
        let min_mc = Duration::from_nanos(min_ns as u64).as_micros();
        let max_mc = Duration::from_nanos(max_ns as u64).as_micros();

        println!("---");
        println!("Times to run a complete call of solve_collisions fn");
        println!("Total diag duration {} seconds", total.as_secs_f64());
        println!("Average {} µs | {} ns", avg_mc, avg_ns);
        println!("Minimum {} µs | {} ns", min_mc, min_ns);
        println!("Maximum {} µs | {} ns", max_mc, max_ns);
    }
}

impl GridUpdateDiagnostic {
    pub fn new(max_samples: usize) -> Self {
        Self {
            times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn print_result(&self) {
        if self.times.is_empty() {
            return;
        }

        let total: Duration = self.times.iter().sum();
        let avg_ns = total.as_nanos() / self.times.len() as u128;
        let min_ns = self.times.iter().min().unwrap().as_nanos();
        let max_ns = self.times.iter().max().unwrap().as_nanos();
        let avg_mc = Duration::from_nanos(avg_ns as u64).as_micros();
        let min_mc = Duration::from_nanos(min_ns as u64).as_micros();
        let max_mc = Duration::from_nanos(max_ns as u64).as_micros();

        println!("---");
        println!("Times to run a complete call of update_grid fn");
        println!("Total diag duration {} seconds", total.as_secs_f64());
        println!("Average {} µs | {} ns", avg_mc, avg_ns);
        println!("Minimum {} µs | {} ns", min_mc, min_ns);
        println!("Maximum {} µs | {} ns", max_mc, max_ns);
    }
}
