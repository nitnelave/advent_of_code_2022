type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

struct Section {
    start: u32,
    // Exclusive end.
    end: u32,
}

struct Assignment(Section, Section);

fn parse_section(section: &str) -> Result<Section> {
    let (start, end) = section.split_once('-').ok_or("Section with no '-'")?;
    Ok(Section {
        start: start.parse::<u32>().map_err(|_| "Invalid int")?,
        end: end.parse::<u32>().map_err(|_| "Invalid int")? + 1,
    })
}

fn parse_line<S: AsRef<str>>(line: S) -> Result<Assignment> {
    let (left, right) = line
        .as_ref()
        .split_once(',')
        .ok_or("No ',' found for assignment")?;
    Ok(Assignment(parse_section(left)?, parse_section(right)?))
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        if self.start <= other.start {
            other.start < self.end
        } else {
            self.start < other.end
        }
    }
}

impl Assignment {
    fn has_full_overlap(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }
    fn has_any_overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

fn main() {
    let assignments = std::io::stdin()
        .lines()
        .map(std::result::Result::unwrap)
        .map(parse_line)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    println!(
        "{}",
        assignments.iter().filter(|a| a.has_full_overlap()).count()
    );
    println!(
        "{}",
        assignments.iter().filter(|a| a.has_any_overlap()).count()
    );
}
