use std::io::BufRead;

#[derive(Debug)]
struct MyMove(Move);
#[derive(Debug)]
struct OpponentMove(Move);

#[derive(PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors
}

impl Into<i32> for Move {
    fn into(self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}

enum Resolution {
    Win,
    Loss,
    Tie
}

impl Into<i32> for Resolution {
    fn into(self) -> i32 {
        match self {
            Resolution::Win => 6,
            Resolution::Loss => 0,
            Resolution::Tie => 3
        }
    }
}

impl From<(&OpponentMove, &MyMove)> for Resolution {
    fn from(value: (&OpponentMove, &MyMove)) -> Self {
        let (opponent_move, my_move) = value;
        match (&opponent_move.0, &my_move.0) {
            (Move::Scissors, Move::Rock) => Resolution::Win,
            (Move::Paper, Move::Scissors) => Resolution::Win,
            (Move::Rock, Move::Paper) => Resolution::Win,
            (Move::Scissors, Move::Paper) => Resolution::Loss,
            (Move::Paper, Move::Rock) => Resolution::Loss,
            (Move::Rock, Move::Scissors) => Resolution::Loss,
            (Move::Rock, Move::Rock) => Resolution::Tie,
            (Move::Scissors, Move::Scissors) => Resolution::Tie,
            (Move::Paper, Move::Paper) => Resolution::Tie,
        }
    }
}


impl From<char> for OpponentMove{
    fn from(c: char) -> Self {
        match c {
            'A' => OpponentMove(Move::Rock),
            'B' => OpponentMove(Move::Paper),
            'C' => OpponentMove(Move::Scissors),
            _ => panic!("Invalid move")
        }
    }
}

impl From<char> for MyMove {
    fn from(c: char) -> Self {
        match c {
            'X' => MyMove(Move::Rock),
            'Y' => MyMove(Move::Paper),
            'Z' => MyMove(Move::Scissors),
            _ => panic!("Invalid move")
        }
    }
}

#[derive(Debug)]
struct Game {
    opponent_move: OpponentMove,
    my_move: MyMove
}

impl From<String> for Game {
    fn from(s: String) -> Self {
        // Strip whitespace and take the first two characters
        let s = s
            .trim()
            .replace(" ", "")
            .chars()
            .take(2)
            .collect::<Vec<char>>();
        let opponent_move = OpponentMove::from(s[0]);
        let my_move = MyMove::from(s[1]);
        Game {
            opponent_move,
            my_move
        }
    }
}

fn resolve(g: Game) -> i32 {
    let resolution: Resolution = (&g.opponent_move, &g.my_move).into();
    let resolution_value: i32 = resolution.into();
    let my_move = g.my_move.0;
    let my_value: i32 = my_move.into();

    resolution_value + my_value
}

#[derive(Debug)]
enum ExpectedResult {
    ToWin,
    ToLose,
    ToTie
}

impl From<char> for ExpectedResult {
    fn from(c: char) -> Self {
        match c {
            'Z' => ExpectedResult::ToWin,
            'X' => ExpectedResult::ToLose,
            'Y' => ExpectedResult::ToTie,
            _ => panic!("Invalid expected result")
        }
    }
}


struct Game2(OpponentMove, ExpectedResult);

impl From<String> for Game2 {
    fn from(s: String) -> Self {
        // Strip whitespace and take the first two characters
        let s = s
            .trim()
            .replace(" ", "")
            .chars()
            .take(2)
            .collect::<Vec<char>>();
        let opponent_move = OpponentMove::from(s[0]);
        let expected_result = ExpectedResult::from(s[1]);
        Game2(opponent_move, expected_result)
    }
}

fn required_move(opponent_move: &OpponentMove, expected_result: ExpectedResult) -> MyMove {
    match (&opponent_move.0, expected_result) {
        (Move::Rock, ExpectedResult::ToWin) => MyMove(Move::Paper),
        (Move::Rock, ExpectedResult::ToLose) => MyMove(Move::Scissors),
        (Move::Rock, ExpectedResult::ToTie) => MyMove(Move::Rock),
        (Move::Paper, ExpectedResult::ToWin) => MyMove(Move::Scissors),
        (Move::Paper, ExpectedResult::ToLose) => MyMove(Move::Rock),
        (Move::Paper, ExpectedResult::ToTie) => MyMove(Move::Paper),
        (Move::Scissors, ExpectedResult::ToWin) => MyMove(Move::Rock),
        (Move::Scissors, ExpectedResult::ToLose) => MyMove(Move::Paper),
        (Move::Scissors, ExpectedResult::ToTie) => MyMove(Move::Scissors),
    }
}

fn resolve_2(expected_game: Game2) -> i32 {
    let Game2(opponent_move, expected_result) = expected_game;
    let required_move = required_move(&opponent_move, expected_result);
    let game = Game {
        opponent_move,
        my_move: required_move
    };

    resolve(game)
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let stdin = std::io::stdin();
    let input: Vec<String> = stdin
        .lock()
        .lines()
        .map (|line| line.unwrap() )
        .collect();

    let games: Vec<Game> = input
        .clone()
        .into_iter()
        .map(|line| Game::from(line) )
        .collect();

    let result: i32 = games
        .into_iter()
        .map(|g| resolve(g))
        .sum();

    println!("Result {} ", result);

    let games2: Vec<Game2> = input
        .into_iter()
        .map(|line| Game2::from(line) )
        .collect();

    let result2: i32 = games2
        .into_iter()
        .map(|g| resolve_2(g))
        .sum();

    println!("Result2 {} ", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let g = Game {
            opponent_move: OpponentMove::from('A'),
            my_move: MyMove::from('Y')
        };
        assert_eq!(resolve(g), 8)
    }

    #[test]
    fn test2() {
        let g = Game {
            opponent_move: OpponentMove::from('B'),
            my_move: MyMove::from('X')
        };
        assert_eq!(resolve(g), 1)
    }

    #[test]
    fn test3() {
        let g = Game {
            opponent_move: OpponentMove::from('C'),
            my_move: MyMove::from('Z')
        };
        assert_eq!(resolve(g), 6)
    }

    #[test]
    fn test_parse() {
        let g = Game::from("A X".to_string());
        println!("{:?}", g);
        assert_eq!(g.opponent_move.0, Move::Rock);
        assert_eq!(g.my_move.0, Move::Rock);
    }

    #[test]
    fn integ() {
        let input = "A Y
        B X
        C Z";

        let result: i32 = input
            .lines()
            .map(|line| Game::from(line.to_string()))
            .map(|g| resolve(g))
            .sum();
        assert_eq!(result, 15)
    }

    #[test]
    fn test4() {
        let g = Game2::from("A Y".to_string());
        let expeceted_move = required_move(&g.0, g.1);
        assert_eq!(expeceted_move.0, Move::Rock);
    }

    #[test]
    fn test5() {
        let g = Game2::from("B X".to_string());
        assert_eq!(resolve_2(g), 1)
    }

}