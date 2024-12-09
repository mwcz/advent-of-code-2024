//! A solution to day 6 year 2024.
//! https://adventofcode.com/2024/day/6

#![warn(unused)]

use std::{collections::HashSet, fmt::Display};

use crate::{direction::CardDir, grid::Grid, point::Point};

type Model = Map;
type Answer = usize;

pub fn parse(input: String) -> Model {
    let mut guard: (Point<2>, CardDir) = ([0usize, 0].into(), CardDir::Up);

    let cells = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let spot = Spot::from(c);
                    if let Spot::Guard(d) = spot {
                        guard.0.coords = [x as i64, y as i64];
                        guard.1 = d;
                        Spot::Empty
                    } else {
                        spot
                    }
                })
                .collect()
        })
        .collect();

    Map {
        grid: Grid::new(cells),
        visited: [guard].into(),
        guard,
    }
}

pub fn part1(mut model: Model) -> Answer {
    while model.next().is_some() {
        // println!("\n\n{model}");
        // std::thread::sleep_ms(100);
    }

    model
        .visited
        .iter()
        .map(|p| p.0)
        .collect::<HashSet<Point<2>>>()
        .len()
}

pub fn part2(mut model: Model) -> Answer {
    let model2 = model.clone();

    while model.next().is_some() {}

    let mut obs: HashSet<(usize, usize)> = HashSet::new();

    for pair in model
        .visited
        .into_iter()
        .skip(1) // skip guard pos
        .collect::<Vec<(Point<2>, CardDir)>>()
    {
        let mut submodel = model2.clone();
        submodel.grid.setp(pair.0, Spot::Obstacle);

        while let Some(seen) = submodel.next() {
            // std::thread::sleep_ms(2);
            // println!("\n\n{submodel}");
            if seen {
                obs.insert((pair.0.x() as usize, pair.0.y() as usize));
                break;
            }
        }
    }

    obs.len()
}

/// returns true if the model produces an infinite loop
pub fn search_p2(
    mut model: Model,
    looping_obs: &mut HashSet<Point<2>>,
    obs: Option<Point<2>>,
    obs_list: HashSet<Point<2>>,
) {
    // reset visited list
    // model.visited = [model.guard].into();

    while let Some(seen) = model.next() {
        // std::thread::sleep_ms(20);
        if seen {
            if let Some(obs) = obs {
                println!("{model}");
                looping_obs.insert(obs);
                return;
            }
        } else if obs.is_none() {
            let mut new_model = model.clone();
            if let Some((new_obs, list)) = new_model.place_obstacle(&obs_list) {
                search_p2(new_model, looping_obs, Some(new_obs), list);
            }
        }

        // println!("\n\n{model}");
        // std::thread::sleep_ms(100);
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    grid: Grid<Spot>,
    visited: HashSet<(Point<2>, CardDir)>,
    guard: (Point<2>, CardDir),
}

#[derive(Copy, Clone, Debug)]
pub enum Spot {
    Guard(CardDir),
    Empty,
    Obstacle,
}

impl Map {
    /// returns Some(true) if the position+direction was already seen, implying an infinite loop.
    /// Some(false) means it's a new position+direction.
    /// None means we left the grid.
    fn next(&mut self) -> Option<bool> {
        let new_pos = self.guard.0.move_in_grid(self.guard.1, &self.grid)?;
        let new_spot = self.grid.getp(new_pos)?;

        match new_spot {
            Spot::Obstacle => self.guard.1 = self.guard.1.cw(),
            Spot::Empty => {
                self.guard.0 = new_pos;
            }
            _ => (),
        }

        let seen = self.visited.contains(&self.guard);

        self.visited.insert(self.guard);

        Some(seen)
    }

    /// try to put an obstacle in front of the guard if the spot is empty.  
    fn place_obstacle(
        &mut self,
        obs_list: &HashSet<Point<2>>,
    ) -> Option<(Point<2>, HashSet<Point<2>>)> {
        if let Some(new_pos) = self.guard.0.move_in_grid(self.guard.1, &self.grid) {
            if matches!(self.grid.getp(new_pos), Some(Spot::Empty)) && !obs_list.contains(&new_pos)
            {
                self.grid.setp(new_pos, Spot::Obstacle);
                let mut new_list = obs_list.clone();
                new_list.insert(new_pos);
                Some((new_pos, new_list))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl From<char> for Spot {
    fn from(value: char) -> Self {
        match value {
            '^' | '<' | '>' | 'v' => Spot::Guard(CardDir::from(value)),
            '#' => Spot::Obstacle,
            _ => Spot::Empty,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.grid.cells.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if self.guard.0 == [x, y].into() {
                    write!(
                        f,
                        "{}",
                        match self.guard.1 {
                            CardDir::Up => "^",
                            CardDir::Down => "v",
                            CardDir::Left => "<",
                            CardDir::Right => ">",
                        }
                    )?;
                } else if let Spot::Guard(_dir) = c {
                } else if self.visited.contains(&([x, y].into(), CardDir::Up))
                    || self.visited.contains(&([x, y].into(), CardDir::Right))
                    || self.visited.contains(&([x, y].into(), CardDir::Down))
                    || self.visited.contains(&([x, y].into(), CardDir::Left))
                {
                    write!(f, "+")?;
                } else {
                    match c {
                        Spot::Guard(_card_dir) => write!(f, "!")?,
                        Spot::Empty => write!(f, ".")?,
                        Spot::Obstacle => write!(f, "#")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d6");
    const EXAMPLE: &str = include_str!("../examples/d6");

    #[test]
    fn d6p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 41);
    }

    #[test]
    fn d6p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 4778);
    }

    #[test]
    fn d6p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 6);
    }

    #[test]
    fn d6p2_input_slow_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 1618);
    }
}
