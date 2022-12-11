#![feature(iter_array_chunks, iterator_try_collect)]
type Item = usize;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Monkey {
    current_items: Vec<Item>,
    operation: Op,
    action: Action,
}

struct Throw {
    item: Item,
    to: usize,
}

impl Monkey {
    fn throw_one_item(&mut self, common_divisor: usize, worried: bool) -> Option<Throw> {
        let item = self.operation.apply(*self.current_items.first()?, worried) % common_divisor;
        self.current_items.remove(0);
        Some(Throw {
            item,
            to: self.action.get_target(item),
        })
    }
}

impl TryFrom<[String; 7]> for Monkey {
    type Error = &'static str;

    fn try_from(value: [String; 7]) -> Result<Self, Self::Error> {
        if !value[0].starts_with("Monkey ") {
            return Err("No monkey");
        }
        let current_items = value[1]
            .strip_prefix("  Starting items: ")
            .ok_or("No items")?
            .split(", ")
            .map(str::parse::<Item>)
            .try_collect::<Vec<_>>()
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
        let divisible_by = value[3]
            .strip_prefix("  Test: divisible by ")
            .ok_or("Invalid test")?
            .parse::<usize>()
            .map_err(|_| "Invalid divisor")?;
        let if_true = value[4]
            .strip_prefix("    If true: throw to monkey ")
            .ok_or("Invalid if_true")?
            .parse::<usize>()
            .map_err(|_| "Invalid monkey")?;
        let if_false = value[5]
            .strip_prefix("    If false: throw to monkey ")
            .ok_or("Invalid if_false")?
            .parse::<usize>()
            .map_err(|_| "Invalid monkey")?;
        Ok(Monkey {
            current_items,
            operation,
            action: Action {
                divisible_by,
                if_true,
                if_false,
            },
        })
    }
}

fn run_one_round(
    monkeys: &mut Vec<Monkey>,
    items_inspected: &mut Vec<usize>,
    common_divisor: usize,
    worried: bool,
) {
    for i in 0..monkeys.len() {
        while let Some(Throw { item, to }) = monkeys[i].throw_one_item(common_divisor, worried) {
            items_inspected[i] += 1;
            monkeys[to].current_items.push(item);
        }
    }
}

fn run_all_rounds(
    mut monkeys: Vec<Monkey>,
    num_rounds: usize,
    common_divisor: usize,
    worried: bool,
) -> usize {
    let mut items_inspected = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        run_one_round(&mut monkeys, &mut items_inspected, common_divisor, worried);
    }
    items_inspected.sort();
    items_inspected[items_inspected.len() - 1] * items_inspected[items_inspected.len() - 2]
}

fn main() {
    let monkeys = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .array_chunks::<7>()
        .map(Monkey::try_from)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let common_divisor = monkeys
        .iter()
        .map(|m| m.action.divisible_by)
        .product::<usize>();
    println!(
        "{}",
        run_all_rounds(monkeys.clone(), 20, common_divisor, false)
    );
    println!("{}", run_all_rounds(monkeys, 10000, common_divisor, true));
}
