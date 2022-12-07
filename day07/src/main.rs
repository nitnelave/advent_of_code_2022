type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum TargetDirectory {
    Up,
    SubDirectory(String),
}

#[derive(Debug)]
enum Instruction {
    Cd(TargetDirectory),
    Ls,
    FileListing(u64),
    DirListing(String),
}

fn line_to_instruction<S: AsRef<str>>(input: S) -> Result<Instruction> {
    let input = input.as_ref();
    if input.starts_with("$ ") {
        let command = &input[2..];
        if command == "ls" {
            Ok(Instruction::Ls)
        } else if command.starts_with("cd ") {
            let target = &input[5..];
            Ok(Instruction::Cd(if target == "/" {
                return Err("cd / not supported");
            } else if target == ".." {
                TargetDirectory::Up
            } else {
                TargetDirectory::SubDirectory(target.to_owned())
            }))
        } else {
            Err("Unrecognized command")
        }
    } else if input.starts_with("dir ") {
        Ok(Instruction::DirListing(input[4..].to_owned()))
    } else {
        if let Some((size, _)) = input.split_once(' ') {
            Ok(Instruction::FileListing(
                size.parse::<u64>().map_err(|_| "Invalid number")?,
            ))
        } else {
            Err("Unrecognized line")
        }
    }
}

struct File {
    size: u64,
}

struct Directory {
    dirs: Vec<Box<(String, Directory)>>,
    size: u64,
}

#[derive(Default)]
struct DirectoryBuilder {
    current_files: Vec<File>,
    current_unvisited_dirs: Vec<String>,
    current_dirs: Vec<Box<(String, Directory)>>,
    current_builder: Box<Option<(String, DirectoryBuilder)>>,
}

enum ApplyResult {
    Applied(DirectoryBuilder),
    Finished(Directory),
}

impl DirectoryBuilder {
    fn finish(mut self) -> Directory {
        assert!(self.current_unvisited_dirs.is_empty());
        if let Some(dir) = self.current_builder.take() {
            let last_dir = Box::new((dir.0, dir.1.finish()));
            self.current_dirs.push(last_dir);
        }
        let size = self.current_files.iter().map(|f| f.size).sum::<u64>()
            + self.current_dirs.iter().map(|d| d.1.size).sum::<u64>();
        Directory {
            dirs: self.current_dirs,
            size,
        }
    }
    fn apply_top(self, instruction: Instruction) -> Self {
        match self.apply(instruction) {
            ApplyResult::Applied(s) => s,
            ApplyResult::Finished(_) => unreachable!(),
        }
    }
    fn apply(mut self, instruction: Instruction) -> ApplyResult {
        if self.current_builder.is_some() {
            let maybe_builder = std::mem::replace::<Option<(String, DirectoryBuilder)>>(
                &mut *self.current_builder,
                None,
            );
            *self.current_builder = if let Some((name, builder)) = maybe_builder {
                match builder.apply(instruction) {
                    ApplyResult::Applied(builder) => Some((name, builder)),
                    ApplyResult::Finished(dir) => {
                        self.current_dirs.push(Box::new((name, dir)));
                        None
                    }
                }
            } else {
                unreachable!()
            };
        } else {
            match instruction {
                Instruction::Cd(TargetDirectory::Up) => {
                    return ApplyResult::Finished(self.finish())
                }
                Instruction::Cd(TargetDirectory::SubDirectory(name)) => {
                    assert!(self.current_builder.is_none());
                    let position = self
                        .current_unvisited_dirs
                        .iter()
                        .position(|d| *d == name)
                        .expect("Directory not expected");
                    self.current_unvisited_dirs.remove(position);
                    *self.current_builder = Some((name, DirectoryBuilder::default()));
                }
                Instruction::Ls => (),
                Instruction::FileListing(size) => self.current_files.push(File { size }),
                Instruction::DirListing(name) => self.current_unvisited_dirs.push(name),
            }
        }
        ApplyResult::Applied(self)
    }
}

fn compute_sum_of_sizes(dir: &Directory, max_size: u64) -> u64 {
    dir.dirs
        .iter()
        .map(|d| compute_sum_of_sizes(&d.1, max_size))
        .sum::<u64>()
        + if dir.size < max_size { dir.size } else { 0 }
}

fn find_smallest_dir_above<'a>(min_size: u64, dir: &'a Directory, name: &'a str) -> (u64, String) {
    let best_subdir = dir
        .dirs
        .iter()
        .filter(|d| d.1.size >= min_size) // short-circuit
        .map(|d| find_smallest_dir_above(min_size, &d.1, &d.0))
        .min_by_key(|(s, _)| *s);
    assert!(dir.size >= min_size);
    if let Some((s, n)) = best_subdir {
        if dir.size >= s {
            return (s, name.to_owned() + "/" + &n);
        }
    }
    return (dir.size, name.to_owned());
}

fn main() {
    let root = std::io::stdin()
        .lines()
        .map(std::result::Result::unwrap)
        .skip(1) // Skip "$ cd /"
        .map(line_to_instruction)
        .map(std::result::Result::unwrap)
        .fold(DirectoryBuilder::default(), DirectoryBuilder::apply_top)
        .finish();
    let sum = compute_sum_of_sizes(&root, 100000);
    println!("{}", sum);
    let min_size = 30_000_000 - (70_000_000 - root.size);
    assert!(min_size < root.size);
    let smallest_dir = find_smallest_dir_above(min_size, &root, "");
    println!("{}", smallest_dir.0);
}
