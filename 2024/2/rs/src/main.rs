use anyhow::{anyhow, Error, Result};
use std::io::BufRead;

#[derive(Debug)]
struct Report {
    levels: Vec<i8>,
}

impl From<&String> for Report {
    fn from(value: &String) -> Self {
        let levels: Vec<i8> = value
            .split_whitespace()
            .map(|s| {
                s.parse::<i8>()
                    .expect(&format!("Malformed input in '{}', [{}]", value, s))
            })
            .collect();

        Report { levels }
    }
}

// Ugh, leaving this here for posterity cause thought I was being super clever...
trait IteratorExt: Iterator {
    fn allmost<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool;
}

use std::ops::ControlFlow;
impl<T: Iterator> IteratorExt for T {
    #[inline]
    // clever pun, yoinked from the source code for all
    fn allmost<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        #[inline]
        fn check<T>(mut f: impl FnMut(T) -> bool) -> impl FnMut((), T) -> ControlFlow<()> {
            let mut mulligan = true;
            move |(), x| {
                if f(x) {
                    ControlFlow::Continue(())
                } else if mulligan {
                    mulligan = false;
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(())
                }
            }
        }
        self.try_fold((), check(f)) == ControlFlow::Continue(())
    }
}

impl Report {
    fn safe(&self) -> bool {
        let all_increasing = self.levels.windows(2).all(|w| w[0] < w[1]);
        let all_decreasing = self.levels.windows(2).all(|w| w[0] > w[1]);
        let all_reasonable = self
            .levels
            .windows(2)
            .map(|w| (w[0] - w[1]).abs())
            .all(|d| (1..=3).contains(&d));

        (all_increasing || all_decreasing) && all_reasonable
    }

    // i think your approach here is fundamentally broken. A mulligan is not the same as removing an element.
    fn almost_safe(&self) -> bool {
        let all_increasing = self.levels.windows(2).allmost(|w| w[0] < w[1]);
        let all_decreasing = self.levels.windows(2).allmost(|w| w[0] > w[1]);
        let all_reasonable = self
            .levels
            .windows(2)
            .map(|w| (w[0] - w[1]).abs())
            .allmost(|d| (1..=3).contains(&d));

        (all_increasing || all_decreasing) && all_reasonable
    }
}

fn main() -> Result<()> {
    let input = std::fs::read("../input.txt")?;
    let reports: Vec<_> = input
        .lines()
        .map(|l| Report::from(&l.expect("Couldn't read line")))
        .collect();

    let step_one = reports
        .iter()
        .filter_map(|r| r.safe().then_some(true))
        .count();

    println!("Step 1: {}", step_one);

    let step_two = reports
        .iter()
        .filter_map(|r| r.almost_safe().then_some(true))
        .count();

    println!("Step 2: {}", step_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    
    #[test]
    fn safe_with_removals() {
        let report = Report { levels: vec![1, 3, 2, 4, 5]}.almost_safe();
        let report2 = Report { levels: vec![8, 6, 4, 4, 1] }.almost_safe();

        assert!(report);
        assert!(report2);
    }

    #[test]
    fn unreasonable_step_not_safe() {
        let report = Report {
            levels: vec![93, 90, 80, 92, 91],
        };
        assert!(!report.safe())
    }

    #[test]
    fn unsafe_even_with_removals() {
        let report = Report {
            levels: vec![1, 2, 7, 8, 9],
        };
        let is_safe = report.almost_safe();
        assert!(!is_safe);


        let report = Report {
            levels: vec![9, 7, 6, 2, 1],
        };
        let is_safe = report.almost_safe();
        assert!(!is_safe);
    }

    #[test]
    fn bullshit() {
        let report = Report {
            levels: vec![1, 1, 2, 3, 4],
        };
        let is_safe = report.almost_safe();
        assert!(!is_safe);
    }

    #[test]
    fn more_bullshit() {
        let report = Report {
            levels: vec![2, 5, 4, 3, 2],
        };
        assert!(report.almost_safe())
    }

    #[test]
    fn test_allmost() {
        let all_fives = [5, 5, 3, 5, 5];
        assert!(all_fives.iter().allmost(|n| *n == 5));
    }
}
