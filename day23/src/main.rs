use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    NORTH,
    SOUTH,
    WEST,
    // Defaults to the one before north.
    #[default]
    EAST,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        }
    }

    fn checks(self) -> [(i64, i64); 3] {
        match self {
            Direction::NORTH => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::SOUTH => [(1, -1), (1, 0), (1, 1)],
            Direction::WEST => [(-1, -1), (0, -1), (1, -1)],
            Direction::EAST => [(-1, 1), (0, 1), (1, 1)],
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Elf {
    last_direction: Direction,
    wants_to_move: bool,
}

fn get_proposals(elves: &mut HashMap<Point, RefCell<Elf>>) -> HashMap<Point, Option<Point>> {
    let mut proposals = HashMap::new();
    for (position, elf) in elves.iter() {
        let next_dir = elf.borrow().last_direction.next();
        elf.borrow_mut().last_direction = next_dir;
        elf.borrow_mut().wants_to_move = false;
        let mut count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if elves.contains_key(&Point {
                    x: position.x + i,
                    y: position.y + j,
                }) {
                    count += 1;
                }
            }
        }
        if count == 1 {
            continue;
        }
        let mut direction = elf.borrow().last_direction;
        'direction: for _ in 0..4 {
            for (dx, dy) in direction.checks() {
                if elves.contains_key(&Point {
                    x: position.x + dx,
                    y: position.y + dy,
                }) {
                    direction = direction.next();
                    continue 'direction;
                }
            }
            elf.borrow_mut().wants_to_move = true;
            let (dx, dy) = direction.checks()[1];
            proposals
                .entry(Point {
                    x: position.x + dx,
                    y: position.y + dy,
                })
                .and_modify(|e| *e = None)
                .or_insert(Some(*position));
            break;
        }
    }
    proposals
}

fn apply_proposals(
    elves: &mut HashMap<Point, RefCell<Elf>>,
    proposals: HashMap<Point, Option<Point>>,
) -> bool {
    let mut has_moved = false;
    for (target, maybe_from) in proposals {
        if let Some(from) = maybe_from {
            let elf = elves.remove(&from).unwrap();
            assert_eq!(elves.insert(target, elf), None);
            has_moved = true;
        }
    }
    has_moved
}

fn try_to_move(elves: &mut HashMap<Point, RefCell<Elf>>) -> bool {
    let proposals = get_proposals(elves);
    apply_proposals(elves, proposals)
}

fn main() {
    let mut elves = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .flat_map(move |(x, l)| {
            l.into_bytes()
                .iter()
                .copied()
                .enumerate()
                .flat_map(move |(y, b)| {
                    if b == b'#' {
                        Some((
                            Point {
                                x: x as i64,
                                y: y as i64,
                            },
                            RefCell::from(Elf::default()),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();
    for _ in 0..10 {
        try_to_move(&mut elves);
    }
    let min_x = elves.keys().map(|p| p.x).min().unwrap();
    let min_y = elves.keys().map(|p| p.y).min().unwrap();
    let max_x = elves.keys().map(|p| p.x).max().unwrap();
    let max_y = elves.keys().map(|p| p.y).max().unwrap();
    println!(
        "{}",
        ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - elves.len()
    );
    let mut counter = 11;
    while try_to_move(&mut elves) {
        counter += 1;
    }
    println!("{}", counter);
}
