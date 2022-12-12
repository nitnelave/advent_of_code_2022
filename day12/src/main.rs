type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;
struct Grid<T> {
    height: i64,
    width: i64,
    cells: Vec<T>,
}

impl<T> std::ops::Index<(i64, i64)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        &self.cells[(self.width * index.0 + index.1) as usize]
    }
}

impl<T> std::ops::IndexMut<(i64, i64)> for Grid<T> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        &mut self.cells[(self.width * index.0 + index.1) as usize]
    }
}

impl<T: Default> Grid<T> {
    fn new(height: i64, width: i64) -> Self {
        Self {
            height,
            width,
            cells: Vec::from_iter(
                std::iter::repeat_with(|| T::default()).take((height * width) as usize),
            ),
        }
    }
}
impl Grid<u8> {
    fn from_iterator<S: AsRef<str>, I: Iterator<Item = S>>(mut iter: I) -> Result<Self> {
        let first_line = iter.next().ok_or("empty")?;
        let mut grid = Self {
            height: 1,
            width: first_line.as_ref().bytes().len() as i64,
            cells: first_line.as_ref().bytes().collect(),
        };

        for line in iter {
            grid.height += 1;
            if line.as_ref().bytes().len() as i64 != grid.width {
                return Err("Inconsistent line lengths");
            }
            grid.cells.extend(line.as_ref().bytes());
        }

        Ok(grid)
    }
}

struct CellValue {
    distance: usize,
    visited: bool,
}

impl Default for CellValue {
    fn default() -> Self {
        Self {
            distance: usize::max_value(),
            visited: false,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    distance: usize,
    coords: (i64, i64),
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            coords: (i64::max_value(), i64::max_value()),
            distance: usize::max_value(),
        }
    }
}

fn can_step(from: u8, to: u8) -> bool {
    if from == b'S' {
        to == b'a' || to == b'b'
    } else if to == b'E' {
        from == b'y' || from == b'z'
    } else {
        to as i16 - from as i16 <= 1
    }
}

fn run_dijkstra(input_grid: &Grid<u8>, start: (i64, i64), reset_at_a: bool) -> Option<usize> {
    let mut dijkstra_grid = Grid::<CellValue>::new(input_grid.height, input_grid.width);
    dijkstra_grid[start].distance = 0;
    let mut next_cells = std::collections::BTreeSet::<Cell>::new();
    next_cells.insert(Cell {
        coords: start,
        distance: 0,
    });
    const CARDINALS: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(c) = next_cells.pop_first() {
        if input_grid[c.coords] == b'E' {
            return Some(c.distance);
        }
        if dijkstra_grid[c.coords].visited {
            continue;
        }
        assert_eq!(dijkstra_grid[c.coords].distance, c.distance);
        dijkstra_grid[c.coords].visited = true;
        for (dx, dy) in CARDINALS {
            let new_coords = (c.coords.0 + dx, c.coords.1 + dy);
            if new_coords.0 < 0
                || new_coords.1 < 0
                || new_coords.0 >= input_grid.height
                || new_coords.1 >= input_grid.width
            {
                continue;
            }
            let new_distance = if reset_at_a && input_grid[new_coords] == b'a' {
                0
            } else {
                c.distance + 1
            };
            if !can_step(input_grid[c.coords], input_grid[new_coords])
                || dijkstra_grid[new_coords].distance <= new_distance
            {
                continue;
            }
            dijkstra_grid[new_coords].distance = new_distance;
            next_cells.insert(Cell {
                coords: new_coords,
                distance: new_distance,
            });
        }
    }
    None
}

fn main() {
    let input_grid =
        Grid::<u8>::from_iterator(std::io::stdin().lines().map(std::result::Result::unwrap))
            .unwrap();
    let start = (|| {
        for i in 0..input_grid.height {
            for j in 0..input_grid.width {
                if input_grid[(i, j)] == b'S' {
                    return (i, j);
                }
            }
        }
        unreachable!();
    })();
    println!("{}", run_dijkstra(&input_grid, start, false).unwrap());
    println!("{}", run_dijkstra(&input_grid, start, true).unwrap());
}
