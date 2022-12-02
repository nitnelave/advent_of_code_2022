type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Lose = -1,
    Draw = 0,
    Win = 1,
}

impl From<i32> for Move {
    fn from(m: i32) -> Self {
        match (m + 3) % 3 {
            0 => Move::Scissors,
            1 => Move::Rock,
            2 => Move::Paper,
            _ => unreachable!(),
        }
    }
}

impl From<i32> for Outcome {
    fn from(m: i32) -> Self {
        match (m + 3) % 3 {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Lose,
            _ => unreachable!(),
        }
    }
}

impl Outcome {
    fn to_move(self, theirs: Move) -> Move {
        (theirs as i32 + self as i32).into()
    }
}

impl TryFrom<&str> for Move {
    fn try_from(mov: &str) -> Result<Self> {
        if mov.len() > 1 {
            return Err("Too long move");
        }
        match mov.chars().next() {
            Some('A' | 'X') => Ok(Move::Rock),
            Some('B' | 'Y') => Ok(Move::Paper),
            Some('C' | 'Z') => Ok(Move::Scissors),
            None => Err("Empty move"),
            _ => Err("Invalid move"),
        }
    }

    type Error = Error;
}

impl TryFrom<&str> for Outcome {
    fn try_from(mov: &str) -> Result<Outcome> {
        if mov.len() > 1 {
            return Err("Too long outcome");
        }
        match mov.chars().next() {
            Some('X') => Ok(Outcome::Lose),
            Some('Y') => Ok(Outcome::Draw),
            Some('Z') => Ok(Outcome::Win),
            None => Err("Empty outcome"),
            _ => Err("Invalid outcome"),
        }
    }
    type Error = Error;
}

fn parse_line<S: AsRef<str>>(line: S) -> Result<(Move, Move, Outcome)> {
    let (theirs, yours) = line.as_ref().split_once(' ').ok_or("No space")?;
    Ok((theirs.try_into()?, yours.try_into()?, yours.try_into()?))
}

fn to_outcome(theirs: Move, yours: Move) -> Outcome {
    (yours as i32 - theirs as i32).into()
}

fn to_score(theirs: Move, yours: Move) -> i32 {
    (to_outcome(theirs, yours) as i32 * 3) + 3 + yours as i32
}

fn main() {
    println!(
        "Score: {:?}",
        std::io::stdin()
            .lines()
            .map(std::result::Result::unwrap)
            .map(parse_line)
            .map(std::result::Result::unwrap)
            .map(|(t, y, o)| (to_score(t, y), to_score(t, o.to_move(t))))
            .fold((0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_score() {
        assert_eq!(to_score(Move::Rock, Move::Rock), 4);
        assert_eq!(to_score(Move::Rock, Move::Paper), 8);
        assert_eq!(to_score(Move::Rock, Move::Scissors), 3);
        assert_eq!(to_score(Move::Scissors, Move::Rock), 7);
        assert_eq!(to_score(Move::Scissors, Move::Paper), 2);
        assert_eq!(to_score(Move::Scissors, Move::Scissors), 6);
    }
}
