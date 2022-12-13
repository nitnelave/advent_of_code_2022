#![feature(iter_array_chunks)]

type Int = u8;

#[derive(PartialEq, Eq, Clone)]
enum Node {
    Int(Int),
    List(Vec<Node>),
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (Node::Int(a), Node::Int(b)) => a.cmp(b),
            (Node::Int(_), Node::List(l)) if l.is_empty() => Ordering::Greater,
            (Node::Int(a), Node::List(l)) => [Node::Int(*a)].as_slice().cmp(l.as_slice()),
            (Node::List(l), Node::Int(_)) if l.is_empty() => Ordering::Less,
            (Node::List(l), Node::Int(b)) => l.as_slice().cmp([Node::Int(*b)].as_slice()),
            (Node::List(l1), Node::List(l2)) => l1.cmp(l2),
        }
    }
}

impl<'a> TryFrom<&'a str> for Node {
    type Error = &'static str;
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        fn parse_one_node(mut line: &[u8]) -> Result<(Node, &[u8]), &'static str> {
            if line.is_empty() {
                return Err("Input too short");
            }
            if line[0] == b'[' {
                if line[1] == b']' {
                    return Ok((Node::List(vec![]), &line[2..]));
                }
                let mut nodes = vec![];
                loop {
                    let (node, rest) = parse_one_node(&line[1..])?;
                    nodes.push(node);
                    if rest[0] == b',' {
                        line = &rest[0..];
                    } else if rest[0] == b']' {
                        break Ok((Node::List(nodes), &rest[1..]));
                    } else {
                        break Err("Missing closing delimiter");
                    }
                }
            } else if line[0].is_ascii_digit() {
                let mut num = 0;
                let mut i = 0;
                while i < line.len() && line[i].is_ascii_digit() {
                    num *= 10;
                    num += line[i] - b'0';
                    i += 1;
                }
                Ok((Node::Int(num), &line[i..]))
            } else {
                Err("Unexpected input")
            }
        }
        let (node, rest) = parse_one_node(line.as_bytes())?;
        if rest.is_empty() {
            Ok(node)
        } else {
            Err("Leftover input")
        }
    }
}

fn main() {
    let mut nodes = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty())
        .map(|s| Node::try_from(s.as_str()).unwrap())
        .collect::<Vec<_>>();
    println!(
        "{}",
        nodes
            .iter()
            .array_chunks::<2>()
            .enumerate()
            .filter_map(|(i, chunk)| if chunk[0] <= chunk[1] {
                Some(i + 1)
            } else {
                None
            })
            .sum::<usize>()
    );
    let delim_1 = Node::List(vec![Node::List(vec![Node::Int(2)])]);
    let delim_2 = Node::List(vec![Node::List(vec![Node::Int(6)])]);
    nodes.push(delim_1.clone());
    nodes.push(delim_2.clone());
    nodes.sort_unstable();
    let pos_1 = nodes.binary_search(&delim_1).unwrap() + 1;
    let pos_2 = nodes.binary_search(&delim_2).unwrap() + 1;
    println!("{}", pos_1 * pos_2);
}
