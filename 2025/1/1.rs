#!/usr/bin/env rust-script
//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! ```


fn main() {
    let filename = std::env::args().nth(1).expect("No filename provided");
    let input = std::fs::read_to_string(filename).expect("Can't read file");

    let mut value = 50;
    let mut clicks = 0;
    let mut crosses = 0;
    for cmd in input.lines() {
        let mut chars = cmd.chars();
        let direction = chars.next().expect("First cmd was not a char");

        let sign = if direction == 'L' {
            -1
        } else if direction == 'R' {
            1
        } else {
            panic!("Expected L or R")
        };
        let num: i32 = chars.as_str().parse().expect("Wasn't a number");

        let next = ((value + num * sign) + 100).rem_euclid(100);
        
        if next == 0 {
            clicks += 1;
        }
        // End part 1
        
        let mut cursor = value;
        for _ in 0..num {
            cursor += sign;
            cursor = cursor.rem_euclid(100);
            dbg!(&cursor);

            if cursor == 0 {
                crosses += 1;
            }
        }
        assert_eq!(cursor, next);

        println!("{value} -({})-> {next} ", num * sign);

        value = next;
    }
    println!("Clicks: {clicks}");
    println!("Crosses: {crosses}");
}
