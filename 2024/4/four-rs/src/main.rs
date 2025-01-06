use std::collections::HashSet;
use anyhow::Result;
use aoc_shared_rs::decode;
use glam::IVec2;

type Coord = IVec2;
type Int = i32;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Span(Coord, Coord, Coord, Coord);

impl Span {
    fn make(root: Coord, a: Coord, b: Coord, c: Coord) -> Self {
        Span(root, root + a, root + b, root + c)
    }
}

struct Grid {
    cells: Vec<char>,
    width: Int,
}

struct Run<'a>(
    Option<&'a char>,
    Option<&'a char>,
    Option<&'a char>,
    Option<&'a char>,
);

impl<'a> Run<'a> {
    fn is_xmas(&self) -> bool {
        if let Some('X') = self.0 {
            if let Some('M') = self.1 {
                if let Some('A') = self.2 {
                    if let Some('S') = self.3 {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let width = input.lines().take(1).count() as Int;
        let cells = input.chars().collect(); // This may capture \n
        Self { cells, width }
    }

    fn get(&self, coord: &Coord) -> Option<&char> {
        self.cells.get((coord.y * self.width + coord.x) as usize)
    }

    fn get_span(&self, span: Span) -> Run {
        Run(
            self.get(&span.0),
            self.get(&span.1),
            self.get(&span.2),
            self.get(&span.3),
        )
    }

    fn iter(&self) -> GridIter {
        GridIter { grid: self, current: 0 }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    current: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (Coord, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.grid.cells.len() {
            None
        } else {
            let x = self.current as Int % self.grid.width;
            let y = self.current as Int / self.grid.width;
            let coord = Coord::new(x, y);
            let cell = self.grid.cells[self.current];
            self.current += 1;
            Some((coord, cell))
        }
    }
}

fn c(x: Int, y: Int) -> IVec2 {
    IVec2::new(x, y)
}

fn directions(root: Coord) -> Vec<Span> {
    let mut ds = Vec::with_capacity(8);
    // Horizontal Forward
    ds.push(Span::make(root, c(1, 0), c(2, 0), c(3, 0)));
    // Horizontal Backward
    ds.push(Span::make(root, c(-1, 0), c(-2, 0), c(-3, 0)));
    // Vertical Forward
    ds.push(Span::make(root, c(0, 1), c(0, 2), c(0, 3)));
    // Vertical Backward
    ds.push(Span::make(root, c(0, -1), c(0, -2), c(0, -3)));
    // Diagonal Down-Right
    ds.push(Span::make(root, c(1, 1), c(2, 2), c(3, 3)));
    // Diagonal Down-Left
    ds.push(Span::make(root, c(-1, 1), c(-2, 2), c(-3, 3)));
    // Diagonal Up-Right
    ds.push(Span::make(root, c(1, -1), c(2, -2), c(3, -3)));
    // Diagonal Up-Left
    ds.push(Span::make(root, c(-1, -1), c(-2, -2), c(-3, -3)));
    ds
}

fn stage_one(input: String) -> usize {
    let grid = Grid::new(&input.replace("\n", ""));
    let mut count = 0;

    for (coord, _char) in grid.iter() {
        for d in directions(coord) {
            let run = grid.get_span(d);
            if run.is_xmas() {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let input = decode("../input.enc")?;


    print!("{}", input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_input() {
        let input = decode("../input.enc").expect("Can't find input");
        let grid = Grid::new(&input);
        let x = grid.get(&Coord { x: 46, y: 0 });
        let m = grid.get(&Coord { x: 13, y: 1 });
        let a = grid.get(&Coord { x: 13, y: 4 });
        let s = grid.get(&Coord { x: 42, y: 43 });
        assert_eq!(x.unwrap(), &'X');
        assert_eq!(m.unwrap(), &'M');
        assert_eq!(a.unwrap(), &'A');
        assert_eq!(s.unwrap(), &'S');
        let run = Run(x, m, a, s);
        assert!(run.is_xmas());
    }

    #[test]
    fn test_case() {
        let output = stage_one(SAMPLE.to_owned());
        assert_eq!(output, 18);

    }

    #[test]
    fn grid_construction_is_sound() {

    }
}
