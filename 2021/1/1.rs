use std::io::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum State {
  NoChange,
  Increased,
  Decreased
}

// Exploring a more iterative approach. 

fn main() -> Result<(), Error> {
  let file = fs::read_to_string("input.txt")?;
  let input: Vec<i32> = file.lines().map(|l| l.parse().unwrap() ).collect();

  {
    let mut states: Vec<State> = vec!();

    for i in 1..input.len() {
      let a = input[i-1];
      let b = input[i];

      if b > a {
        states.push(State::Increased);
      } else {
        states.push(State::Decreased);
      }
    }

    println!("{:?}", states.into_iter().filter(|s| *s == State::Increased).collect::<Vec<State>>().len());
  }
  
  {
    let mut states: Vec<State> = vec!();

    for i in 3..input.len() {
      let a = input[i-3] + input[i-2] + input[i-1];
      let b = input[i-2] + input[i-1] + input[i];

      if b > a {
        states.push(State::Increased);
      } else {
        states.push(State::Decreased);
      }
    }

    println!("{:?}", states.into_iter().filter(|s| *s == State::Increased).collect::<Vec<State>>().len());
  }


  Ok(())
}
