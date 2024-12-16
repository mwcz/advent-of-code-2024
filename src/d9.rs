//! A solution to day 9 year 2024.
//! https://adventofcode.com/2024/day/9

use std::collections::HashSet;

use termion::clear;

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
    let mut disk: Vec<Block> = Vec::with_capacity(model.iter().sum::<u64>() as usize);
    let mut empties: Vec<usize> = Vec::with_capacity(disk.len());
    let mut fulls: Vec<usize> = Vec::with_capacity(disk.len());

    let mut total_file_size = 0;

    for (id, pair) in model.chunks(2).enumerate() {
        for _ in 0..pair[0] {
            fulls.push(disk.len());
            disk.push(Block::File(ID(id as u64)));
            total_file_size += 1;
        }
        if pair.len() == 2 {
            for _ in 0..pair[1] {
                empties.push(disk.len());
                disk.push(Block::Empty);
            }
        }
    }

    empties.reverse();

    loop {
        let block_id = fulls.pop().unwrap();
        let empty_id = empties.pop().unwrap();
        if empty_id >= total_file_size {
            break;
        }
        disk.swap(block_id, empty_id);
    }

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
    print!("{}", clear::All);
    let msg = disk
        .iter()
        .map(|b| match b {
            Block::Empty => ".",
            Block::File(id) => "#",
        })
        .collect::<Vec<&str>>()
        .join("");
    println!("{}", msg);
    std::thread::sleep_ms(4);
}

pub fn part2(model: Model) -> Answer {
    let mut disk: Vec<Block> = Vec::with_capacity(model.iter().sum::<u64>() as usize);
    // map file ID to block index on disk
    let mut file_locs: Vec<usize> = Vec::with_capacity(disk.len());
    let mut last_id = 0;
    let mut sizes: Vec<u64> = Vec::with_capacity(disk.len());

    for (id, pair) in model.chunks(2).enumerate() {
        file_locs.push(disk.len());
        for _ in 0..pair[0] {
            disk.push(Block::File(ID(id as u64)));
        }
        if pair.len() == 2 {
            for _ in 0..pair[1] {
                disk.push(Block::Empty);
            }
        }

        // sizes[id] = pair[0];
        sizes.push(pair[0]);
        last_id = id;
    }

    // print_disk(&disk);

    for file_id in (0..=last_id).rev() {
        let file_size = sizes[file_id];
        // println!("\nmoving file: {file_id}");

        if let Some(empty_idx) = find_space(&disk, file_size as usize) {
            // println!("found empty space: {empty_idx}");
            let file_idx = file_locs[file_id];
            if empty_idx < file_idx {
                for i in 0..(file_size as usize) {
                    disk.swap(file_idx + i, empty_idx + i);
                }
            }
        }
        // print_disk(&disk);
    }

    // loop {}

    // loop {

    // start at the end of the disk
    // get the ID of the file
    // and iterate towards the front
    // until a block != ID is found
    //
    // capture that length
    //
    // then start at the beginning and find the first Empty
    // check its length, if not long enough then find the next one and check its length, repeat
    // until a long enough Empty chain is found

    // let block_id = fulls.pop().unwrap();
    // let empty_id = empties.pop().unwrap();
    // if empty_id >= total_file_size {
    //     break;
    // }
    // disk.swap(block_id, empty_id);
    // }

    score_disk(&disk)
}

/// Find the beginning of a series of Empty blocks that could fit a file of the given size, if any.
fn find_space(disk: &[Block], size: usize) -> Option<usize> {
    let mut space_so_far = 0;
    let mut start = 0;

    for (i, block) in disk.iter().enumerate() {
        if let Block::Empty = block {
            if space_so_far == 0 {
                start = i;
            }

            space_so_far += 1;

            if space_so_far == size {
                return Some(start);
            }
        } else {
            space_so_far = 0;
        }
    }

    None
}

#[derive(Debug, Clone, Copy)]
enum Block {
    Empty,
    File(ID),
}

#[derive(Debug, Clone, Copy)]
struct ID(u64);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d9");
    const EXAMPLE: &str = include_str!("../examples/d9");

    #[test]
    fn d9p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 1928);
    }

    #[test]
    fn d9p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 6370402949053);
    }

    #[test]
    fn d9p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 2858,);
    }
    #[test]
    fn d9p2_find_space_test() {
        use Block::*;
        assert_eq!(
            find_space(
                &[
                    File(ID(1)),
                    File(ID(1)),
                    Empty,
                    File(ID(2)),
                    Empty,
                    Empty,
                    Empty,
                    File(ID(3))
                ],
                2
            ),
            Some(4)
        );
        assert_eq!(
            find_space(
                &[
                    File(ID(1)),
                    File(ID(1)),
                    Empty,
                    File(ID(2)),
                    Empty,
                    Empty,
                    Empty,
                    File(ID(3))
                ],
                4
            ),
            None
        );
    }

    // #[test]
    // fn d9p2_input_test() {
    //     assert_eq!(
    //         part2(parse(INPUT.to_string())),
    //         "put part 2 final answer here"
    //     );
    // }
}
