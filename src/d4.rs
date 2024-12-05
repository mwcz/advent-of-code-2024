//! A solution to day 4 year 2024.
//! https://adventofcode.com/2024/day/4

use crate::{direction::OrdDir, grid::Grid, point::Point};

type Model = (Grid<Letter>, Vec<Point<2>>);
type Answer = usize;

pub fn parse(input: String) -> Model {
    let mut xlocs = Vec::new();

    let grid: Vec<Vec<Letter>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let letter = c.into();
                    if letter == Letter::X {
                        xlocs.push([x, y].into());
                    }
                    letter
                })
                .collect()
        })
        .collect();

    let grid = Grid::new(grid);

    (grid, xlocs)
}

pub fn part1(model: Model) -> Answer {
    let mut sum = 0;

    for xloc in model.1 {
        for dir in OrdDir::all() {
            sum += search_p1(&model.0, Letter::X, xloc, dir);
        }
    }

    sum
}

pub fn part2(model: Model) -> Answer {
    0
}

fn search_p1(grid: &Grid<Letter>, letter: Letter, point: Point<2>, dir: OrdDir) -> usize {
    if letter == Letter::S {
        return 1;
    }

    // get the next letter in the given direction
    // at S, return count + 1

    let next_point = point.move_in_grid_diag(dir, grid);

    let next = next_point.and_then(|p| grid.get(p.x(), p.y()));

    if let Some(letter_in_line) = next {
        if letter_in_line == letter.next() {
            search_p1(grid, letter.next(), next_point.unwrap(), dir)
        } else {
            0
        }
    } else {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Letter {
    X,
    M,
    A,
    S,
    None,
}

impl Letter {
    fn next(&self) -> Letter {
        match self {
            Letter::X => Letter::M,
            Letter::M => Letter::A,
            Letter::A => Letter::S,
            Letter::S => Letter::None,
            Letter::None => Letter::None,
        }
    }
}

impl From<char> for Letter {
    fn from(value: char) -> Self {
        use Letter::*;
        match value {
            'X' => X,
            'M' => M,
            'A' => A,
            'S' => S,
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d4");
    const EXAMPLE: &str = include_str!("../examples/d4");
    const EXAMPLE_SMALL: &str = include_str!("../examples/d4-small");

    #[test]
    fn d4p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 18);
    }

    #[test]
    fn d4p1_example_small_test() {
        assert_eq!(part1(parse(EXAMPLE_SMALL.to_string())), 4);
    }

    #[test]
    fn d4p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 2500);
    }

    #[test]
    fn d4p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 9);
    }

    // #[test]
    // fn d4p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
