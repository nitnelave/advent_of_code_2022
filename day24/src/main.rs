use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
        match dir {
            Direction::NORTH => Point {
                x: self.x - 1,
                ..self
            },
            Direction::EAST => Point {
                y: self.y + 1,
                ..self
            },
            Direction::SOUTH => Point {
                x: self.x + 1,
                ..self
            },
            Direction::WEST => Point {
                y: self.y - 1,
                ..self
            },
        }
    }
}

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
        if p.x == self.height + 1 || p.x == 0 {
            return false;
        }
        let shifted_point = Point {
            x: (p.x - 1 - turn as i64 * self.direction.0).rem_euclid(self.height) + 1,
            y: (p.y - 1 - turn as i64 * self.direction.1).rem_euclid(self.width) + 1,
        };
        self.grid.at(shifted_point)
    }
}

fn walk_one_step(
    from: Point,
    turn: usize,
    exit: Point,
    walls: &Grid,
    blizzards: &[BlizzardGrid; 4],
    reachable_squares: &mut HashSet<Point>,
) -> bool {
    let check_blizzards = |p| blizzards.iter().any(|b| b.at(p, turn));

    if !check_blizzards(from) {
        reachable_squares.insert(from);
    }
    for d in CARDINALS {
        let p = from + d;
        if !walls.at(p) && !check_blizzards(p) {
            if p == exit {
                return true;
            }
            reachable_squares.insert(p);
        }
    }
    false
}

fn walk_one_turn(
    turn: usize,
    exit: Point,
    walls: &Grid,
    blizzards: &[BlizzardGrid; 4],
    previously_reachable_squares: &HashSet<Point>,
) -> (HashSet<Point>, bool) {
    let mut reachable_squares = HashSet::new();
    let exited = previously_reachable_squares
        .iter()
        .any(|p| walk_one_step(*p, turn, exit, walls, blizzards, &mut reachable_squares));
    (reachable_squares, exited)
}

fn walk_to_exit(
    from: Point,
    exit: Point,
    starting_turn: usize,
    walls: &Grid,
    blizzards: &[BlizzardGrid; 4],
) -> usize {
    let mut reachable_squares = HashSet::new();
    assert!(!walls.at(from));
    assert!(!walls.at(exit));
    for turn in starting_turn + 1.. {
        if !blizzards.iter().any(|b| b.at(from, turn)) {
            reachable_squares.insert(from);
        }
        let (new_squares, exit) = walk_one_turn(turn, exit, walls, blizzards, &reachable_squares);
        if exit {
            return turn;
        }
        reachable_squares = new_squares;
    }
    unreachable!()
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
    let blizzards = [
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::NORTH),
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::EAST),
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::SOUTH),
        BlizzardGrid::new(blizzards_grid.pop().unwrap(), width, Direction::WEST),
    ];
    let last_turn = walk_to_exit(entrance + Direction::SOUTH, exit, 0, &walls, &blizzards);
    println!("{last_turn}");
    walls.set(exit, true);
    walls.set(entrance, false);
    let last_turn = walk_to_exit(
        exit + Direction::NORTH,
        entrance,
        last_turn,
        &walls,
        &blizzards,
    );
    walls.set(entrance, true);
    walls.set(exit, false);
    let last_turn = walk_to_exit(
        entrance + Direction::SOUTH,
        exit,
        last_turn,
        &walls,
        &blizzards,
    );
    println!("{last_turn}");
}
