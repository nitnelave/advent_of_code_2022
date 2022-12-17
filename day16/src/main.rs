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

#[derive(Eq, Hash, PartialEq)]
struct State {
    visited_states: u64,
    time_reached: usize,
    current_state: usize,
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

fn step_through_time(
    adjacency_matrix: &Grid<usize>,
    interesting_valves: &[(usize, u8)],
    start_index: usize,
    max_time: usize,
    max_distance: usize,
) -> HashMap<State, usize> {
    let mut current_states = HashMap::<State, usize>::new();
    current_states.insert(
        State {
            visited_states: 0,
            time_reached: 0,
            current_state: start_index,
        },
        0,
    );
    for i in 1..=max_time {
        let mut new_states = vec![];
        for (state, flow) in current_states.iter() {
            if state.time_reached + max_distance + 1 < i {
                continue;
            }
            for (target, target_flow) in interesting_valves.iter() {
                if state.visited_states & (1 << target) != 0 {
                    continue;
                }
                if state.time_reached + adjacency_matrix[(state.current_state, *target)] + 1 == i {
                    let new_state = State {
                        visited_states: state.visited_states | (1 << target),
                        time_reached: i,
                        current_state: *target,
                    };
                    let new_flow = flow + *target_flow as usize * (max_time - i);
                    if current_states.get(&new_state).copied().unwrap_or_default() < new_flow {
                        new_states.push((new_state, new_flow));
                    }
                }
            }
        }
        for (state, flow) in new_states.into_iter() {
            let entry = current_states.entry(state).or_default();
            if *entry < flow {
                *entry = flow;
            }
        }
    }
    current_states
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
    let max_distance = interesting_valves
        .iter()
        .map(|(i, _)| {
            interesting_valves
                .iter()
                .map(|(j, _)| adjacency_matrix[(*i, *j)])
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    {
        let all_reachable_states =
            step_through_time(&adjacency_matrix, &interesting_valves, 0, 30, max_distance);
        let max_flow = *all_reachable_states.values().max().unwrap();
        println!("{}", max_flow);
    }

    {
        let all_reachable_states =
            step_through_time(&adjacency_matrix, &interesting_valves, 0, 26, max_distance);
        let mut max_flow = *all_reachable_states.values().max().unwrap();
        let mut states_for_flow = all_reachable_states
            .iter()
            .map(|(state, flow)| (flow, state))
            .collect::<Vec<_>>();
        states_for_flow.sort_by_key(|(flow, _)| usize::max_value() - *flow);

        for i in 0..states_for_flow.len() - 1 {
            let s1 = states_for_flow[i];
            if s1.0 + states_for_flow[i + 1].0 < max_flow {
                break;
            }
            for j in (i + 1)..states_for_flow.len() {
                let s2 = states_for_flow[j];
                let new_flow = s1.0 + s2.0;
                if new_flow < max_flow {
                    break;
                }
                if s1.1.visited_states & s2.1.visited_states == 0 {
                    max_flow = std::cmp::max(max_flow, new_flow);
                }
            }
        }
        println!("{}", max_flow);
    }
}
