//! A solution to day 10 year 2024.
//! https://adventofcode.com/2024/day/10

use termion::{color, style};

use crate::{grid::Grid, point::Point};

type Model = Map;
type Answer = u32;

pub fn parse(input: String) -> Model {
    let mut trailheads = Vec::new();

    let topography = Grid::new(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '0' {
                            trailheads.push([x, y].into())
                        }
                        c.to_digit(10).unwrap() as u8
                    })
                    .collect()
            })
            .collect(),
    );

    Map {
        trailheads,
        topography,
    }
}

pub fn part1(model: Model) -> Answer {
    println!("{}", model.topography);
    println!("{:?}", model.trailheads);
    model
        .trailheads
        .iter()
        .map(|trailhead| model.search(*trailhead))
        .sum()
    // model.search(model.trailheads[0])
}

pub fn part2(model: Model) -> Answer {
    0
}

pub struct Map {
    trailheads: Vec<Point<2>>,
    topography: Grid<u8>,
}

impl Map {
    fn search(&self, loc: Point<2>) -> u32 {
        let cur = self.topography.getp(loc).unwrap();

        std::thread::sleep_ms(10);
        println!("{}", termion::clear::All);
        let mut output = format!("{}", self.topography);
        for (y, line) in output.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if loc.x() == x as i64 && loc.y() == y as i64 {
                    if cur == 9 {
                        print!(
                            "{blue}{c}{reset}",
                            blue = color::Fg(color::Blue),
                            reset = style::Reset
                        );
                    } else {
                        print!(
                            "{green}{c}{reset}",
                            green = color::Fg(color::Green),
                            reset = style::Reset
                        );
                    }
                } else {
                    print!("{c}");
                }
            }
            println!();
        }

        if cur == 9 {
            return 1;
        }

        let moves = self.topography.adj_4(loc);

        let mut score = 0;

        moves.cells.into_iter().flatten().for_each(|cell| {
            // if cur == 8 && cell.data == 9 {
            //     score += 1;
            //     println!("found 9 at {}", cell.pos);
            //     for (y, line) in output.lines().enumerate() {
            //         for (x, c) in line.chars().enumerate() {
            //             if cell.pos.x() == x as i64 && cell.pos.y() == y as i64 {
            //                 print!(
            //                     "{green}!{reset}",
            //                     green = color::Fg(color::Green),
            //                     reset = style::Reset
            //                 );
            //             } else {
            //                 print!("{c}");
            //             }
            //         }
            //         println!();
            //     }
            //} else
            if cur + 1 == cell.data {
                score += self.search(cell.pos);
            }
        });

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d10");
    const EXAMPLE: &str = include_str!("../examples/d10");

    #[test]
    fn d10p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 36);
    }

    // #[test]
    // fn d10p1_input_test() {
    //     assert_eq!(
    //         part1(parse(INPUT.to_string())),
    //         "put part 1 final answer here"
    //     );
    // }
    //
    // #[test]
    // fn d10p2_example_test() {
    //     assert_eq!(
    //         part2(parse(EXAMPLE.to_string())),
    //         "put part 2 example answer here"
    //     );
    // }
    //
    // #[test]
    // fn d10p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
