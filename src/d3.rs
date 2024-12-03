//! A solution to day 3 year 2024.
//! https://adventofcode.com/2024/day/3

type Model = String;
type Answer = u64;

pub fn parse(input: String) -> Model {
    input
}

pub fn part1(model: Model) -> Answer {
    let mut sm = StateMachine::new(false);
    sm.process(model)
}

pub fn part2(model: Model) -> Answer {
    let mut sm = StateMachine::new(true);
    sm.process(model)
}

#[derive(Debug)]
struct StateMachine {
    state: State,
    prev_state: State,
    num1: u64,
    num2: u64,
    conds: bool,
    mul_enabled: bool,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum State {
    Scanning,
    M,
    U,
    L,
    LeftParen,
    Num1,
    Comma,
    Num2,
    D,
    O,
    N,
    Apos,
    T,
}

impl StateMachine {
    /// conds controls whether conditional statements are enabled
    fn new(conds: bool) -> StateMachine {
        StateMachine {
            state: State::Scanning,
            prev_state: State::Scanning,
            num1: 0,
            num2: 0,
            conds,
            mul_enabled: true,
        }
    }

    fn process(&mut self, input: Model) -> u64 {
        let mut sum = 0;

        for c in input.chars() {
            if let Some(vals) = self.feed(c) {
                sum += vals.0 * vals.1;
            }
        }

        sum
    }

    fn feed(&mut self, c: char) -> Option<(u64, u64)> {
        use State::*;

        let prev_state = self.state;

        self.state = match self.state {
            // scan for mul
            Scanning if (!self.conds || self.mul_enabled) && c == 'm' => M,
            M if c == 'u' => U,
            U if c == 'l' => L,
            L if c == '(' => LeftParen,
            LeftParen if c.is_ascii_digit() => {
                self.num1 = self.num1 * 10 + c.to_digit(10).unwrap() as u64;
                Num1
            }
            Num1 if c.is_ascii_digit() => {
                self.num1 = self.num1 * 10 + c.to_digit(10).unwrap() as u64;
                Num1
            }
            Num1 if c == ',' => Comma,
            Comma if c.is_ascii_digit() => {
                self.num2 = self.num2 * 10 + c.to_digit(10).unwrap() as u64;
                Num2
            }
            Num2 if c.is_ascii_digit() => {
                self.num2 = self.num2 * 10 + c.to_digit(10).unwrap() as u64;
                Num2
            }
            Num2 if c == ')' => {
                let vals = (self.num1, self.num2);
                self.reset();
                return Some(vals);
            }

            // scan for do/don't
            Scanning if c == 'd' => D,
            D if c == 'o' => O,
            O if c == '(' => LeftParen,
            LeftParen if c == ')' => {
                self.mul_enabled = self.prev_state == O;
                Scanning
            }
            O if c == 'n' => N,
            N if c == '\'' => Apos,
            Apos if c == 't' => T,
            T if c == '(' => LeftParen,

            // reset otherwise
            _ => {
                self.reset();
                Scanning
            }
        };

        self.prev_state = prev_state;

        None
    }

    fn reset(&mut self) {
        self.state = State::Scanning;
        self.prev_state = State::Scanning;
        self.num1 = 0;
        self.num2 = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d3");
    const EXAMPLE: &str = include_str!("../examples/d3");
    const EXAMPLE_P2: &str = include_str!("../examples/d3-p2");

    #[test]
    fn d3p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 161);
    }

    #[test]
    fn d3p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 196826776);
    }

    #[test]
    fn d3p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE_P2.to_string())), 48);
    }

    #[test]
    fn d3p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 106780429);
    }
}
