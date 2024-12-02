//! A solution to day 2 year 2024.
//! https://adventofcode.com/2024/day/2

type Model = Vec<Vec<i32>>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    input.lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect()
}

pub fn part1(model: Model) -> Answer {
    model.into_iter().filter_map(is_safe::<false>).count()
}

fn is_safe<const DAMP: bool>(report: Vec<i32>) -> Option<()> {
    // asc or dec
    let mut dir = 0;

    for (i, pair) in report.windows(2).enumerate() {
        let a = pair[0];
        let ai = i;
        let b = pair[1];
        let bi = i + 1;

        if a == b {
            return problem(&report, DAMP);
        }

        let this_dir = (b-a) / (b-a).abs();

        // initialize dir
        if dir == 0 {
            dir = this_dir;
        }

        if dir != this_dir {
            return problem(&report, DAMP);
        }

        if (b-a).abs() > 3 {
            return problem(&report, DAMP);
        }
    }

    Some(())
}

fn problem(report: &Vec<i32>, damp: bool) -> Option<()> {
            if damp  {
                return dampen(&report).into_iter().any(|r| is_safe::<false>(r).is_some()).then_some(());
            } else {
                return None;
            }
}

fn dampen(report: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut reports = Vec::with_capacity(report.len());

    for i in 0..report.len() {
        let mut rep = report.clone();
        rep.remove(i);
        reports.push(rep);
    }

    reports
}

pub fn part2(model: Model) -> Answer {
    model.into_iter().filter_map(|r| {
        let safety = is_safe::<true>(r);
        safety
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d2");
    const EXAMPLE: &str = include_str!("../examples/d2");

    #[test]
    fn d2p1_example_test() {
        assert_eq!(
            part1(parse(EXAMPLE.to_string())),
            2
        );
    }

    #[test]
    fn d2p1_input_test() {
        assert_eq!(
            part1(parse(INPUT.to_string())),
            572
        );
    }

    #[test]
    fn d2p2_example_test() {
        assert_eq!(
            part2(parse(EXAMPLE.to_string())),
            4
        );
    }

    #[test]
    fn d2p2_input_test() {
        assert_eq!(
            part2(parse(INPUT.to_string())),
            0
        );
    }
}
