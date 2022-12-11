#![feature(iter_array_chunks, iterator_try_collect)]

use std::collections::VecDeque;
type Item = usize;

enum Op {
    Square,
    Add(usize),
    Multiply(usize),
}

impl Op {
    fn apply(&self, item: Item, worried: bool) -> Item {
        (match &self {
            Op::Square => item * item,
            Op::Add(v) => item + v,
            Op::Multiply(v) => item * v,
        }) / (if worried { 1 } else { 3 })
    }
}

struct Action {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl Action {
    fn get_target(&self, item: Item) -> usize {
        if item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

struct Monkey {
    operation: Op,
    action: Action,
}

struct Throw {
    item: Item,
    to: usize,
}

impl Monkey {
    fn throw_one_item(&self, item: Item, common_divisor: usize, worried: bool) -> Throw {
        let item = self.operation.apply(item, worried) % common_divisor;
        Throw {
            item,
            to: self.action.get_target(item),
        }
    }
}

fn parse_monkey(value: [String; 7]) -> Result<(Monkey, VecDeque<Item>), &'static str> {
    if !value[0].starts_with("Monkey ") {
        return Err("No monkey");
    }
    let current_items = value[1]
        .strip_prefix("  Starting items: ")
        .ok_or("No items")?
        .split(", ")
        .map(str::parse::<Item>)
        .try_collect()
        .map_err(|_| "Invalid item")?;
    let operation = {
        let op = value[2]
            .strip_prefix("  Operation: new = old ")
            .ok_or("No op")?;
        if op == "* old" {
            Op::Square
        } else if let Some(v) = op.strip_prefix("* ") {
            Op::Multiply(str::parse::<usize>(v).map_err(|_| "invalid mult")?)
        } else if let Some(v) = op.strip_prefix("+ ") {
            Op::Add(str::parse::<usize>(v).map_err(|_| "invalid add")?)
        } else {
            return Err("Invalid operation");
        }
    };
    let parse_trailing_int =
        |line: &str, prefix, invalid_prefix_error, parse_error| -> Result<usize, &'static str> {
            line.strip_prefix(prefix)
                .ok_or(invalid_prefix_error)?
                .parse::<usize>()
                .map_err(|_| parse_error)
        };
    let divisible_by = parse_trailing_int(
        &value[3],
        "  Test: divisible by ",
        "Invalid test",
        "Invalid divisor",
    )?;
    let if_true = parse_trailing_int(
        &value[4],
        "    If true: throw to monkey ",
        "Invalid if_true",
        "Invalid monkey",
    )?;
    let if_false = parse_trailing_int(
        &value[5],
        "    If false: throw to monkey ",
        "Invalid if_false",
        "Invalid monkey",
    )?;
    Ok((
        Monkey {
            operation,
            action: Action {
                divisible_by,
                if_true,
                if_false,
            },
        },
        current_items,
    ))
}

fn run_one_round(
    monkeys: &[Monkey],
    starting_items: &mut [VecDeque<Item>],
    items_inspected: &mut [usize],
    common_divisor: usize,
    worried: bool,
) {
    for i in 0..monkeys.len() {
        while let Some(Throw { item, to }) = starting_items[i]
            .pop_front()
            .map(|item| monkeys[i].throw_one_item(item, common_divisor, worried))
        {
            items_inspected[i] += 1;
            starting_items[to].push_back(item);
        }
    }
}

fn run_all_rounds(
    monkeys: &[Monkey],
    mut starting_items: Vec<VecDeque<Item>>,
    num_rounds: usize,
    common_divisor: usize,
    worried: bool,
) -> usize {
    let mut items_inspected = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        run_one_round(
            monkeys,
            &mut starting_items,
            &mut items_inspected,
            common_divisor,
            worried,
        );
    }
    items_inspected.sort();
    items_inspected[items_inspected.len() - 1] * items_inspected[items_inspected.len() - 2]
}

fn main() {
    let (monkeys, starting_items): (Vec<_>, Vec<_>) = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .array_chunks::<7>()
        .map(parse_monkey)
        .map(Result::unwrap)
        .unzip();
    let common_divisor = monkeys
        .iter()
        .map(|m| m.action.divisible_by)
        .product::<usize>();
    println!(
        "{}",
        run_all_rounds(&monkeys, starting_items.clone(), 20, common_divisor, false)
    );
    println!(
        "{}",
        run_all_rounds(&monkeys, starting_items, 10000, common_divisor, true)
    );
}
