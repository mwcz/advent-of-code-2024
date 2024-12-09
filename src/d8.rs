//! A solution to day 8 year 2024.
//! https://adventofcode.com/2024/day/8

use std::collections::{HashMap, HashSet};

use crate::point::Point;

type Model = (HashMap<char, Vec<Point<2>>>, Point<2>);
type Answer = usize;

pub fn parse(input: String) -> Model {
    let mut map: HashMap<char, Vec<Point<2>>> = HashMap::new();

    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c)
                    .and_modify(|points| points.push([x, y].into()))
                    .or_insert_with(|| vec![[x, y].into()]);
            }
            width = x;
        }
        height = y
    }

    (map, [width + 1, height + 1].into())
}

pub fn part1((map, size): Model) -> Answer {
    let mut antinodes: HashSet<Point<2>> = HashSet::new();

    for (ant, locs) in &map {
        for loc1 in locs {
            for loc2 in locs {
                // skip processing self
                if loc1 != loc2 {
                    let dist = *loc1 - *loc2;

                    let anti = *loc1 + dist;
                    // dbg!("PAIR", loc1, loc2, dist, anti);

                    if anti.x() >= 0 && anti.y() >= 0 && anti.x() < size.x() && anti.y() < size.y()
                    {
                        antinodes.insert(anti);
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub fn part2((map, size): Model) -> Answer {
    let mut antinodes: HashSet<Point<2>> = HashSet::new();

    for (ant, locs) in &map {
        for loc1 in locs {
            for loc2 in locs {
                // skip processing self
                if loc1 != loc2 {
                    // antennas are antinodes too (here we already know there are >1 of this freq)
                    antinodes.insert(*loc1);
                    antinodes.insert(*loc2);

                    // apply "lines"

                    let dist = *loc1 - *loc2;
                    let mut anti = *loc1 + dist;
                    loop {
                        if anti.x() >= 0
                            && anti.y() >= 0
                            && anti.x() < size.x()
                            && anti.y() < size.y()
                        {
                            antinodes.insert(anti);
                        } else {
                            break;
                        }

                        anti = anti + dist;
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d8");
    const EXAMPLE: &str = include_str!("../examples/d8");

    #[test]
    fn d8p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 14);
    }

    #[test]
    fn d8p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 336);
    }

    #[test]
    fn d8p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 34);
    }

    #[test]
    fn d8p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 1131);
    }
}
