struct Coords {
    x: i64,
    y: i64,
}

struct Sensor {
    coords: Coords,
}

struct Beacon {
    coords: Coords,
}

fn parse_line(line: String) -> (Sensor, Beacon) {
    let mut iter = line.split_ascii_whitespace();
    let sensor_x = iter.nth(2).unwrap();
    let sensor_y = iter.nth(0).unwrap();
    let beacon_x = iter.nth(4).unwrap();
    let beacon_y = iter.nth(0).unwrap();
    (
        Sensor {
            coords: Coords {
                x: sensor_x[2..sensor_x.len() - 1].parse::<i64>().unwrap(),
                y: sensor_y[2..sensor_y.len() - 1].parse::<i64>().unwrap(),
            },
        },
        Beacon {
            coords: Coords {
                x: beacon_x[2..beacon_x.len() - 1].parse::<i64>().unwrap(),
                y: beacon_y[2..].parse::<i64>().unwrap(),
            },
        },
    )
}

#[derive(PartialEq, Eq)]
enum IntervalEnd {
    Start(i64),
    End(i64),
}

impl PartialOrd for IntervalEnd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntervalEnd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let extract_val = |i: &IntervalEnd| match i {
            IntervalEnd::Start(e) => 2 * *e,
            IntervalEnd::End(e) => 2 * *e + 1,
        };
        extract_val(self).cmp(&extract_val(other))
    }
}

fn distance(c1: &Coords, c2: &Coords) -> u64 {
    c1.x.abs_diff(c2.x) + c1.y.abs_diff(c2.y)
}

fn count_at_row(
    sensors: &[(Sensor, Beacon)],
    row_y: i64,
    x_bounds: Option<(i64, i64)>,
) -> (u64, Option<Coords>) {
    let mut intervals = vec![];
    let mut beacons_on_row = std::collections::HashSet::new();
    let (min_x, max_x) = x_bounds.unwrap_or((i64::min_value(), i64::max_value()));
    for (sensor, beacon) in sensors {
        let d = distance(&sensor.coords, &beacon.coords);
        let distance_to_row = sensor.coords.y.abs_diff(row_y);
        if distance_to_row > d {
            continue;
        }
        if beacon.coords.y == row_y {
            beacons_on_row.insert(beacon.coords.x);
        }
        let diff = (d - distance_to_row) as i64;
        if sensor.coords.x + diff < min_x || sensor.coords.x - diff > max_x {
            continue;
        }
        intervals.push(IntervalEnd::Start(std::cmp::max(
            sensor.coords.x - diff,
            min_x,
        )));
        intervals.push(IntervalEnd::End(std::cmp::min(
            sensor.coords.x + diff,
            max_x,
        )));
    }
    intervals.sort();
    let mut active_intervals = 0;
    let mut total_count = 0;
    let mut interval_start = 0;
    let mut first_hole = None;
    for end in intervals.iter() {
        match end {
            IntervalEnd::Start(x) => {
                if active_intervals == 0 {
                    interval_start = *x;
                }
                active_intervals += 1;
            }
            IntervalEnd::End(x) => {
                active_intervals -= 1;
                if active_intervals == 0 {
                    if first_hole.is_none() && *x != max_x {
                        first_hole = Some(Coords { x: x + 1, y: row_y });
                    }
                    total_count += (x - interval_start + 1) as u64;
                }
            }
        }
    }
    (total_count - beacons_on_row.len() as u64, first_hole)
}

fn main() {
    let sensors = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .collect::<Vec<_>>();
    println!("{}", count_at_row(&sensors, 2000000, None).0);
    for y in 0..4000000 {
        if let (count, Some(hole)) = count_at_row(&sensors, y, Some((0, 4000000))) {
            if count == 4000000 {
                println!("{}", hole.x * 4000000 + hole.y);
            }
        }
    }
}
