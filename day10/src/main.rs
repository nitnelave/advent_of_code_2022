#![feature(iter_intersperse)]
type RegisterValue = i32;
enum Instruction {
    Noop,
    Addx(RegisterValue),
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            Ok(Instruction::Noop)
        } else if let Some(int) = value.strip_prefix("addx ") {
            Ok(Instruction::Addx(
                int.parse::<RegisterValue>().map_err(|_| "Invalid int")?,
            ))
        } else {
            Err("Unknown instruction")
        }
    }
}

enum CpuState {
    Free,
    Computing(RegisterValue),
}

struct RegisterStates<I: Iterator<Item = Instruction>> {
    instructions: I,
    state: CpuState,
    register: RegisterValue,
}

impl<I: Iterator<Item = Instruction>> Iterator for RegisterStates<I> {
    type Item = RegisterValue;

    fn next(&mut self) -> Option<Self::Item> {
        if let CpuState::Computing(mut v) = std::mem::replace(&mut self.state, CpuState::Free) {
            std::mem::swap(&mut self.register, &mut v);
            Some(v)
        } else {
            match self.instructions.next() {
                None => None,
                Some(Instruction::Noop) => Some(self.register),
                Some(Instruction::Addx(v)) => {
                    self.state = CpuState::Computing(self.register + v);
                    Some(self.register)
                }
            }
        }
    }
}

impl<I: Iterator<Item = Instruction>> From<I> for RegisterStates<I> {
    fn from(it: I) -> Self {
        Self {
            instructions: it,
            state: CpuState::Free,
            register: 1,
        }
    }
}

const NUM_ROW: usize = 6;
const NUM_COL: usize = 40;

struct Screen([bool; NUM_ROW * NUM_COL]);

impl Screen {
    fn maybe_set_pixel(&mut self, cycle: usize, register: RegisterValue) {
        self.0[cycle] = ((cycle % NUM_COL) as i32).abs_diff(register) <= 1
    }
}

impl ToString for Screen {
    fn to_string(&self) -> String {
        self.0
            .chunks_exact(NUM_COL)
            .map(|c| {
                c.into_iter()
                    .map(|b| if *b { '#' } else { ' ' })
                    .collect::<String>()
            })
            .intersperse("\n".to_owned())
            .collect::<String>()
    }
}

fn main() {
    let mut screen = Screen([false; NUM_ROW * NUM_COL]);
    println!(
        "{}",
        RegisterStates::<_>::from(
            std::io::stdin()
                .lines()
                .map(Result::unwrap)
                .map(|s| Instruction::try_from(s.as_str()))
                .map(Result::unwrap),
        )
        .enumerate()
        .map(|(i, r)| {
            screen.maybe_set_pixel(i, r);
            (i, r)
        })
        .map(|(i, r)| ((i + 1) as i32, r)) // count from 1
        .filter_map(|(i, r)| if i % 40 == 20 { Some(i * r) } else { None })
        .sum::<i32>()
    );
    println!("{}", screen.to_string());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn short_test() {
        assert_eq!(
            RegisterStates::<_>::from(
                [
                    Instruction::Noop,
                    Instruction::Addx(3),
                    Instruction::Addx(-5),
                    Instruction::Noop,
                    Instruction::Noop,
                    Instruction::Addx(3),
                    Instruction::Noop,
                ]
                .into_iter()
            )
            .collect::<Vec<_>>(),
            vec![1, 1, 1, 4, 4, -1, -1, -1, -1, 2]
        );
    }
}
