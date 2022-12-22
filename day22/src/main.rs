struct Grid {
    cells: Vec<Vec<u8>>,
    min_max_rows: Vec<(usize, usize)>,
    min_max_cols: Vec<(usize, usize)>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            cells: vec![],
            min_max_rows: vec![],
            min_max_cols: vec![],
        }
    }

    fn push(&mut self, new_row: Vec<u8>) {
        let min_row = new_row.iter().position(|c| *c != b' ').unwrap();
        let max_row = new_row.len() - 1;
        self.min_max_rows.push((min_row, max_row));
        if self.min_max_cols.len() < new_row.len() {
            self.min_max_cols
                .resize(new_row.len(), (usize::max_value(), usize::max_value()));
        }
        for (i, c) in new_row.iter().enumerate() {
            if *c != b' ' {
                if self.min_max_cols[i].0 == usize::max_value() {
                    self.min_max_cols[i].0 = self.min_max_rows.len() - 1;
                }
                self.min_max_cols[i].1 = self.min_max_rows.len() - 1;
            }
        }
        self.cells.push(new_row);
    }

    fn next(&self, point: &Point, dir: Direction) -> Point {
        match dir {
            Direction::Right => Point((
                point.0 .0,
                if point.0 .1 == self.min_max_rows[point.0 .0].1 {
                    self.min_max_rows[point.0 .0].0
                } else {
                    point.0 .1 + 1
                },
            )),
            Direction::Down => Point((
                if point.0 .0 == self.min_max_cols[point.0 .1].1 {
                    self.min_max_cols[point.0 .1].0
                } else {
                    point.0 .0 + 1
                },
                point.0 .1,
            )),
            Direction::Left => Point((
                point.0 .0,
                if point.0 .1 == self.min_max_rows[point.0 .0].0 {
                    self.min_max_rows[point.0 .0].1
                } else {
                    point.0 .1 - 1
                },
            )),
            Direction::Up => Point((
                if point.0 .0 == self.min_max_cols[point.0 .1].0 {
                    self.min_max_cols[point.0 .1].1
                } else {
                    point.0 .0 - 1
                },
                point.0 .1,
            )),
        }
    }

    fn get_starting_position(&self) -> Point {
        let mut position = Point((0, self.min_max_rows[0].0));
        while self[&position] == b'#' {
            position = self.next(&position, Direction::Right);
        }
        position
    }
}

impl std::ops::Index<&Point> for Grid {
    type Output = u8;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.cells[index.0 .0][index.0 .1]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl std::ops::Add<i8> for Direction {
    type Output = Self;
    fn add(mut self, clockwise: i8) -> Self::Output {
        self += clockwise;
        self
    }
}

impl std::ops::AddAssign<i8> for Direction {
    fn add_assign(&mut self, clockwise: i8) {
        *self = match (self.to_value() as i8 + clockwise).rem_euclid(4) {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn to_value(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

enum Move {
    Turn(bool),
    Move(usize),
}

#[derive(Clone)]
struct MoveIterator {
    input: String,
    position: usize,
}

impl Iterator for MoveIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.input.as_bytes();
        if self.position == bytes.len() {
            return None;
        }
        if bytes[self.position].is_ascii_digit() {
            let mut num = 0;
            while self.position < bytes.len() && bytes[self.position].is_ascii_digit() {
                num *= 10;
                num += (bytes[self.position] - b'0') as usize;
                self.position += 1;
            }
            Some(Move::Move(num))
        } else {
            let dir = bytes[self.position] == b'R';
            self.position += 1;
            Some(Move::Turn(dir))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point((usize, usize));

impl std::ops::Add<Direction> for Point {
    type Output = Self;

    fn add(mut self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Right => self.0 .1 += 1,
            Direction::Down => self.0 .0 += 1,
            Direction::Left => self.0 .1 -= 1,
            Direction::Up => self.0 .0 -= 1,
        };
        self
    }
}

type TransitionFn = Box<dyn Fn(Point, Direction) -> (Point, Direction)>;

struct Transition {
    // Which face we arrive at.
    target_face: usize,
    // Where, and facing how.
    convert_coords: TransitionFn,
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Face")
            .field("target_face", &self.target_face)
            .finish()
    }
}

#[derive(Debug)]
struct Face {
    offset: (usize, usize),
    // For each direction.
    transitions: [Transition; 4],
}

struct Cube {
    grid: Grid,
    edge_length: usize,
    faces: [Face; 6],
}

fn walk_grid(grid: &Grid, iter: MoveIterator) -> (Point, Direction) {
    let mut position = grid.get_starting_position();
    let mut last_dir = Direction::Right;
    for mov in iter {
        match mov {
            Move::Turn(d) => last_dir += if d { 1 } else { -1 },
            Move::Move(len) => {
                for _ in 0..len {
                    let new_pos = grid.next(&position, last_dir);
                    if grid[&new_pos] == b'#' {
                        break;
                    }
                    position = new_pos;
                }
            }
        }
    }
    (position, last_dir)
}

fn set_transition_functions(
    (from_face, from_dir): (usize, Direction),
    (to_face, to_dir): (usize, Direction),
    edge_length: usize,
    resolved_faces: &[(usize, usize)],
    transitions: &mut [Vec<Option<(usize, TransitionFn)>>],
) {
    let (from_x, from_y) = resolved_faces[from_face];
    let (to_x, to_y) = resolved_faces[to_face];
    let transition_fn = |from_x, from_y, from_dir, to_x, to_y, to_dir, edge_length| {
        Box::new(move |Point((x, y)), dir| {
            assert_eq!(dir, from_dir);
            // Distance between the (rotated) origin corner and the point on the line.
            let diff = match dir {
                Direction::Right => {
                    assert_eq!(y, from_y + edge_length - 1);
                    x - from_x
                }
                Direction::Down => {
                    assert_eq!(x, from_x + edge_length - 1);
                    from_y + edge_length - 1 - y
                }
                Direction::Left => {
                    assert_eq!(y, from_y);
                    from_x + edge_length - 1 - x
                }
                Direction::Up => {
                    assert_eq!(x, from_x);
                    y - from_y
                }
            };
            // Map back to the point on the new edge.
            let new_point = Point(match to_dir {
                Direction::Right => (to_x + diff, to_y),
                Direction::Down => (to_x, to_y + edge_length - 1 - diff),
                Direction::Left => (to_x + edge_length - 1 - diff, to_y + edge_length - 1),
                Direction::Up => (to_x + edge_length - 1, to_y + diff),
            });
            (new_point, to_dir)
        })
    };
    assert!(std::mem::replace(
        &mut transitions[from_face][from_dir.to_value()],
        Some((
            to_face,
            transition_fn(from_x, from_y, from_dir, to_x, to_y, to_dir, edge_length),
        ))
    )
    .is_none());
    assert!(std::mem::replace(
        &mut transitions[to_face][(to_dir + 2).to_value()],
        Some((
            from_face,
            transition_fn(
                to_x,
                to_y,
                to_dir + 2,
                from_x,
                from_y,
                from_dir + 2,
                edge_length,
            ),
        ))
    )
    .is_none());
}

fn build_cube(grid: Grid) -> Cube {
    let edge_length = grid
        .min_max_cols
        .iter()
        .chain(grid.min_max_rows.iter())
        .map(|(a, b)| b + 1 - a)
        .min()
        .unwrap();
    let mut faces = vec![None; 6]; // offsets.
    let mut transitions = Vec::new();
    transitions.resize_with(6, || {
        let mut t = Vec::new();
        t.resize_with(4, || None);
        t
    });
    let identity = || -> TransitionFn { Box::new(|p, d| (p + d, d)) };
    let mut add_identity_transition = |from: usize, to: usize, dir: Direction| {
        transitions[from][dir.to_value()] = Some((to, identity()));
        transitions[to][(dir + 2).to_value()] = Some((from, identity()));
    };
    faces[0] = Some((0, grid.min_max_rows[0].0));
    faces[2] = Some((edge_length, faces[0].unwrap().1));
    add_identity_transition(0, 2, Direction::Down);
    faces[5] = Some((2 * edge_length, faces[0].unwrap().1));
    add_identity_transition(2, 5, Direction::Down);
    if grid.min_max_rows[0].1 > faces[0].unwrap().1 + edge_length {
        faces[1] = Some((0, 2 * edge_length));
        add_identity_transition(0, 1, Direction::Right);
        assert_eq!(grid.min_max_rows[0].0, edge_length);
        faces[4] = Some((2 * edge_length, 0));
        add_identity_transition(4, 5, Direction::Right);
        faces[3] = Some((3 * edge_length, 0));
        add_identity_transition(4, 3, Direction::Down);
    } else {
        // Example cube.
        assert_eq!(grid.min_max_rows[0].0, 2 * edge_length);
        faces[4] = Some((edge_length, edge_length));
        add_identity_transition(4, 2, Direction::Right);
        faces[3] = Some((edge_length, 0));
        add_identity_transition(3, 4, Direction::Right);
        faces[1] = Some((2 * edge_length, 3 * edge_length));
        add_identity_transition(5, 1, Direction::Right);
    }
    let resolved_faces: [_; 6] = faces
        .into_iter()
        .map(Option::unwrap)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut add_transitions = |from, to| {
        set_transition_functions(
            from,
            to,
            edge_length,
            &resolved_faces,
            transitions.as_mut_slice(),
        )
    };
    if grid.min_max_rows[0].1 > resolved_faces[0].1 + edge_length {
        add_transitions((0, Direction::Left), (4, Direction::Right));
        add_transitions((2, Direction::Left), (4, Direction::Down));
        add_transitions((0, Direction::Up), (3, Direction::Right));
        add_transitions((3, Direction::Down), (1, Direction::Down));
        add_transitions((3, Direction::Right), (5, Direction::Up));
        add_transitions((5, Direction::Right), (1, Direction::Left));
        add_transitions((2, Direction::Right), (1, Direction::Up));
    } else {
        // Example cube.
        add_transitions((0, Direction::Left), (4, Direction::Down));
        add_transitions((2, Direction::Right), (1, Direction::Down));
        add_transitions((0, Direction::Up), (3, Direction::Down));
        add_transitions((3, Direction::Down), (5, Direction::Up));
        add_transitions((3, Direction::Left), (1, Direction::Up));
        add_transitions((5, Direction::Left), (4, Direction::Up));
        add_transitions((0, Direction::Right), (1, Direction::Left));
    }

    Cube {
        grid,
        edge_length,
        faces: std::iter::zip(resolved_faces, transitions.into_iter())
            .map(|(offset, transitions)| Face {
                offset,
                transitions: transitions
                    .into_iter()
                    .map(Option::unwrap)
                    .map(|(f, fun)| Transition {
                        target_face: f,
                        convert_coords: fun,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    }
}

fn walk_cube(cube: &Cube, moves: MoveIterator) -> (Point, Direction) {
    let mut position = cube.grid.get_starting_position();
    let mut face = &cube.faces[0];
    let mut dir = Direction::Right;
    let edge_length = cube.edge_length;
    for mov in moves {
        match mov {
            Move::Turn(d) => dir += if d { 1 } else { -1 },
            Move::Move(len) => {
                for _ in 0..len {
                    let is_on_edge = match dir {
                        Direction::Right => position.0 .1 == face.offset.1 + edge_length - 1,
                        Direction::Down => position.0 .0 == face.offset.0 + edge_length - 1,
                        Direction::Left => position.0 .1 == face.offset.1,
                        Direction::Up => position.0 .0 == face.offset.0,
                    };
                    let (new_pos, new_dir, new_face) = if is_on_edge {
                        let transition = &face.transitions[dir.to_value()];
                        let new_face = &cube.faces[transition.target_face];
                        let (new_pos, new_dir) = (*transition.convert_coords)(position, dir);
                        (new_pos, new_dir, new_face)
                    } else {
                        (position + dir, dir, face)
                    };
                    if cube.grid[&new_pos] == b'#' {
                        break;
                    }
                    (position, dir, face) = (new_pos, new_dir, new_face);
                }
            }
        }
    }
    (position, dir)
}

fn main() {
    let mut grid = Grid::new();
    let mut iter = std::io::stdin().lines().map(Result::unwrap);
    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        grid.push(line.into_bytes());
    }
    let moves = MoveIterator {
        input: iter.next().unwrap(),
        position: 0,
    };
    let print_position = |(Point((x, y)), dir): (_, Direction)| {
        println!("{}", 1000 * (x + 1) + 4 * (y + 1) + dir.to_value());
    };
    print_position(walk_grid(&grid, moves.clone()));
    let cube = build_cube(grid);
    print_position(walk_cube(&cube, moves));
}
