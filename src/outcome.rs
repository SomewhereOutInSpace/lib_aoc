use std::{fmt::{Debug, Display, Write}, time::Duration};
use colored::{Colorize, ColoredString};

use crate::Timer;

/// Represents the final product of a [`Solution`](crate::Solution).
pub struct Outcome<T: Debug> {
    /// The computed answer to part one, if any.
    pub part_one: Option<T>,
    /// The computer answer to part two, if any.
    pub part_two: Option<T>,
    /// Benchmark timing data.
    pub timings: Timings,
    /// The day of the source [`Solution`](crate::Solution).
    pub day: u8,
}

impl<T: Debug> Display for Outcome<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "--- DAY {} ---", self.day.to_string().bright_cyan().bold())?;
        writeln!(f, "{}: {}", "Part 1".bold(), format_answer(&self.part_one))?;
        writeln!(f, "{}: {}", "Part 2".bold(), format_answer(&self.part_two))?;

        let opt_target = match cfg!(debug_assertions) {
            true => "(DEBUG)".yellow().bold(),
            false => "(RELEASE)".green().bold()
        };

        writeln!(f, "\n--- BENCH {opt_target} ---\n{}", self.timings)?;

        Ok(())
    }
}

fn format_answer(ans: &Option<impl Debug>) -> ColoredString {
    match ans {
        Some(answer) => format!("{answer:?}").green(),
        None => "unimplemented".red()
    }
}

/// Represents benchmarking timing data from the execution of a solution.
pub struct Timings {
    pub parsing: Duration,
    pub part_one: Duration,
    pub part_two: Duration,
    pub total: Duration
}

impl From<Timer> for Timings {
    fn from(timer: Timer) -> Self {
        let buffer: Vec<_> = timer
            .buffer()
            .iter()
            .map(|lap| lap.1)
            .collect();

        Self {
            parsing: buffer[0],
            part_one: buffer[1],
            part_two: buffer[2],
            total: buffer[3]
        }
    }
}

impl Display for Timings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let mut write_timing = |time: &Duration, name: &str| {
            writeln!(
                output,
                "{}: {} μs / {} ns",
                name.bold(),
                time.as_micros().to_string().green(),
                time.as_nanos().to_string().green()
            )
        };

        write_timing(&self.parsing, "Parsing")?;
        write_timing(&self.part_one, "Parsing")?;
        write_timing(&self.part_one, "Parsing")?;
        write_timing(&self.part_one, "Parsing")?;

        write!(f, "{}", output.trim())?;
        Ok(())
    }
}