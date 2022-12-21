use std::collections::HashMap;

type MonkeyName = [u8; 4];

struct CachedOperation {
    op: Operation,
    value: Option<i64>, // None if computation involves "humn"
}

#[derive(Clone)]
enum Operation {
    Int(i64),
    Add(MonkeyName, MonkeyName),
    Sub(MonkeyName, MonkeyName),
    Mul(MonkeyName, MonkeyName),
    Div(MonkeyName, MonkeyName),
}

enum OpResult<'a> {
    Int(i64),
    Op(&'a MonkeyName, &'a MonkeyName, fn(i64, i64) -> i64),
}

impl Operation {
    fn as_fn(&self) -> OpResult {
        match self {
            Operation::Int(i) => OpResult::Int(*i),
            Operation::Add(a, b) => OpResult::Op(a, b, <i64 as std::ops::Add<i64>>::add),
            Operation::Sub(a, b) => OpResult::Op(a, b, <i64 as std::ops::Sub<i64>>::sub),
            Operation::Mul(a, b) => OpResult::Op(a, b, <i64 as std::ops::Mul<i64>>::mul),
            Operation::Div(a, b) => OpResult::Op(a, b, <i64 as std::ops::Div<i64>>::div),
        }
    }
}

fn parse_line(line: String) -> (MonkeyName, CachedOperation) {
    let (name, rest) = line.split_once(": ").unwrap();
    let op = if rest.len() == 11 {
        (match rest.as_bytes()[5] {
            b'+' => Operation::Add,
            b'-' => Operation::Sub,
            b'*' => Operation::Mul,
            b'/' => Operation::Div,
            _ => unreachable!(),
        })(
            rest.as_bytes()[0..4].try_into().unwrap(),
            rest.as_bytes()[7..11].try_into().unwrap(),
        )
    } else {
        Operation::Int(rest.parse::<i64>().unwrap())
    };

    (
        name.as_bytes().try_into().unwrap(),
        CachedOperation { op, value: None },
    )
}

fn eval_monkey(
    monkeys: &mut HashMap<MonkeyName, CachedOperation>,
    name: &MonkeyName,
) -> (i64, Option<i64>) {
    let (val, cache) = match monkeys[name].op.clone().as_fn() {
        OpResult::Int(i) => (i, if name == b"humn" { None } else { Some(i) }),
        OpResult::Op(a, b, op) => {
            let (a_val, a_cache) = eval_monkey(monkeys, a);
            let (b_val, b_cache) = eval_monkey(monkeys, b);
            let val = op(a_val, b_val);
            (
                val,
                if a_cache.is_some() && b_cache.is_some() {
                    Some(val)
                } else {
                    None
                },
            )
        }
    };
    monkeys.entry(*name).and_modify(|v| v.value = cache);
    (val, cache)
}

fn find_equality_input(
    monkeys: &HashMap<MonkeyName, CachedOperation>,
    name: &MonkeyName,
    target: i64,
) -> i64 {
    if name == b"humn" {
        return target;
    }
    let (a, b, _) = match monkeys[name].op.as_fn() {
        OpResult::Op(a, b, op) => (a, b, op),
        OpResult::Int(_) => unreachable!(),
    };
    let mut unknown_operand = b;
    let mut known_operand = a;
    let known_result = monkeys[a].value.unwrap_or_else(|| {
        unknown_operand = a;
        known_operand = b;
        monkeys[b].value.unwrap()
    });
    match (&monkeys[name].op, unknown_operand == a) {
        (Operation::Add(_, _), _) => {
            find_equality_input(monkeys, unknown_operand, target - known_result)
        }
        (Operation::Mul(_, _), _) => {
            find_equality_input(monkeys, unknown_operand, target / known_result)
        }
        (Operation::Sub(_, _), true) => {
            find_equality_input(monkeys, unknown_operand, target + known_result)
        }
        (Operation::Sub(_, _), false) => {
            find_equality_input(monkeys, unknown_operand, known_result - target)
        }
        (Operation::Div(_, _), true) => {
            find_equality_input(monkeys, unknown_operand, target * known_result)
        }
        (Operation::Div(_, _), false) => {
            find_equality_input(monkeys, unknown_operand, known_result / target)
        }
        (Operation::Int(_), _) => unreachable!(),
    }
}

fn find_humn_input(monkeys: &HashMap<MonkeyName, CachedOperation>) -> i64 {
    let (a, b) = match monkeys[b"root"].op.as_fn() {
        OpResult::Op(a, b, _) => (a, b),
        OpResult::Int(_) => unreachable!(),
    };
    let mut unknown_operand = b;
    let known_result = monkeys[a].value.unwrap_or_else(|| {
        unknown_operand = a;
        monkeys[b].value.unwrap()
    });
    find_equality_input(monkeys, unknown_operand, known_result)
}

fn main() {
    let mut monkeys = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .collect::<HashMap<_, _>>();
    println!("{}", eval_monkey(&mut monkeys, b"root").0);
    println!("{}", find_humn_input(&monkeys));
}
