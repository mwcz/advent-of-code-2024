//! A solution to day 10 year 2024.
//! https://adventofcode.com/2024/day/10

use std::collections::HashSet;

use itertools::Itertools;
use termion::{color, style};

use crate::{grid::Grid, point::Point};

type Model = Map;
type Answer = usize;

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
    #[cfg(feature = "visualize")]
    let mut all_peaks = HashSet::new();

    model
        .trailheads
        .iter()
        .map(|trailhead| {
            let mut peaks = HashSet::new();

            #[cfg(feature = "visualize")]
            let mut steps = [*trailhead].into();

            model.search(
                *trailhead,
                &mut peaks,
                #[cfg(feature = "visualize")]
                &mut steps,
                #[cfg(feature = "visualize")]
                &mut all_peaks,
            );
            peaks.iter().unique().count()
        })
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
    fn search(
        &self,
        loc: Point<2>,
        mut peaks: &mut HashSet<Point<2>>,
        #[cfg(feature = "visualize")] mut steps: &mut HashSet<Point<2>>,
        #[cfg(feature = "visualize")] mut all_peaks: &mut HashSet<Point<2>>,
    ) {
        let cur = self.topography.getp(loc).unwrap();

        #[cfg(feature = "visualize")]
        {
            std::thread::sleep_ms(10);
            println!("{}", termion::clear::All);
            let mut output = format!("{}", self.topography);
            for (y, line) in output.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    let current_loc = loc.x() == x as i64 && loc.y() == y as i64;

                    let whitebg = color::Bg(color::LightWhite);
                    let grey = color::Fg(color::LightBlack);
                    let greybg = color::Bg(color::LightBlack);
                    let blackfg = color::Fg(color::Black);
                    let reset = style::Reset;
                    let green = color::Bg(color::Green);

                    if (current_loc && cur == 9) || all_peaks.contains(&[x, y].into()) {
                        print!("{green}{c}{reset}",);
                    } else if current_loc {
                        print!("{whitebg}{blackfg}{c}{reset}",);
                    } else if steps.contains(&[x, y].into()) {
                        print!("{greybg}{blackfg}{c}{reset}",);
                    } else {
                        print!("{grey}{c}{reset}",);
                    }
                }

                println!();
            }
        }

        if cur == 9 {
            peaks.insert(loc);

            #[cfg(feature = "visualize")]
            all_peaks.insert(loc);
            #[cfg(feature = "visualize")]
            std::thread::sleep_ms(10);
        } else {
            let moves = self.topography.adj_4(loc);

            let mut score = 0;

            moves.cells.into_iter().flatten().for_each(|cell| {
                if cur + 1 == cell.data {
                    #[cfg(feature = "visualize")]
                    let mut steps = {
                        let mut steps = steps.clone();
                        steps.insert(cell.pos);
                        steps
                    };

                    self.search(
                        cell.pos,
                        peaks,
                        #[cfg(feature = "visualize")]
                        &mut steps,
                        #[cfg(feature = "visualize")]
                        all_peaks,
                    );
                }
            });
        }
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
