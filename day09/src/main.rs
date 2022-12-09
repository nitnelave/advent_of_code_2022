use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Direction {
    x: i32,
    y: i32,
}

impl std::ops::Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

type Rope<const N: usize> = [Point; N];

fn parse_line<S: AsRef<str>>(line: S) -> (Direction, u16) {
    let bytes = line.as_ref().as_bytes();
    let direction = match bytes[0] {
        b'U' => Direction { x: -1, y: 0 },
        b'D' => Direction { x: 1, y: 0 },
        b'L' => Direction { x: 0, y: -1 },
        b'R' => Direction { x: 0, y: 1 },
        _ => panic!(),
    };
    (
        direction,
        std::str::from_utf8(&bytes[2..])
            .unwrap()
            .parse::<u16>()
            .unwrap(),
    )
}

fn pull_rope(head: Point, tail: Point) -> Point {
    if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
        Point {
            x: ((head.x * 2 + tail.x) as f32 / 3.0).round() as i32,
            y: ((head.y * 2 + tail.y) as f32 / 3.0).round() as i32,
        }
    } else {
        tail
    }
}

fn apply_singe_move<const N: usize>(
    mut rope: Rope<N>,
    direction: Direction,
    distance: u16,
    mut positions: HashSet<Point>,
) -> (Rope<N>, HashSet<Point>) {
    for _ in 0..distance {
        rope[0] = rope[0] + direction;
        for i in 0..(N - 1) {
            rope[i + 1] = pull_rope(rope[i], rope[i + 1]);
        }
        positions.insert(rope[N - 1]);
    }
    (rope, positions)
}

fn apply_all_moves<const N: usize>(moves: &[(Direction, u16)]) -> usize {
    let start = Point::default();
    let (_, mut positions) = moves.iter().fold(
        ([start; N], HashSet::<Point>::new()),
        |(rope, positions), (direction, distance)| {
            apply_singe_move(rope, *direction, *distance, positions)
        },
    );
    positions.insert(start);
    positions.len()
}

fn main() {
    let moves = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .collect::<Vec<_>>();
    println!("{}", apply_all_moves::<2>(&moves));
    println!("{}", apply_all_moves::<10>(&moves));
}
