#[derive(Debug)]
struct Coords {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Sensor {
    coords: Coords,
}

#[derive(Debug)]
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

fn count_at_row(sensors: &[(Sensor, Beacon)], row_y: i64) -> u64 {
    let mut intervals = vec![];
    let mut beacons_on_row = std::collections::HashSet::new();
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
        intervals.push(IntervalEnd::Start(sensor.coords.x - diff));
        intervals.push(IntervalEnd::End(sensor.coords.x + diff));
    }
    intervals.sort();
    let mut active_intervals = 0;
    let mut total_count = 0;
    let mut interval_start = 0;
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
                    total_count += (x - interval_start + 1) as u64;
                }
            }
        }
    }
    total_count - beacons_on_row.len() as u64
}

#[derive(Debug)]
struct Segment {
    from: Coords,
    length: i64,
}

// Iterate over the ranges of elements that are equal by the key function.
struct PartitionIterator<'a, T, Key: Eq, F: FnMut(&T) -> Key> {
    slice: &'a [T],
    mapper: F,
    first_index: usize,
}

impl<'a, T, Key: Eq, F: FnMut(&T) -> Key> PartitionIterator<'a, T, Key, F> {
    fn new(slice: &'a [T], mapper: F) -> Self {
        Self {
            slice,
            mapper,
            first_index: 0,
        }
    }
}
impl<'a, T, Key: Eq, F: FnMut(&T) -> Key> Iterator for PartitionIterator<'a, T, Key, F> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_index >= self.slice.len() {
            return None;
        }
        let key = (self.mapper)(&self.slice[self.first_index]);
        for i in self.first_index + 1..self.slice.len() {
            if (self.mapper)(&self.slice[i]) != key {
                let first = self.first_index;
                self.first_index = i;
                return Some(&self.slice[first..self.first_index]);
            }
        }
        let first = self.first_index;
        self.first_index = self.slice.len();
        Some(&self.slice[first..])
    }
}

fn find_hole(sensors: &[(Sensor, Beacon)], max_coord: i64) -> Coords {
    // Get all the outer edges of the squares, and where the line would cross x=0.
    let mut positive_slope_edges = sensors
        .iter()
        .flat_map(|(Sensor { coords: s }, Beacon { coords: b })| {
            let d = distance(&s, &b) as i64 + 1;
            [
                (
                    Segment {
                        from: Coords { x: s.x - d, y: s.y },
                        length: d + 1,
                    },
                    s.y - (s.x - d),
                ),
                (
                    Segment {
                        from: Coords { x: s.x, y: s.y - d },
                        length: d + 1,
                    },
                    s.y - d - s.x,
                ),
            ]
        })
        .collect::<Vec<_>>();
    let mut negative_slope_edges = sensors
        .iter()
        .flat_map(|(Sensor { coords: s }, Beacon { coords: b })| {
            let d = distance(&s, &b) as i64 + 1;
            [
                (
                    Segment {
                        from: Coords { x: s.x - d, y: s.y },
                        length: d + 1,
                    },
                    s.y + s.x - d,
                ),
                (
                    Segment {
                        from: Coords { x: s.x, y: s.y + d },
                        length: d + 1,
                    },
                    s.y + d + s.x,
                ),
            ]
        })
        .collect::<Vec<_>>();
    // Sort by origin crossing to find the segments on the same line.
    let get_offset = |(_, offset): &(Segment, i64)| *offset;
    positive_slope_edges.sort_by_key(get_offset);
    negative_slope_edges.sort_by_key(get_offset);
    // Find segment overlaps.
    let find_common_segments = |edges, sign| {
        let mut common_segments = vec![];
        for range in PartitionIterator::new(edges, get_offset) {
            for i in 0..range.len() - 1 {
                let ri = &range[i].0;
                for j in i + 1..range.len() {
                    let rj = &range[j].0;
                    assert_eq!(range[i].1, range[j].1);
                    let min_x = std::cmp::max(ri.from.x, rj.from.x);
                    let max_x = std::cmp::min(ri.from.x + ri.length, rj.from.x + rj.length);
                    if min_x <= max_x {
                        common_segments.push(Segment {
                            from: Coords {
                                x: min_x,
                                y: ri.from.y + sign * (min_x - ri.from.x),
                            },
                            length: max_x - min_x + 1,
                        });
                    }
                }
            }
        }
        common_segments
    };
    let positive_slope_common_segments = find_common_segments(&positive_slope_edges, 1);
    let negative_slope_common_segments = find_common_segments(&negative_slope_edges, -1);
    // Find intersections between positive and negative segments.
    for sp in positive_slope_common_segments.iter() {
        for sn in negative_slope_common_segments.iter() {
            if sp.from.y > sn.from.y {
                continue;
            }
            let check_coords = |s1: &Segment, s2: &Segment, sign: i64| {
                assert!(s1.from.x <= s2.from.x);
                if s1.from.x + s1.length < s2.from.x {
                    return None;
                }
                let diff = s2.from.x - s1.from.x;
                if s1.from.y + diff * sign > s2.from.y {
                    return None;
                }
                let diff_y = s2.from.y.abs_diff(s1.from.y + diff) as i64;
                if diff_y % 2 == 1 {
                    return None;
                }
                if diff + diff_y / 2 > s1.length || diff_y / 2 > s2.length {
                    return None;
                }
                let coords = Coords {
                    x: s2.from.x + diff_y / 2,
                    y: s2.from.y - diff_y / 2 * sign,
                };
                assert_eq!(s1.from.y + diff + diff_y / 2 * sign, coords.y);
                assert_eq!(s1.from.x + diff + diff_y / 2, coords.x);
                if coords.x < 0 || coords.y < 0 || coords.x > max_coord || coords.y > max_coord {
                    return None;
                }
                Some(coords)
            };
            if let Some(c) = if sp.from.x <= sn.from.x {
                check_coords(sp, sn, 1)
            } else {
                check_coords(sn, sp, -1)
            } {
                // Check that it's not covered by another square.
                if sensors.iter().all(|(sensor, beacon)| {
                    distance(&sensor.coords, &c) > distance(&sensor.coords, &beacon.coords)
                }) {
                    return c;
                }
            }
        }
    }
    unreachable!()
}

fn main() {
    let sensors = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(parse_line)
        .collect::<Vec<_>>();
    println!("{}", count_at_row(&sensors, 2000000));
    let hole = find_hole(&sensors, 4000000);
    println!("{}", hole.x * 4000000 + hole.y);
}
