use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

const CARDINALS: [Direction; 4] = [
    Direction::NORTH,
    Direction::EAST,
    Direction::SOUTH,
    Direction::WEST,
];

impl Direction {
    fn to_coords(self) -> (i64, i64) {
        match self {
            Direction::NORTH => (-1, 0),
            Direction::EAST => (0, 1),
            Direction::SOUTH => (1, 0),
            Direction::WEST => (0, -1),
        }
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, dir: Direction) -> Self::Output {
        Point {
            x: self.x + dir.to_coords().0,
            y: self.y + dir.to_coords().1,
        }
    }
}

#[derive(Clone)]
struct Grid {
    cells: Vec<u8>,
    width: usize,
}

impl Grid {
    fn new(width: usize) -> Self {
        let w = (width + 7) / 8;
        Grid {
            cells: vec![],
            width: w,
        }
    }

    fn push_new_line(&mut self) {
        self.cells.resize(self.cells.len() + self.width, 0);
    }

    fn cell_index(&self, index: Point) -> usize {
        (index.x as usize * self.width + index.y as usize / 8) as usize
    }

    fn at(&self, index: Point) -> bool {
        (self.cells[self.cell_index(index)] & (1 << index.y % 8)) != 0
    }

    fn set(&mut self, index: Point, val: bool) {
        let cell_index = self.cell_index(index);
        if val {
            self.cells[cell_index] |= 1 << index.y % 8
        } else {
            self.cells[cell_index] &= !(1 << index.y % 8)
        }
    }
}

struct BlizzardGrid {
    grid: Grid,
    height: i64,
    width: i64,
    direction: (i64, i64),
}

impl BlizzardGrid {
    fn new(grid: Grid, width: usize, direction: Direction) -> Self {
        let height = (grid.cells.len() / grid.width - 2) as i64;
        Self {
            grid,
            height,
            width: (width - 2) as i64,
            direction: direction.to_coords(),
        }
    }

    fn at(&self, p: Point, turn: usize) -> bool {
        let shifted_point = Point {
            x: (p.x - 1 - turn as i64 * self.direction.0).rem_euclid(self.height) + 1,
            y: (p.y - 1 - turn as i64 * self.direction.1).rem_euclid(self.width) + 1,
        };
        self.grid.at(shifted_point)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    weight: usize,
    turn: usize,
    point: Point,
}

fn dijkstra_to_exit(
    from: Point,
    exit: Point,
    starting_turn: usize,
    walls: &Grid,
    blizzards: &[BlizzardGrid; 4],
) -> usize {
    let mut visited = HashSet::new();
    let mut next_cells = std::collections::BTreeSet::<Cell>::new();
    let distance_to_exit = |p: Point| (exit.x.abs_diff(p.x) + exit.y.abs_diff(p.y)) as usize;
    let blocked_by_blizzards = |p, turn| blizzards.iter().any(|b| b.at(p, turn));
    let mut last_weight;
    let mut last_start_turn = starting_turn;
    let distance_from_start_to_exit = distance_to_exit(from);
    loop {
        last_start_turn += 1;
        while blocked_by_blizzards(from, last_start_turn) || visited.contains(&(from, last_start_turn)) {
            last_start_turn += 1;
        }
        last_weight = last_start_turn + distance_from_start_to_exit;
        next_cells.insert(Cell { weight: last_weight, turn: last_start_turn, point: from });
        while let Some(c) = next_cells.pop_first() {
            if c.point == exit {
                return c.turn + 1;
            }
            if visited.contains(&(c.point, c.turn)) {
                continue;
            }
            visited.insert((c.point, c.turn));
            while c.weight > last_weight {
                last_weight += 1;
                last_start_turn += 1;
                if !blocked_by_blizzards(from, last_start_turn) {
                    next_cells.insert(Cell { weight: last_weight, turn: last_start_turn, point: from });
                }
            }
            if !blocked_by_blizzards(c.point, c.turn + 1) {
                next_cells.insert(Cell {
                    weight: c.weight + 1,
                    turn: c.turn + 1,
                    point: c.point,
                });
            }
            for dir in CARDINALS {
                let p = c.point + dir;
                if !walls.at(p) && !blocked_by_blizzards(p, c.turn + 1) {
                    next_cells.insert(Cell {
                        weight: c.turn + 1 + distance_to_exit(p),
                        turn: c.turn + 1,
                        point: p,
                    });
                }
            }
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap).peekable();
    let width = lines.peek().unwrap().as_bytes().len();
    let mut walls = Grid::new(width);
    let mut blizzards_grid = vec![
        Grid::new(width),
        Grid::new(width),
        Grid::new(width),
        Grid::new(width),
    ];
    let mut entrance = None;
    let mut exit = Point { x: 0, y: 0 };
    lines.enumerate().for_each(|(x, l)| {
        walls.push_new_line();
        for b in blizzards_grid.iter_mut() {
            b.push_new_line();
        }
        l.as_bytes().iter().copied().enumerate().for_each(|(y, b)| {
            let point = Point {
                x: x as i64,
                y: y as i64,
            };
            match b {
                b'#' => walls.set(point, true),
                // Opposite order since we'll pop.
                b'^' => blizzards_grid[3].set(point, true),
                b'>' => blizzards_grid[2].set(point, true),
                b'v' => blizzards_grid[1].set(point, true),
                b'<' => blizzards_grid[0].set(point, true),
                b'.' => {
                    if entrance.is_none() {
                        entrance = Some(point);
                    }
                    exit = point;
                }
                _ => unreachable!(),
            }
        });
    });
    let entrance = entrance.unwrap();
    walls.set(entrance, true);
    walls.set(exit, true);
    let blizzards = [
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::NORTH),
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::EAST),
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::SOUTH),
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::WEST),
    ];
    let last_turn = dijkstra_to_exit(
        entrance + Direction::SOUTH,
        exit + Direction::NORTH,
        0,
        &walls,
        &blizzards,
    );
    println!("{last_turn}");
    let last_turn = dijkstra_to_exit(
        exit + Direction::NORTH,
        entrance + Direction::SOUTH,
        last_turn,
        &walls,
        &blizzards,
    );
    let last_turn = dijkstra_to_exit(
        entrance + Direction::SOUTH,
        exit + Direction::NORTH,
        last_turn,
        &walls,
        &blizzards,
    );
    println!("{last_turn}");
}
