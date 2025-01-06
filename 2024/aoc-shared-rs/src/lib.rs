use std::{fs::File, io::Read, path::Path};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn decode(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
    let cipher = b"HOHOHO";
    let mut f = File::open(path)?;
    let mut b = Vec::new();
    f.read_to_end(&mut b)?;
    b.iter_mut()
        .enumerate()
        .for_each(|(i, byte)| *byte ^= cipher[i % cipher.len()]);

    let s = String::from_utf8(b).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(s)
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
