type OreAmount = u16;

type Recipe = [OreAmount; 4];

#[derive(Debug)]
struct Blueprint {
    recipes: [Recipe; 4],
}

fn parse_blueprint(line: String) -> Blueprint {
    let mut iter = line.split_ascii_whitespace();
    let ore_recipe = [iter.nth(6).unwrap().parse::<OreAmount>().unwrap(), 0, 0, 0];
    let clay_recipe = [iter.nth(5).unwrap().parse::<OreAmount>().unwrap(), 0, 0, 0];
    let obsidian_recipe = [
        iter.nth(5).unwrap().parse::<OreAmount>().unwrap(),
        iter.nth(2).unwrap().parse::<OreAmount>().unwrap(),
        0,
        0,
    ];
    let geode_recipe = [
        iter.nth(5).unwrap().parse::<OreAmount>().unwrap(),
        0,
        iter.nth(2).unwrap().parse::<OreAmount>().unwrap(),
        0,
    ];
    Blueprint {
        recipes: [ore_recipe, clay_recipe, obsidian_recipe, geode_recipe],
    }
}

#[derive(Debug)]
struct State {
    ores: [OreAmount; 4],
    robots: [u16; 4],
    time: usize,
}

fn recurse_simulation(
    blueprint: &Blueprint,
    state: State,
    max_time: usize,
    max_robots: &[u16; 4],
) -> OreAmount {
    let mut max_geodes = 0;
    for i in 0..blueprint.recipes.len() {
        if state.robots[i] == max_robots[i] {
            continue;
        }
        if i < 2 && state.robots[i + 2] > 2 {
            continue;
        }
        let recipe = &blueprint.recipes[i];
        if let Some(wait_time) = (0..recipe.len())
            .filter_map(|ore_type| {
                if recipe[ore_type] == 0 {
                    None
                } else {
                    if recipe[ore_type] <= state.ores[ore_type] {
                        Some(0)
                    } else if state.robots[ore_type] == 0 {
                        Some(max_time as u16 + 1)
                    } else {
                        Some(
                            (recipe[ore_type] - state.ores[ore_type] + state.robots[ore_type] - 1)
                                / state.robots[ore_type],
                        )
                    }
                }
            })
            .max()
        {
            let time_finished = state.time + wait_time as usize + 1;
            if time_finished < max_time {
                let mut new_ores = [0; 4];
                let mut new_robots = [0; 4];
                for o in 0..4 {
                    new_ores[o] = state.ores[o] + state.robots[o] * (wait_time + 1) - recipe[o];
                    new_robots[o] = state.robots[o] + if o == i { 1 } else { 0 };
                }
                max_geodes = std::cmp::max(
                    max_geodes,
                    recurse_simulation(
                        blueprint,
                        State {
                            ores: new_ores,
                            robots: new_robots,
                            time: time_finished,
                        },
                        max_time,
                        max_robots,
                    ),
                );
            }
        }
    }
    if max_geodes == 0 {
        state.ores[3] + state.robots[3] * (max_time - state.time) as u16
    } else {
        max_geodes
    }
}

fn simulate_blueprint(blueprint: &Blueprint, max_time: usize) -> OreAmount {
    let mut max_robots = [u16::max_value(); 4];
    for i in 0..3 {
        max_robots[i] = blueprint.recipes.iter().map(|r| r[i]).max().unwrap();
    }
    recurse_simulation(
        blueprint,
        State {
            ores: [0; 4],
            robots: [1, 0, 0, 0],
            time: 0,
        },
        max_time,
        &max_robots,
    )
}

fn main() {
    let blueprints = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_blueprint)
        .collect::<Vec<_>>();
    println!(
        "{}",
        blueprints
            .iter()
            .enumerate()
            .map(|(i, b)| simulate_blueprint(b, 24) as usize * (i + 1))
            .sum::<usize>()
    );
    println!(
        "{}",
        blueprints
            .iter()
            .take(3)
            .map(|b| simulate_blueprint(b, 32) as usize)
            .product::<usize>()
    );
}
