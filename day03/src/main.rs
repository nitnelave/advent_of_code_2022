#![feature(array_chunks, array_methods)]

type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;
#[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
struct Letter(u8);
struct Pocket(std::collections::BTreeSet<Letter>);
struct BackPack(Pocket, Pocket);

impl TryFrom<&u8> for Letter {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        const LOWER_A: u8 = b'a';
        const UPPER_A: u8 = b'A';
        const LOWER_Z: u8 = b'z';
        const UPPER_Z: u8 = b'Z';
        match value {
            LOWER_A..=LOWER_Z => Ok(Letter(value - LOWER_A + 1)),
            UPPER_A..=UPPER_Z => Ok(Letter(value - UPPER_A + 27)),
            _ => Err("Invalid letter"),
        }
    }
}

impl Letter {
    fn to_integer(self) -> u32 {
        self.0 as u32
    }
}

impl Pocket {
    fn common_letter(&self, other_pocket: &Self) -> Result<Letter> {
        let mut intersection = self.0.intersection(&other_pocket.0);
        let value = intersection.next().ok_or("No common letter")?;
        if intersection.next().is_some() {
            Err("Too many common letters")
        } else {
            Ok(*value)
        }
    }
}

impl BackPack {
    fn common_letter(&self) -> Result<Letter> {
        self.0.common_letter(&self.1)
    }

    fn all_items(&self) -> impl Iterator<Item = &Letter> {
        self.0 .0.union(&self.1 .0)
    }
}

fn parse_pocket(contents: &[u8]) -> Result<Pocket> {
    Ok(Pocket(
        contents
            .iter()
            .map(Letter::try_from)
            .collect::<Result<_>>()?,
    ))
}

fn parse_backpack<S: AsRef<str>>(line: S) -> Result<BackPack> {
    let len = line.as_ref().len();
    if len % 2 != 0 {
        return Err("Odd line length");
    }
    let (left, right) = line.as_ref().as_bytes().split_at(len / 2);
    Ok(BackPack(parse_pocket(left)?, parse_pocket(right)?))
}

struct SetIntersection<'a, const N: usize, T, I>
where
    T: 'a + Ord,
    I: Iterator<Item = &'a T>,
{
    iterators: [std::iter::Peekable<I>; N],
}

impl<'a, const N: usize, T, I> SetIntersection<'a, N, T, I>
where
    T: 'a + Ord,
    I: Iterator<Item = &'a T>,
{
    fn new(iterators: [I; N]) -> Self {
        Self {
            iterators: iterators.map(Iterator::peekable),
        }
    }
}

impl<'a, const N: usize, T, I> Iterator for SetIntersection<'a, N, T, I>
where
    T: 'a + Ord,
    I: Iterator<Item = &'a T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if N == 0 {
            return None;
        }
        loop {
            let next = match self.iterators[0].next() {
                None => return None,
                Some(v) => v,
            };
            let mut all_matches = true;
            for it in &mut self.iterators[1..] {
                loop {
                    use std::cmp::Ordering;
                    match it.peek().map(|v| v.cmp(&next)) {
                        // Keep iterating on this iterator, we haven't caught up.
                        Some(Ordering::Less) => {
                            it.next();
                        }
                        // We caught up and that's a match.
                        Some(Ordering::Equal) => break,
                        // We passed it. No match.
                        Some(Ordering::Greater) => {
                            all_matches = false;
                            break;
                        }
                        // Got to the end, no more matches.
                        None => return None,
                    }
                }
                if !all_matches {
                    break;
                }
            }
            if all_matches {
                return Some(next);
            }
            // No match, advance the first iterator again.
        }
    }
}

fn get_group_badge(packs: &[BackPack; 3]) -> Result<Letter> {
    let mut group_badges = SetIntersection::new(packs.each_ref().map(|p| p.all_items()));
    let value = group_badges.next().ok_or("No badge for group")?;
    if group_badges.next().is_some() {
        Err("Multiple badges for group")
    } else {
        Ok(*value)
    }
}

fn main() {
    let backpacks = std::io::stdin()
        .lines()
        .map(std::result::Result::unwrap)
        .map(parse_backpack)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    println!(
        "Part 1: {}",
        backpacks
            .iter()
            .map(|p| p.common_letter().map(Letter::to_integer))
            .sum::<Result<u32>>()
            .unwrap()
    );
    println!(
        "Part 2: {}",
        backpacks
            .array_chunks::<3>()
            .map(|group| get_group_badge(group).map(Letter::to_integer))
            .sum::<Result<u32>>()
            .unwrap()
    );
}
