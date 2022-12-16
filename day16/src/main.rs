#![feature(entry_insert)]
use std::collections::{BTreeMap, BinaryHeap, HashMap};

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    cells: Vec<T>,
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.cells[self.width * index.0 + index.1]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[self.width * index.0 + index.1]
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct ValveName([u8; 2]);
impl std::fmt::Debug for ValveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Debug)]
struct Valve {
    flow_rate: u8,
    links_to: Vec<ValveName>,
}

fn parse_valve(line: String) -> (ValveName, Valve) {
    let mut iter = line.split(' ');
    let name = ValveName(iter.nth(1).unwrap().as_bytes().try_into().unwrap());
    let flow_word = iter.nth(2).unwrap();
    let flow_rate = flow_word[5..flow_word.len() - 1].parse::<u8>().unwrap();
    let links_to = iter
        .skip(4)
        .map(|w| {
            let wb = w.as_bytes();
            let word = if *wb.last().unwrap() == b',' {
                &wb[0..wb.len() - 1]
            } else {
                wb
            };
            ValveName(word.try_into().unwrap())
        })
        .collect();
    (
        name,
        Valve {
            flow_rate,
            links_to,
        },
    )
}

#[derive(PartialEq, Eq, Debug)]
struct ValveState {
    index: usize,
    step: usize,
    total_pressure_released: usize,
    current_flow_rate: usize,
}

impl PartialOrd for ValveState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValveState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.total_pressure_released / self.step)
            .cmp(&(other.total_pressure_released / other.step))
    }
}

fn run_dijkstra(
    adjacency_matrix: &Grid<usize>,
    interesting_valves: &[(usize, u8)],
    start: usize,
    turns: usize,
) -> usize {
    let mut dijkstra_weight = vec![(0f32, false); adjacency_matrix.width];
    let mut next_cells = BinaryHeap::<ValveState>::new();
    next_cells.push(ValveState {
        index: start,
        step: 0,
        total_pressure_released: 0,
        current_flow_rate: 0,
    });
    let mut max_total_pressure = 0;
    while let Some(c) = next_cells.pop() {
        if c.step > turns {
            return max_total_pressure;
        }
        max_total_pressure = std::cmp::max(
            c.total_pressure_released + (turns - c.step) * c.current_flow_rate,
            max_total_pressure,
        );
        println!("entering: {:?}", &c);
        if dijkstra_weight[c.index].1 {
            if next_cells.is_empty() {
                return max_total_pressure;
            }
            continue;
        }
        dijkstra_weight[c.index].1 = true;
        for neighbor in interesting_valves.iter() {
            if neighbor.0 == c.index {
                continue;
            }
            let move_cost = adjacency_matrix[(c.index, neighbor.0)] + 1;
            let new_total_pressure =
                c.total_pressure_released + c.current_flow_rate * move_cost;
            let new_weight = (new_total_pressure + neighbor.1 as usize) as f32 / (c.step + move_cost + 1) as f32;
            let neighbor_weight = &mut dijkstra_weight[neighbor.0].0;
            if *neighbor_weight < new_weight {
                *neighbor_weight = new_weight;
                next_cells.push(ValveState {
                    index: neighbor.0,
                    step: c.step + move_cost,
                    total_pressure_released: new_total_pressure,
                    current_flow_rate: c.current_flow_rate + neighbor.1 as usize,
                });
            }
        }
        if next_cells.is_empty() {
            return max_total_pressure;
        }
    }
    unreachable!();
}

fn floyd_warshall(grid: &mut Grid<usize>) {
    for i in 0..grid.width {
        grid[(i, i)] = 0;
    }
    for k in 0..grid.width {
        for i in 0..grid.width {
            for j in 0..grid.width {
                if grid[(i, k)] != usize::max_value() && grid[(k, j)] != usize::max_value() {
                    grid[(i, j)] = std::cmp::min(grid[(i, j)], grid[(i, k)] + grid[(k, j)]);
                }
            }
        }
    }
}

fn main() {
    let valves = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_valve)
        .collect::<BTreeMap<_, _>>();
    let num_valves = valves.len();
    let valve_names = valves.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let mut adjacency_matrix = Grid {
        width: num_valves,
        cells: vec![usize::max_value(); num_valves * num_valves],
    };
    let interesting_valves = valves
        .iter()
        .map(|(name, data)| {
            let index = valve_names.binary_search(&name).unwrap();
            for neighbor in data.links_to.iter() {
                let n_index = valve_names.binary_search(&&neighbor).unwrap();
                adjacency_matrix[(index, n_index)] = 1;
            }
            data.flow_rate
        })
        .enumerate()
        .filter(|(_, r)| *r > 0)
        .collect::<Vec<_>>();
    floyd_warshall(&mut adjacency_matrix);

    let start_position = valve_names
        .binary_search(&&ValveName([b'A', b'A']))
        .unwrap();

    println!(
        "{}",
        run_dijkstra(&adjacency_matrix, &interesting_valves, start_position, 30)
    );
}
