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
impl<T: Iterator> IteratorExt for T{
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
                } else {
                    if mulligan {
                        mulligan = false;
                        ControlFlow::Continue(())
                    } else {
                        ControlFlow::Break(())
                    }

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
        let all_reasonable = self.levels.windows(2).map(|w| (w[0] - w[1]).abs()).all(|d| d >= 1 && d <= 3);

        (all_increasing || all_decreasing) && all_reasonable
    }

    fn almost_safe(&self) -> bool {
        let mut all_increasing = true;
        let mut all_decreasing = true;
        let mut all_reasonable = true;
        let mut mulligan = true;

        for w in self.levels.windows(2) {
            if w[0] < w[1] {
                if !mulligan {
                    all_decreasing = false;
                } else {
                     mulligan = false;
                     continue;
                }
            }

            if w[0] > w[1] {
                if !mulligan {
                    all_increasing = false;
                } else {
                    mulligan = false;
                    continue;
                }
            }
            let diff = (w[0] - w[1]).abs();

            if !(diff >= 1 || diff <= 3) {
                if !mulligan {
                    all_reasonable = false;
                } else {
                    mulligan = false;
                    continue;
                }
            }
         }

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
        .filter_map(|r| r.safe().then(|| true))
        .count();

    println!("Step 1: {}", step_one);

    let step_two = reports
        .iter()
        .filter_map(|r| r.almost_safe().then(|| true))
        .count();

    println!("Step 2: {}", step_two);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_ascending_safe() {
        let report = Report {
            levels: vec![1, 4, 8, 12, 48],
        };
        assert!(report.safe())
    }

    #[test]
    fn all_descending_safe() {
        let report = Report {
            levels: vec![92, 91, 90, 89, 70],
        };
        assert!(report.safe())
    }

    // See, writing a test made it clear I didn't fully understand the third requirement!
    #[test]
    fn reasonable_step_safe() {
        let report = Report {
            levels: vec![93, 90, 93, 92, 91],
        };
        assert!(report.safe())
    }

    #[test]
    fn unreasonable_step_not_safe() {
        let report = Report {
            levels: vec![93, 90, 80, 92, 91],
        };
        assert!(!report.safe())
    }

    #[test]
    fn bullshit() {
        let report = Report { levels: vec!(1, 1, 2, 3, 4)};
        assert!(report.almost_safe())
    }

    #[test]
    fn more_bullshit() {
        let report = Report { levels: vec![2, 5, 4, 3, 2]};
        assert!(report.almost_safe())
    }
}
