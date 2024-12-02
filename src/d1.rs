//! A solution to day 1 year 2024.
//! https://adventofcode.com/2024/day/1

use std::collections::HashMap;

type Model = (Vec<u32>, Vec<u32>);
type Answer = u32;

pub fn parse(input: String) -> Model {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    input.lines().for_each(|l| {
        let nums = l.split_once("   ").unwrap();
        col1.push(nums.0.parse().unwrap());
        col2.push(nums.1.parse().unwrap());
    });

    (col1, col2)
}

pub fn part1(mut model: Model) -> Answer {
    model.0.sort();
    model.1.sort();

    let col1 = model.0.iter();
    let col2 = model.1.iter();

    let dist = col1
        .into_iter()
        .zip(col2)
        .fold(0, |acc, (n1, n2)| acc + n1.abs_diff(*n2));

    dist
}

pub fn part2(model: Model) -> Answer {
    let mut counts = HashMap::new();

    model.1.iter().for_each(|n| {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    });

    model
        .0
        .iter()
        .map(|n| n * counts.get(n).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d1");
    const EXAMPLE: &str = include_str!("../examples/d1");

    #[test]
    fn d1p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 11);
    }

    #[test]
    fn d1p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 1882714);
    }

    #[test]
    fn d1p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 31);
    }

    #[test]
    fn d1p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 19437052);
    }
}
