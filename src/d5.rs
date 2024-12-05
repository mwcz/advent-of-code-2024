//! A solution to day 5 year 2024.
//! https://adventofcode.com/2024/day/5

use std::{cmp::Ordering, collections::HashSet};

type Model = PrintPlan;
type Answer = u32;

pub fn parse(input: String) -> Model {
    let (ord, prod) = input.split_once("\n\n").unwrap();

    let ord = ord
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let prod = prod
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Model { ord, print: prod }
}

pub fn part1(model: Model) -> Answer {
    model
        .correct_print_runs()
        .map(|run| run[run.len() / 2])
        .sum()
}

pub fn part2(model: Model) -> Answer {
    model.fixed_print_runs().map(|run| run[run.len() / 2]).sum()
}

#[derive(Debug)]
pub struct PrintPlan {
    ord: HashSet<(u32, u32)>,
    print: Vec<Vec<u32>>,
}

impl Model {
    fn in_order(&self, pages: &[u32]) -> bool {
        if pages.len() <= 1 {
            return true;
        }

        if !self.ord.contains(&(pages[0], pages[1])) {
            return false;
        }

        self.in_order(&pages[1..])
    }

    fn correct_print_runs(&self) -> impl Iterator<Item = &[u32]> {
        self.print
            .iter()
            .filter(|&run| self.in_order(run))
            .map(|run| run.as_slice())
    }

    fn fixed_print_runs(&self) -> impl Iterator<Item = Vec<u32>> + use<'_> {
        self.print
            .iter()
            .filter(|&run| !self.in_order(run))
            .map(|run| self.fix_order(run))
    }

    fn fix_order(&self, pages: &[u32]) -> Vec<u32> {
        let mut sorted = pages.to_vec();

        sorted.sort_by(|a, b| {
            if self.ord.contains(&(*a, *b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d5");
    const EXAMPLE: &str = include_str!("../examples/d5");

    #[test]
    fn d5p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 143);
    }

    #[test]
    fn d5p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 5452);
    }

    #[test]
    fn d5p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 123);
    }

    #[test]
    fn d5p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 4598);
    }
}
