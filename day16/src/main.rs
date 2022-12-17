#![feature(entry_insert)]
use std::collections::{BTreeMap, HashMap};

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

#[derive(Eq, Hash, PartialEq, Clone)]
struct State {
    visited_states: u64,
    time_reached: u8,
    elephant_time_reached: u8,
    current_state: usize,
    elephant_current_state: usize,
}

impl State {
    fn current_state(&self, is_elephant: bool) -> usize {
        if is_elephant {
            self.elephant_current_state
        } else {
            self.current_state
        }
    }
    fn time_reached(&self, is_elephant: bool) -> u8 {
        if is_elephant {
            self.elephant_time_reached
        } else {
            self.time_reached
        }
    }

    fn update_with(
        &self,
        visited_states: u64,
        new_state: usize,
        time_reached: u8,
        is_elephant: bool,
    ) -> Self {
        let mut ret = Self {
            visited_states,
            time_reached: if is_elephant {
                self.time_reached
            } else {
                time_reached
            },
            current_state: if is_elephant {
                self.current_state
            } else {
                new_state
            },
            elephant_time_reached: if is_elephant {
                time_reached
            } else {
                self.elephant_time_reached
            },
            elephant_current_state: if is_elephant {
                new_state
            } else {
                self.elephant_current_state
            },
        };
        let swap = if is_elephant {
            self.current_state < new_state
        } else {
            new_state < self.elephant_current_state
        };
        if swap {
            std::mem::swap(&mut ret.current_state, &mut ret.elephant_current_state);
            std::mem::swap(&mut ret.time_reached, &mut ret.elephant_time_reached);
        }
        ret
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("visited_states", &format!("{:b}", &self.visited_states))
            .field("time_reached", &self.time_reached)
            .field("current_state", &self.current_state)
            .finish()
    }
}

fn floyd_warshall(grid: &mut Grid<u8>) {
    for i in 0..grid.width {
        grid[(i, i)] = 0;
    }
    for k in 0..grid.width {
        for i in 0..grid.width {
            for j in 0..grid.width {
                if grid[(i, k)] != u8::max_value() && grid[(k, j)] != u8::max_value() {
                    grid[(i, j)] = std::cmp::min(grid[(i, j)], grid[(i, k)] + grid[(k, j)]);
                }
            }
        }
    }
}

fn take_one_step(
    adjacency_matrix: &Grid<u8>,
    interesting_valves: &[(usize, u8)],
    current_states: &mut HashMap<State, usize>,
    time: u8,
    max_time: u8,
    max_distances: &[u8],
    is_elephant: bool,
) {
    let mut new_states = vec![];
    let mut to_remove = vec![];
    for (state, flow) in current_states.iter() {
        if state.time_reached(is_elephant) + max_distances[state.current_state(is_elephant)] + 1
            < time
        {
            to_remove.push(state.clone());
            continue;
        }
        for (target, target_flow) in interesting_valves.iter() {
            if state.visited_states & (1 << target) != 0 {
                continue;
            }
            if state.time_reached(is_elephant)
                + adjacency_matrix[(state.current_state(is_elephant), *target)]
                + 1
                == time
            {
                let new_state = state.update_with(
                    state.visited_states | (1 << target),
                    *target,
                    time,
                    is_elephant,
                );
                let new_flow = flow + *target_flow as usize * (max_time - time) as usize;
                if current_states.get(&new_state).copied().unwrap_or_default() < new_flow {
                    new_states.push((new_state, new_flow));
                }
            }
        }
    }
    for state in to_remove.into_iter() {
        current_states.remove(&state);
    }
    for (state, flow) in new_states.into_iter() {
        let entry = current_states.entry(state).or_default();
        if *entry < flow {
            *entry = flow;
        }
    }
}

fn step_through_time(
    adjacency_matrix: &Grid<u8>,
    interesting_valves: &[(usize, u8)],
    start_index: usize,
    max_time: u8,
    max_distances: &[u8],
    with_elephant: bool,
) -> usize {
    let mut current_states = HashMap::<State, usize>::new();
    current_states.insert(
        State {
            visited_states: 1,
            time_reached: 0,
            current_state: start_index,
            elephant_time_reached: 0,
            elephant_current_state: start_index,
        },
        0,
    );
    for i in 1..=max_time {
        take_one_step(
            adjacency_matrix,
            interesting_valves,
            &mut current_states,
            i,
            max_time,
            max_distances,
            false,
        );
        if with_elephant {
            take_one_step(
                adjacency_matrix,
                interesting_valves,
                &mut current_states,
                i,
                max_time,
                max_distances,
                true,
            );
        }
    }
    *current_states.values().max().unwrap()
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
        cells: vec![u8::max_value(); num_valves * num_valves],
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
    let max_distances = (0..adjacency_matrix.width)
        .map(|i| {
            (0..adjacency_matrix.width)
                .map(|j| adjacency_matrix[(i, j)])
                .max()
                .unwrap()
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        step_through_time(
            &adjacency_matrix,
            &interesting_valves,
            0,
            30,
            &max_distances,
            false
        )
    );
    println!(
        "{}",
        step_through_time(
            &adjacency_matrix,
            &interesting_valves,
            0,
            26,
            &max_distances,
            true
        )
    );
}
