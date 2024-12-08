//! A solution to day 7 year 2024.
//! https://adventofcode.com/2024/day/7

use cached::proc_macro::cached;
use itertools::Itertools;
use std::ops::Deref;

type Model = Vec<Eqn>;
type Answer = u64;

pub fn parse(input: String) -> Model {
    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(val, terms)| {
            (
                val.parse().unwrap(),
                terms
                    .split_whitespace()
                    .map(|t| t.parse().unwrap())
                    .collect(),
            )
                .into()
        })
        .collect()
}

pub fn part1(model: Model) -> Answer {
    model
        .into_iter()
        .filter(|eq| eq.check(Op::all_p1().to_vec()))
        .map(|eq| eq.val)
        .sum()
}

pub fn part2(model: Model) -> Answer {
    model
        .into_iter()
        .filter(|eq| eq.check(Op::all_p2().to_vec()))
        .map(|eq| eq.val)
        .sum()
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn all_p1() -> [Op; 2] {
        [Op::Add, Op::Mul]
    }

    fn all_p2() -> [Op; 3] {
        [Op::Add, Op::Mul, Op::Concat]
    }

    fn exec(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Concat => a * (10u64.pow(b.ilog10() + 1)) + b,
        }
    }
}

/// generate all n-length variations of ops
#[cached]
fn perms(n: usize, starting_ops: Vec<Op>) -> Vec<Vec<Op>> {
    let mut ops = starting_ops.iter().cloned().map(|o| vec![o]).collect_vec();

    for _ in 1..n {
        let new_ops: Vec<Vec<Op>> = ops
            .drain(..)
            .flat_map(|mut series| {
                starting_ops
                    .iter()
                    .cloned()
                    .map(|o| series.iter().cloned().chain([o].into_iter()).collect())
                    .collect::<Vec<Vec<Op>>>()
                // let mut a = series.clone();
                // a.push(Op::Add);
                // series.push(Op::Mul);
                // vec![a, series]
            })
            .collect();

        ops = new_ops;
    }

    ops
}

#[derive(Debug)]
pub struct Eqn {
    val: u64,
    terms: Vec<u64>,
}

impl Eqn {
    fn check(&self, starting_ops: Vec<Op>) -> bool {
        let op_perms = perms(self.terms.len() - 1, starting_ops);

        for ops in op_perms {
            let mut opsi = ops.iter();
            let out = self
                .terms
                .iter()
                .cloned()
                .reduce(|acc, n| opsi.next().unwrap().exec(acc, n))
                .unwrap();

            if out == self.val {
                return true;
            }
        }

        false
    }
}

impl From<(u64, Vec<u64>)> for Eqn {
    fn from((val, terms): (u64, Vec<u64>)) -> Self {
        Eqn { val, terms }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d7");
    const EXAMPLE: &str = include_str!("../examples/d7");

    #[test]
    fn d7p2_concat_test() {
        assert_eq!(Op::Concat.exec(12, 345), 12345);
        assert_eq!(Op::Concat.exec(15, 6), 156);
        assert_eq!(Op::Concat.exec(1, 1), 11);
    }

    #[test]
    fn d7p2_problem_test() {
        assert!(Eqn {
            val: 7290,
            terms: vec![6, 8, 6, 15]
        }
        .check(Op::all_p2().to_vec()));
    }

    #[test]
    fn d7p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 3749);
    }

    #[test]
    fn d7p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 5837374519342);
    }

    #[test]
    fn d7p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 11387);
    }

    // #[test]
    // fn d7p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
