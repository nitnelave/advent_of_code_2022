use std::cmp::{max, min};

struct Grid {
    height: u8,
    width: u16,
    cells: Vec<u8>,
}

impl Grid {
    fn new(height: u8, width: u16) -> Self {
        Self {
            height,
            width,
            cells: vec![0; height as usize * width as usize / 8 + 1],
        }
    }

    fn is_full(&self, x: u16, y: u8) -> bool {
        let linear_index = y as usize * self.width as usize + x as usize;
        self.cells[linear_index / 8] & (1 << (linear_index % 8)) != 0
    }

    fn set_full(&mut self, x: u16, y: u8) {
        let linear_index = y as usize * self.width as usize + x as usize;
        self.cells[linear_index / 8] |= 1 << (linear_index % 8)
    }
}

fn parse_line(line: String) -> Vec<(u16, u8)> {
    line.split(" -> ")
        .map(|c| {
            let coords = c.split_once(',').unwrap();
            (
                coords.0.parse::<u16>().unwrap(),
                coords.1.parse::<u8>().unwrap(),
            )
        })
        .collect()
}

fn populate_grid(grid: &mut Grid, rocks: &[Vec<(u16, u8)>], min_x: u16) {
    for rock in rocks.iter() {
        for line in rock.windows(2) {
            let from = line.first().unwrap();
            let to = line.last().unwrap();
            if from.0 == to.0 {
                for y in min(from.1, to.1)..=max(from.1, to.1) {
                    grid.set_full(from.0 - min_x + 1, y);
                }
            } else {
                for x in min(from.0, to.0)..=max(from.0, to.0) {
                    grid.set_full(x - min_x + 1, from.1);
                }
            }
        }
    }
}

fn fill_sand(grid: &mut Grid, start_x: u16, max_y: u8) -> (usize, usize) {
    let mut stack = vec![start_x];
    // The sand moves diagonaly, so we'll never get beyond one grain per line.
    stack.reserve(grid.height.into());
    let mut sand_count = 0;
    let mut first_sand_count = 0;
    loop {
        let x = *stack.last().unwrap();
        let y = (stack.len() - 1) as u8;
        if first_sand_count == 0 && y == max_y {
            first_sand_count = sand_count;
        }
        // Floor at max_y == 2.
        if y == max_y + 1 {
            grid.set_full(x, y);
            stack.truncate(stack.len() - 1);
            sand_count += 1;
        } else if !grid.is_full(x, y + 1) {
            stack.push(x);
        } else if !grid.is_full(x - 1, y + 1) {
            stack.push(x - 1);
        } else if !grid.is_full(x + 1, y + 1) {
            stack.push(x + 1);
        // Sand coming to rest at (500, 0).
        } else if stack.len() == 1 {
            return (first_sand_count, sand_count + 1);
        } else {
            grid.set_full(x, y);
            stack.truncate(stack.len() - 1);
            sand_count += 1;
        }
    }
}

fn main() {
    let rocks = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .collect::<Vec<_>>();
    let max_y = *rocks.iter().flatten().map(|(_, y)| y).max().unwrap();
    let min_x = *rocks.iter().flatten().map(|(x, _)| x).min().unwrap();
    let max_x = *rocks.iter().flatten().map(|(x, _)| x).max().unwrap();
    let min_x_bound = min(min_x, 500 - max_y as u16);
    let max_x_bound = max(max_x, 500 + max_y as u16);
    let mut grid = Grid::new(max_y + 2, max_x_bound - min_x_bound + 1);
    populate_grid(&mut grid, &rocks, min_x_bound);
    let (part_1, part_2) = fill_sand(&mut grid, 500 - min_x_bound + 1, max_y);
    println!("{}", part_1);
    println!("{}", part_2);
}
