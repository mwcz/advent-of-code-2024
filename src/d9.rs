//! A solution to day 9 year 2024.
//! https://adventofcode.com/2024/day/9

type Model = Vec<u64>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

pub fn part1(model: Model) -> Answer {
    // println!("{:?}", model);

    let mut disk: Vec<Block> = Vec::with_capacity(model.iter().sum::<u64>() as usize);
    let mut empties: Vec<usize> = Vec::with_capacity(disk.len());
    let mut fulls: Vec<usize> = Vec::with_capacity(disk.len());

    let mut id = 0;
    let mut total_file_size = 0;

    for pair in model.chunks(2) {
        for _ in 0..pair[0] {
            fulls.push(disk.len());
            disk.push(Block::File(ID(id)));
            total_file_size += 1;
        }
        if pair.len() == 2 {
            for _ in 0..pair[1] {
                empties.push(disk.len());
                disk.push(Block::Empty);
            }
        }
        id += 1;
    }

    empties.reverse();

    // println!("{:?}", disk);
    // println!("total file size {:?}", total_file_size);
    // println!("EMPTY {:?}", empties);
    // println!("FULL {:?}", fulls);

    loop {
        let block_id = fulls.pop().unwrap();
        let empty_id = empties.pop().unwrap();
        if empty_id >= total_file_size {
            break;
        }
        disk.swap(block_id, empty_id);
    }

    // print_disk(&disk);
    score_disk(&disk)
}

fn score_disk(disk: &[Block]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, b)| match b {
            Block::Empty => 0,
            Block::File(id) => id.0 as usize * i,
        })
        .sum()
}

fn print_disk(disk: &[Block]) {
    let msg = disk
        .iter()
        .map(|b| match b {
            Block::Empty => ".".to_string(),
            Block::File(id) => format!("{}", id.0),
        })
        .collect::<Vec<String>>()
        .join(",");
    println!("{msg}");
}

pub fn part2(model: Model) -> Answer {
    0
}

#[derive(Debug)]
enum Block {
    Empty,
    File(ID),
}

#[derive(Debug)]
struct ID(u64);

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const INPUT: &str = include_str!("../input/d9");
//     const EXAMPLE: &str = include_str!("../examples/d9");
//
//     // #[test]
//     // fn d9p1_example_test() {
//     //     assert_eq!(
//     //         part1(parse(EXAMPLE.to_string())),
//     //         1928
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d9p1_input_test() {
//     //     assert_eq!(
//     //         part1(parse(INPUT.to_string())),
//     //         "put part 1 final answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d9p2_example_test() {
//     //     assert_eq!(
//     //         part2(parse(EXAMPLE.to_string())),
//     //         "put part 2 example answer here"
//     //     );
//     // }
//     //
//     // #[test]
//     // fn d9p2_input_test() {
//     //     assert_eq!(
//     //         part2(parse(INPUT.to_string())),
//     //         "put part 2 final answer here"
//     //     );
//     // }
// }
