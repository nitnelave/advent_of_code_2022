struct Tree(u8);

const HEIGHT_MASK: u8 = 0xF;
const VISIBLE_BIT: u8 = 1 << 4;

impl Tree {
    fn height(&self) -> u8 {
        self.0 & HEIGHT_MASK
    }

    fn is_visible(&self) -> bool {
        (self.0 & VISIBLE_BIT) != 0
    }

    fn set_visible(&mut self) {
        self.0 |= VISIBLE_BIT
    }
}

impl From<u8> for Tree {
    fn from(c: u8) -> Self {
        Tree(c - b'0' + 1)
    }
}

struct Grid<T> {
    height: usize,
    width: usize,
    cells: Vec<T>,
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[index.0 * self.width + index.1]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[index.0 * self.width + index.1]
    }
}

fn read_trees(input: &str) -> Grid<Tree> {
    let mut grid = Grid {
        height: 0,
        width: 0,
        cells: vec![],
    };
    for (i, c) in input.bytes().enumerate() {
        if c == b'\n' {
            if grid.width == 0 {
                grid.width = i;
            } else {
                assert_eq!((i + 1) % (grid.width + 1), 0);
            }
        } else {
            grid.cells.push(c.into());
        }
    }
    grid.height = grid.cells.len() / grid.width;
    grid
}

fn check_line<
    I: Iterator<Item = usize> + Clone,
    I2: Iterator<Item = usize> + Clone,
    F: Fn(usize, usize) -> (usize, usize),
>(
    grid: &mut Grid<Tree>,
    row_range: I2,
    col_range: I,
    indexer: F,
) {
    for i in row_range {
        let mut max = 0;
        for j in col_range.clone() {
            let tree = &mut grid[indexer(i, j)];
            if tree.height() > max {
                max = tree.height();
                tree.set_visible();
            }
        }
    }
}

fn check_grid(grid: &mut Grid<Tree>) {
    check_line(grid, 0..grid.width, 0..grid.height, |i, j| (i, j));
    check_line(grid, 0..grid.width, (0..grid.height).rev(), |i, j| (i, j));
    check_line(grid, 0..grid.height, 0..grid.width, |i, j| (j, i));
    check_line(grid, 0..grid.height, (0..grid.width).rev(), |i, j| (j, i));
}

fn check_treehouse_view_line<
    I: Iterator<Item = usize> + Clone + ExactSizeIterator,
    F: Fn(usize) -> (usize, usize),
>(
    grid: &Grid<Tree>,
    range: I,
    indexer: F,
    tree_height: u8,
) -> usize {
    let range_size = range.len();
    for (i, k) in range.enumerate() {
        if grid[indexer(k)].height() >= tree_height {
            return i + 1;
        }
    }
    range_size
}

fn check_treehouse(grid: &Grid<Tree>, i: usize, j: usize) -> usize {
    let tree_height = grid[(i, j)].height();
    check_treehouse_view_line(&grid, (0..i).rev(), |k| (k, j), tree_height)
        * check_treehouse_view_line(&grid, (0..j).rev(), |k| (i, k), tree_height)
        * check_treehouse_view_line(&grid, (i + 1)..grid.height, |k| (k, j), tree_height)
        * check_treehouse_view_line(&grid, (j + 1)..grid.width, |k| (i, k), tree_height)
}

fn main() {
    let contents = {
        let mut contents = String::new();
        use std::io::Read;
        std::io::stdin().read_to_string(&mut contents).unwrap();
        contents
    };
    let grid = {
        let mut grid = read_trees(&contents);
        check_grid(&mut grid);
        grid
    };
    println!("{}", grid.cells.iter().filter(|t| t.is_visible()).count());
    let for_each_inner_cell = (1..(grid.height - 1))
        .map(|w| (1..(grid.width - 1)).map(move |h| (w, h)))
        .flatten();
    let max_score = for_each_inner_cell
        .map(|(i, j)| check_treehouse(&grid, i, j))
        .max()
        .unwrap();
    println!("{}", max_score);
}
