use nom::{
    bytes::complete::{tag, take_until}, character::complete::{char, digit1}, combinator::{map_res, recognize}, multi::many0, sequence::{delimited, separated_pair}, IResult
};
use anyhow::Result;

// Bailing on the nom strat, too dumb to figure this one out...
#[derive(Debug, PartialEq)]
struct Mul(i32, i32);

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let (input, (first, second)) = delimited(
        tag("mul("),
        separated_pair(parse_number, char(','), parse_number),
        char(')')
    )(input)?;

    Ok((input, Mul(first, second)))
}

fn parse_many_mul(input: &str) -> IResult<&str, Vec<Mul>> {
    many0(
        delimited(
            take_until("mul("),
            parse_mul,
            take_until("mul(")
        )
    )(input)
}

fn main() -> Result<()> {
    //let input = std::fs::read("../input.txt")?;

    let out = parse_many_mul("wq4$$11asdemul(3,3)");

    println!("{:?}", out);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (_, res) = parse_many_mul(input).unwrap();
        println!("Got: {:?}", res);
        let kg = vec![Mul(2,4), Mul(5,5), Mul(11,8), Mul(8,5)];

        assert_eq!(res.iter().count(), kg.iter().count());

        for (lhs, rhs) in res.iter().zip(kg.iter()) {
            println!("Checking {:?} vs {:?}", lhs, rhs);
            assert_eq!(lhs, rhs);
        }
    }
}
