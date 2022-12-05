#![feature(slice_take)]
type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

type Crate = u8;

type CrateStack = Vec<Crate>;

#[derive(Clone)]
struct Cargo(Vec<CrateStack>);

impl Cargo {
    fn iter(&self) -> impl Iterator<Item = &CrateStack> {
        self.0.iter()
    }
}

impl std::fmt::Debug for Cargo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(
                self.iter()
                    .map(|stack| std::str::from_utf8(stack.as_slice()).unwrap()),
            )
            .finish()
    }
}

struct CrateLine<'a> {
    input: &'a [Crate],
}

impl<'a> CrateLine<'a> {
    fn new(i: &'a str) -> Self {
        Self {
            input: i.as_bytes(),
        }
    }
}

impl<'a> Iterator for CrateLine<'a> {
    type Item = Result<Option<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        let val = match self.input.take(..3) {
            None => return Some(Err("Too short crate")),
            Some(v) => {
                if v == b"   " {
                    self.input.take_first();
                    return Some(Ok(None));
                } else if v[0] != b'[' || v[2] != b']' {
                    return Some(Err("Invalid crate bounds"));
                } else {
                    v[1]
                }
            }
        };
        if let Some(l) = self.input.take_first() {
            if *l != b' ' {
                return Some(Err("Invalid crate separator"));
            }
        }
        Some(Ok(Some(val)))
    }
}

fn fill_cargo<'a, I: Iterator<Item = CrateLine<'a>>>(
    num_crates: usize,
    crate_lines: I,
) -> Result<Cargo> {
    let mut crates = vec![CrateStack::new(); num_crates];
    for line in crate_lines {
        for (stack, crate_) in std::iter::zip(crates.iter_mut(), line) {
            if let Some(c) = crate_? {
                stack.push(c);
            }
        }
    }
    crates.iter_mut().for_each(|stack| stack.reverse());
    Ok(Cargo(crates))
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_move<S: AsRef<str>>(input: S) -> Move {
    let input = input.as_ref().as_bytes();
    let mut index: usize = 5; // "move "
    let parse_num = |index: &mut usize| {
        let mut num = 0;
        while *index < input.len() && input[*index].is_ascii_digit() {
            num *= 10;
            num += input[*index] - b'0';
            *index += 1;
        }
        num as usize
    };
    let amount = parse_num(&mut index);
    index += 6; // " from "
    let from = parse_num(&mut index) - 1;
    index += 4; // " to "
    let to = parse_num(&mut index) - 1;
    Move { amount, from, to }
}

pub trait SliceExt {
    type Item;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item) {
        use std::cmp::Ordering;
        match index0.cmp(&index1) {
            Ordering::Less => {
                let mut iter = self.iter_mut();
                let item0 = iter.nth(index0).unwrap();
                let item1 = iter.nth(index1 - index0 - 1).unwrap();
                (item0, item1)
            }
            Ordering::Equal => panic!("[T]::get_two_mut(): received same index twice ({})", index0),
            Ordering::Greater => {
                let mut iter = self.iter_mut();
                let item1 = iter.nth(index1).unwrap();
                let item0 = iter.nth(index0 - index1 - 1).unwrap();
                (item0, item1)
            }
        }
    }
}

fn apply_move(cargo: &mut Cargo, mov: &Move) {
    let (from, to) = cargo.0.get_two_mut(mov.from, mov.to);
    for _ in 0..mov.amount {
        to.push(*from.last().unwrap());
        from.pop();
    }
}

fn apply_move_9001(cargo: &mut Cargo, mov: &Move) {
    let from_len = cargo.0[mov.from].len();
    let (from, to) = cargo.0.get_two_mut(mov.from, mov.to);
    to.extend_from_slice(from.as_slice().take((from_len - mov.amount)..).unwrap());
    from.truncate(from_len - mov.amount);
}

fn apply_all_moves<F: Fn(&mut Cargo, &Move)>(mut cargo: Cargo, moves: &[Move], mover: F) -> String {
    moves.iter().for_each(|m| mover(&mut cargo, m));
    let output = cargo
        .iter()
        .map(|stack| stack.last().copied().unwrap_or(b' '))
        .collect::<Vec<_>>();
    String::from_utf8(output).unwrap()
}

fn main() {
    let mut lines = std::io::stdin().lines().map(std::result::Result::unwrap);
    let mut last_line = None;
    let crate_input = lines
        .by_ref()
        .take_while(|l| {
            if l.starts_with('[') {
                true
            } else {
                last_line = Some(l.clone());
                false
            }
        })
        .collect::<Vec<_>>();

    let num_crates = last_line.unwrap().split_ascii_whitespace().count();
    assert!(num_crates > 0);
    assert!(lines.next().unwrap().is_empty());
    let cargo = fill_cargo(
        num_crates,
        crate_input.iter().map(String::as_ref).map(CrateLine::new),
    )
    .unwrap();
    let moves = lines.map(parse_move).collect::<Vec<_>>();
    println!("{}", apply_all_moves(cargo.clone(), &moves, apply_move));
    println!("{}", apply_all_moves(cargo, &moves, apply_move_9001));
}
