#!/usr/bin/env rust-script
//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! ```
use ahash::AHashMap;

fn main() {
	//let filename = std::env::args().nth(1).expect("No filename provided");
	let filename = "../input";
	let input = std::fs::read_to_string(filename).expect("Can't read file");
	let mut lines = input.lines();

	let seeds: Vec<u64> = lines.next().expect("No first line in file.").split(": ").nth(1).expect("Seed line doesn't have a : delim")
		.split(" ").map(|s| s.parse().expect("NaN")).collect();

    println!("seeds: {:?}", seeds);

    const NUM_MAPS: usize = 7;
    const MAP_NAMES: [&'static str; 7] = ["Soil", "Fertilizer", "Water", "Light", "Temperature", "Humidity", "Location"];
    //let mut maps: [AHashMap<u64, u64>; NUM_MAPS] = std::array::from_fn(|_| AHashMap::with_capacity(100000));
    let mut maps: [Vec<(u64, u64)>; NUM_MAPS] = std::array::from_fn(|_| Vec::with_capacity(1_000_000));
	let mut cursor = 0;

	// skip empty line
	lines.next();

	for l in lines {
        if l == "" {
            cursor += 1;
            continue;
        }
        if l.contains("map") {
            println!("map: {}", l);
            continue;
        } else {
            let mut iter = l.split(" ");
            let dst = iter.next().expect("Map missing a dst").parse::<u64>().unwrap();
            let src = iter.next().expect("Map missing a src").parse::<u64>().unwrap();
            let rng = iter.next().expect("Map missing a rng").parse::<u64>().unwrap();

            for i in 0..rng {
                //maps[cursor].insert(src + i, dst + i);
                maps[cursor].push((src + i, dst + i));
            }
        }
    }
    println!("Done parsing");

    let mut lowest = u64::MAX;
    for seed in seeds {
        print!("Seed {seed}");

        let mut current = seed;
        for i in 0..NUM_MAPS {
            let mut mapped = None;
            for (k, v) in maps[i].iter() {
                if current == *k {
                    mapped = Some(v);
                    break;
                }
            }
            if let Some(v) = mapped {
                current = *v;
            }
            //current = *maps[i].get(&current).unwrap_or(&current);
            print!(" {} {}", MAP_NAMES[i], current);
        }
        println!();
        if current < lowest {
            lowest = current;
        }
    }

    println!("{}", lowest);
}