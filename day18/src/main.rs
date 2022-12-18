#![feature(iter_intersperse)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct Grid<T> {
    height: usize, // x
    width: usize,  // y
    depth: usize,  // z
    cells: Vec<T>,
}

impl<T: Copy> Grid<T> {
    fn new(height: usize, width: usize, depth: usize, val: T) -> Self {
        Self {
            height,
            width,
            depth,
            cells: vec![val; height * width * depth],
        }
    }
}

impl<T> std::ops::Index<(usize, usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        &self.cells[self.depth * (self.width * index.0 + index.1) + index.2]
    }
}

impl<T> std::ops::IndexMut<(usize, usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        &mut self.cells[self.depth * (self.width * index.0 + index.1) + index.2]
    }
}

fn parse_droplet(line: String) -> Point {
    let mut iter = line.split(',').map(str::parse::<usize>).map(Result::unwrap);
    Point {
        x: iter.next().unwrap(),
        y: iter.next().unwrap(),
        z: iter.next().unwrap(),
    }
}

fn count_inner_faces(points: &mut [Point]) -> (usize, Point) {
    points.sort();
    let num_touching_z = points
        .windows(2)
        .filter(|d| d[0].x == d[1].x && d[0].y == d[1].y && d[0].z + 1 == d[1].z)
        .count();
    let max_x = points[points.len() - 1].x;
    points.sort_by_key(|d| (d.z, d.x, d.y));
    let num_touching_y = points
        .windows(2)
        .filter(|d| d[0].x == d[1].x && d[0].y + 1 == d[1].y && d[0].z == d[1].z)
        .count();
    let max_z = points[points.len() - 1].z;
    points.sort_by_key(|d| (d.y, d.z, d.x));
    let num_touching_x = points
        .windows(2)
        .filter(|d| d[0].x + 1 == d[1].x && d[0].y == d[1].y && d[0].z == d[1].z)
        .count();
    let max_y = points[points.len() - 1].y;
    (
        num_touching_x + num_touching_y + num_touching_z,
        Point {
            x: max_x,
            y: max_y,
            z: max_z,
        },
    )
}

fn count_connected_components_outer_faces(
    is_lava: &Grid<bool>,
    max_num_components: usize,
) -> usize {
    let mut union_find = petgraph::unionfind::UnionFind::<u16>::new(max_num_components);
    let mut labels = Grid::new(
        is_lava.height,
        is_lava.width,
        is_lava.depth,
        u16::max_value(),
    );
    let mut num_classes = 0;
    for x in 0..is_lava.height {
        for y in 0..is_lava.width {
            for z in 0..is_lava.depth {
                if !is_lava[(x, y, z)] {
                    let mut neighbor_labels = vec![];
                    if x > 0 && !is_lava[(x - 1, y, z)] {
                        neighbor_labels.push(labels[(x - 1, y, z)]);
                    }
                    if y > 0 && !is_lava[(x, y - 1, z)] {
                        neighbor_labels.push(labels[(x, y - 1, z)]);
                    }
                    if z > 0 && !is_lava[(x, y, z - 1)] {
                        neighbor_labels.push(labels[(x, y, z - 1)]);
                    }
                    match neighbor_labels.iter().min() {
                        None => {
                            labels[(x, y, z)] = num_classes;
                            num_classes += 1;
                        }
                        Some(min) => {
                            labels[(x, y, z)] = *min;
                            for label in neighbor_labels.iter() {
                                union_find.union(*label, *min);
                            }
                        }
                    };
                }
            }
        }
    }
    let mut equivalence_classes = std::collections::HashMap::<u16, Vec<Point>>::new();
    let background_class = union_find.find_mut(0);
    // The background class has label 0.
    for x in 0..is_lava.height {
        for y in 0..is_lava.width {
            for z in 0..is_lava.depth {
                if !is_lava[(x, y, z)] {
                    let label = union_find.find_mut(labels[(x, y, z)]);
                    if label != background_class {
                        equivalence_classes
                            .entry(label)
                            .or_default()
                            .push(Point { x, y, z });
                    }
                }
            }
        }
    }
    equivalence_classes
        .values_mut()
        .map(Vec::as_mut_slice)
        .map(|points| 6 * points.len() - 2 * count_inner_faces(points).0)
        .sum()
}

fn main() {
    let mut droplets = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_droplet)
        .collect::<Vec<_>>();
    let (inner_faces, max_coords) = count_inner_faces(&mut droplets);
    let total_faces = 6 * droplets.len() - 2 * inner_faces;
    println!("{}", total_faces);
    let grid = {
        let mut grid = Grid::new(max_coords.x + 1, max_coords.y + 1, max_coords.z + 1, false);
        for d in droplets.iter() {
            grid[(d.x, d.y, d.z)] = true;
        }
        grid
    };
    let inner_faces = count_connected_components_outer_faces(&grid, droplets.len() / 4);
    println!("{}", total_faces - inner_faces);
}
