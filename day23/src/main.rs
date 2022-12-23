use std::collections::{HashMap, HashSet};

struct Grid {
    cells: Vec<i64>,
    width: i64,
}

fn next_power_of_2(mut v: usize) -> usize {
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    v
}

impl Grid {
    fn new(width: usize) -> Self {
        let h = next_power_of_2(3 * width);
        let w = h / 8;
        Grid {
            cells: vec![0; h * w],
            width: w as i64,
        }
    }

    fn at(&self, index: Point) -> bool {
        // offset by width, width
        let cell_index = (index.x + self.width + 1) * self.width + index.y / 8;
        (self.cells[cell_index as usize] & (1 << index.y % 8)) != 0
    }

    fn set(&mut self, index: Point, val: bool) {
        let cell_index = (index.x + self.width + 1) * self.width + index.y / 8;
        if val {
            self.cells[cell_index as usize] |= 1 << index.y % 8
        } else {
            self.cells[cell_index as usize] &= !(1 << index.y % 8)
        }
    }
}

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
}

fn get_proposals(
    elves: &HashSet<Point>,
    grid: &Grid,
    first_direction: Direction,
) -> HashMap<Point, Option<Point>> {
    let mut proposals = HashMap::new();
    for position in elves.iter() {
        let mut count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if grid.at(Point {
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
        let mut direction = first_direction;
        'direction: for _ in 0..4 {
            for (dx, dy) in direction.checks() {
                if grid.at(Point {
                    x: position.x + dx,
                    y: position.y + dy,
                }) {
                    direction = direction.next();
                    continue 'direction;
                }
            }
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
    elves: &mut HashSet<Point>,
    grid: &mut Grid,
    proposals: HashMap<Point, Option<Point>>,
) -> bool {
    let mut has_moved = false;
    for (target, maybe_from) in proposals {
        if let Some(from) = maybe_from {
            assert_eq!(elves.remove(&from), true);
            grid.set(from, false);
            assert_eq!(elves.insert(target), true);
            grid.set(target, true);
            has_moved = true;
        }
    }
    has_moved
}

fn try_to_move(elves: &mut HashSet<Point>, grid: &mut Grid, first_direction: Direction) -> bool {
    let proposals = get_proposals(elves, grid, first_direction);
    apply_proposals(elves, grid, proposals)
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
                        Some(Point {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    let bounds = |map: &HashSet<Point>| {
        let min_x = map.iter().map(|p| p.x).min().unwrap();
        let min_y = map.iter().map(|p| p.y).min().unwrap();
        let max_x = map.iter().map(|p| p.x).max().unwrap();
        let max_y = map.iter().map(|p| p.y).max().unwrap();
        ((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize)
    };
    let mut grid = {
        let (width, height) = bounds(&elves);
        let mut grid = Grid::new(std::cmp::max(width, height));
        for pos in elves.iter() {
            grid.set(*pos, true);
        }
        grid
    };
    let mut first_direction = Direction::NORTH;
    for _ in 0..10 {
        try_to_move(&mut elves, &mut grid, first_direction);
        first_direction = first_direction.next();
    }
    let (width, height) = bounds(&elves);
    println!("{}", (width * height) - elves.len());
    let mut counter = 11;
    while try_to_move(&mut elves, &mut grid, first_direction) {
        counter += 1;
        first_direction = first_direction.next();
    }
    println!("{}", counter);
}
