use std::{fs::File, io::Read, path::Path};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn decode(path: impl AsRef<Path>) -> Result<Vec<u8>, std::io::Error> {
    let cipher = b"HOHOHO";
    let mut f = File::open(path)?;
    let mut b = Vec::new();
    f.read_to_end(&mut b)?;
    b.iter_mut()
        .enumerate()
        .for_each(|(i, byte)| *byte ^= cipher[i % cipher.len()]);

    Ok(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
