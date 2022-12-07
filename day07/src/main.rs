type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

enum TargetDirectory {
    Up,
    Down,
}

enum Instruction {
    Cd(TargetDirectory),
    Ls,
    FileListing(u64),
    DirListing,
}

fn line_to_instruction<S: AsRef<str>>(input: S) -> Result<Instruction> {
    let input = input.as_ref();
    if let Some(command) = input.strip_prefix("$ ") {
        if command == "ls" {
            Ok(Instruction::Ls)
        } else if let Some(target) = command.strip_prefix("cd ") {
            Ok(Instruction::Cd(if target == "/" {
                return Err("cd / not supported");
            } else if target == ".." {
                TargetDirectory::Up
            } else {
                TargetDirectory::Down
            }))
        } else {
            Err("Unrecognized command")
        }
    } else if input.starts_with("dir ") {
        Ok(Instruction::DirListing)
    } else if let Some((size, _)) = input.split_once(' ') {
        Ok(Instruction::FileListing(
            size.parse::<u64>().map_err(|_| "Invalid number")?,
        ))
    } else {
        Err("Unrecognized line")
    }
}

struct File {
    size: u64,
}

struct Directory {
    dirs: Vec<Directory>,
    size: u64,
}

#[derive(Default)]
struct DirectoryBuilder {
    current_files: Vec<File>,
    current_dirs: Vec<Directory>,
    current_builder: Box<Option<DirectoryBuilder>>,
}

enum ApplyResult {
    Applied(DirectoryBuilder),
    Finished(Directory),
}

impl DirectoryBuilder {
    fn finish(mut self) -> Directory {
        if let Some(dir) = self.current_builder.take() {
            let last_dir = dir.finish();
            self.current_dirs.push(last_dir);
        }
        let size = self.current_files.iter().map(|f| f.size).sum::<u64>()
            + self.current_dirs.iter().map(|d| d.size).sum::<u64>();
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
            let maybe_builder = std::mem::replace(&mut *self.current_builder, None);
            *self.current_builder = if let Some(builder) = maybe_builder {
                match builder.apply(instruction) {
                    ApplyResult::Applied(builder) => Some(builder),
                    ApplyResult::Finished(dir) => {
                        self.current_dirs.push(dir);
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
                Instruction::Cd(TargetDirectory::Down) => {
                    assert!(self.current_builder.is_none());
                    *self.current_builder = Some(DirectoryBuilder::default());
                }
                Instruction::Ls | Instruction::DirListing => (),
                Instruction::FileListing(size) => self.current_files.push(File { size }),
            }
        }
        ApplyResult::Applied(self)
    }
}

fn compute_sum_of_sizes(dir: &Directory, max_size: u64) -> u64 {
    dir.dirs
        .iter()
        .map(|d| compute_sum_of_sizes(d, max_size))
        .sum::<u64>()
        + if dir.size < max_size { dir.size } else { 0 }
}

fn find_smallest_dir_above(min_size: u64, dir: &Directory) -> u64 {
    let best_subdir = dir
        .dirs
        .iter()
        .filter(|d| d.size >= min_size) // short-circuit
        .map(|d| find_smallest_dir_above(min_size, d))
        .min();
    assert!(dir.size >= min_size);
    if let Some(s) = best_subdir {
        if dir.size >= s {
            return s;
        }
    }
    dir.size
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
    let smallest_dir = find_smallest_dir_above(min_size, &root);
    println!("{}", smallest_dir);
}
