#!/usr/bin/env rust-script
//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! ```

fn main() {
	//let filename = std::env::args().nth(1).expect("No filename provided");
	let filename = "../input";
	let input = std::fs::read_to_string(filename).expect("Can't read file");
	let mut lines = input.lines();

	let seeds: Vec<u64> = lines.next().expect("No first line in file.").split(": ").nth(1).expect("Seed line doesn't have a : delim")
		.split(" ").map(|s| s.parse().expect("NaN")).collect();

    println!("seeds: {:?}", seeds);

    const MAP_NAMES: [&'static str; 7] = ["Soil", "Fertilizer", "Water", "Light", "Temperature", "Humidity", "Location"];

    let mut lowest = u64::MAX; // Tracks the lowest location, todo.
    for seed in seeds {
        let mut lines = input.lines();
        // Skip seed lines
        lines.next();
        lines.next();

        let mut cursor = 0; // Tracks what map we're in
        let mut current = seed; // Tracks what our current value is, after mapping
        let mut mapped = false;

        print!("Seed {seed}");

        for l in lines {
            if l == "" {
                print!(" {} {}", MAP_NAMES[cursor], current);
                cursor += 1;
                mapped = false;
                continue;
            }
            if l.contains("map") { continue; }
            if mapped { continue; }

            let mut iter = l.split(" ");
            let dst = iter.next().expect("Map missing a dst").parse::<u64>().unwrap();
            let src = iter.next().expect("Map missing a src").parse::<u64>().unwrap();
            let rng = iter.next().expect("Map missing a rng").parse::<u64>().unwrap();

            if current > src && current < src + rng {
                current = dst + (current - src);
                mapped = true;
            }

            // for i in 0..rng {
            //     if src + i == current {
            //         current = dst + i;
            //         mapped = true;
            //         break;
            //     }
            // }
        }

        if current < lowest {
            lowest = current;
        }
        println!();
    }

    println!("Lowest location: {lowest}");
}