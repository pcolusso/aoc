use std::io::BufRead;
use anyhow::Result;

enum Kinds {
    Calorie(i32),
    Breaker,
    Other
}

impl From<String> for Kinds {
    fn from(s: String) -> Self {
        if s == "\n" {
            Kinds::Breaker
        } else {
            match s.parse::<i32>() {
                Ok(calories) => Kinds::Calorie(calories),
                Err(_) => { Kinds::Other }
            }
        }
    }
}

fn run(input: Vec<String>) -> Vec<i32> {
    let mut calorie_counts = vec!();
    let mut current_tally = 0;

    for line in input {
        let kind = Kinds::from(line);
        match kind {
            Kinds::Calorie(n) => {
                current_tally += n
            },
            Kinds::Breaker => {
                calorie_counts.push(current_tally);
                current_tally = 0;
            },
            Kinds::Other => {
                eprintln!("Unexpected input");
            }
        }
    }

    if current_tally > 0 {
        calorie_counts.push(current_tally);
    }

    calorie_counts
}

// Create a test for the above function with a fixed input
#[cfg(test)]
mod tests {
    #[test]
    fn test_casae_1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let as_lines: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
        let processed = super::run(as_lines);
        assert_eq!(processed, vec!(6000, 4000, 11000, 24000, 10000));
    }
}


fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let mut calorie_counts = run(lines);
    calorie_counts.sort_by(|a, b| b.cmp(a)); // Sort is wasteful, we really only need the top 3.

    println!("Top calorie elf: {}", &calorie_counts[0]);

    let top_3_sum: i32 = calorie_counts[0..3].iter().sum();
    println!("Calorie counts: {}", top_3_sum);

    Ok(())
}
