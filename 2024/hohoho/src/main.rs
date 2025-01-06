use std::io::{Error, Read, Write};
use std::fs::File;

const CIPHER: [u8; 6] = *b"HOHOHO";

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("Need a source file");
    let output = args.get(2).expect("Need an output file");

    let mut input = File::open(input).expect("Can't open source");
    let mut output = File::create(output).expect("Can't open destination");
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    let mut encrypted: Vec<u8> = buf.iter().enumerate().map(|(i, &b)| b ^ CIPHER[i % CIPHER.len()]).collect();
    output.write_all(&mut encrypted)?;

    Ok(())

}
