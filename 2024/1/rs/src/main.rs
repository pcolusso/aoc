use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read("../input.txt")?;
    let count = input.lines().count();
    let (mut l, mut r) = (Vec::with_capacity(count), Vec::with_capacity(count));

    for line in input.lines() {
        let line = line?;
        let mut iter = line.split_ascii_whitespace();
        match (iter.next(), iter.next()) {
            (Some(x), Some(y)) => {
                let x = x.parse::<i32>()?;
                let y = y.parse::<i32>()?;
                l.push(x);
                r.push(y);
            }
            _ => panic!("Failed to parse input file"),
        };
    }

    l.sort();
    r.sort();
    let mut acc = 0;

    for (x, y) in l.iter().zip(r.iter()) {
        acc += (x - y).abs();
    }
    println!("Part 1: {}", acc);

    acc = 0;
    for lhs in l.iter() {
        let count = r.iter().filter(|y| lhs == *y).count();
        acc += *lhs * count as i32;
    }

    println!("Part 2: {}", acc);

    Ok(())
}
