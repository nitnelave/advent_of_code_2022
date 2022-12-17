struct Piece {
    width: usize,
    height: usize,
    points: [bool; 4 * 4],
    low_edge: [usize; 4],
}

#[rustfmt::skip]
const PIECES: [Piece; 5] = [
    Piece {
        width: 4,
        height: 1,
        points: [
            true, true, true, true,
            false, false, false, false,
            false, false, false, false,
            false, false, false, false,
        ],
        low_edge: [0, 0, 0, 0],
    },
    Piece {
        width: 3,
        height: 3,
        points: [
            false, true, false, false,
            true, true, true, false,
            false, true, false, false,
            false, false, false, false,
        ],
        low_edge: [1, 0, 1, 0],
    },
    Piece {
        width: 3,
        height: 3,
        points: [ // The pieces are upside down.
            true, true, true, false,
            false, false, true, false,
            false, false, true, false,
            false, false, false, false,
        ],
        low_edge: [0, 0, 0, 0],
    },
    Piece {
        width: 1,
        height: 4,
        points: [
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
        ],
        low_edge: [0, 0, 0, 0],
    },
    Piece {
        width: 2,
        height: 2,
        points: [
            true, true, false, false,
            true, true, false, false,
            false, false, false, false,
            false, false, false, false,
        ],
        low_edge: [0, 0, 0, 0],
    },
];

#[derive(Hash, PartialEq, Eq, Clone)]
struct Row([bool; 7]);

impl Default for Row {
    fn default() -> Self {
        Self([false; 7])
    }
}

fn move_rock(piece: &Piece, jet: bool, rows: &[Row], rock_x: &mut usize, rock_y: usize) {
    if jet {
        // to the right
        if piece.width + *rock_x == 7 {
            return;
        }
        for x in 0..piece.width {
            for y in 0..piece.height {
                if piece.points[y * 4 + x] && rows[rock_y + y].0[*rock_x + x + 1] {
                    return;
                }
            }
        }
        *rock_x += 1;
    } else {
        if *rock_x == 0 {
            return;
        }
        for x in 0..piece.width {
            for y in 0..piece.height {
                if piece.points[y * 4 + x] && rows[rock_y + y].0[*rock_x + x - 1] {
                    return;
                }
            }
        }
        *rock_x -= 1;
    }
}

fn add_one_piece<Jets: Iterator<Item = (usize, bool)>>(
    rows: &mut Vec<Row>,
    jets: &mut Jets,
    piece: &Piece,
    top_rock: &mut usize,
) {
    rows.resize_with(*top_rock + piece.height + 4, Row::default);
    let mut rock_x = 2;
    let mut rock_y = *top_rock + 4;
    loop {
        move_rock(piece, jets.next().unwrap().1, rows, &mut rock_x, rock_y);

        if (0..piece.width).any(|x| rows[rock_y + piece.low_edge[x] - 1].0[rock_x + x]) {
            for x in 0..piece.width {
                for y in 0..piece.height {
                    if !piece.points[y * 4 + x] {
                        continue;
                    }
                    *top_rock = std::cmp::max(*top_rock, rock_y + y);
                    assert!(!rows[rock_y + y].0[rock_x + x]);
                    rows[rock_y + y].0[rock_x + x] = true;
                }
            }
            break;
        } else {
            rock_y -= 1;
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    last_row: Row,
    piece_index: usize,
    jet_index: usize,
}

fn main() {
    let jets = {
        let mut contents = String::new();
        use std::io::Read;
        std::io::stdin().read_to_string(&mut contents).unwrap();
        contents
    };
    assert_eq!(jets.as_bytes()[jets.len() - 1], b'\n');
    let mut jets_iter = jets.as_bytes()[..jets.as_bytes().len() - 1]
        .iter()
        .enumerate()
        .cycle()
        .map(|(i, c)| (i, *c == b'>'))
        .peekable();
    let mut rows = vec![Row([true; 7])];
    let mut piece_iter = PIECES.iter().enumerate().cycle().peekable();
    let mut top_rock = 0;
    for _ in 0..2022 {
        add_one_piece(
            &mut rows,
            &mut jets_iter,
            piece_iter.next().unwrap().1,
            &mut top_rock,
        );
    }
    println!("{}", top_rock);
    let (top_rock_to_add, i) = {
        // Find the cycle.
        let mut i = 2021;
        let mut states = std::collections::HashMap::new();
        loop {
            i += 1;
            let prev_entry = states
                .entry(State {
                    last_row: rows[top_rock].clone(),
                    piece_index: piece_iter.peek().unwrap().0,
                    jet_index: jets_iter.peek().unwrap().0,
                })
                .or_insert((i, top_rock));
            if prev_entry.0 != i {
                let diff_i = i - prev_entry.0;
                let diff_rock = top_rock - prev_entry.1;
                let num_cycles = (1000000000000usize - i) / diff_i;
                i += num_cycles * diff_i;
                break (num_cycles * diff_rock, i);
            }
            add_one_piece(
                &mut rows,
                &mut jets_iter,
                piece_iter.next().unwrap().1,
                &mut top_rock,
            );
        }
    };
    // Do the last few steps after the cycle.
    for _ in i..1000000000000usize {
        add_one_piece(
            &mut rows,
            &mut jets_iter,
            piece_iter.next().unwrap().1,
            &mut top_rock,
        );
    }
    println!("{}", top_rock + top_rock_to_add);
}
