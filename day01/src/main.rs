use std::{io::Read, mem::MaybeUninit};

struct UninitAssoc<T>(T);
impl<T> UninitAssoc<T> {
    const UNINIT: MaybeUninit<T> = MaybeUninit::uninit();
}

struct TopN<const N: usize, T: Ord> {
    elements: [T; N],
}

impl<const N: usize, T: Ord> TopN<N, T> {
    fn push(&mut self, mut val: T) {
        for mut v in &mut self.elements {
            if val > *v {
                std::mem::swap(&mut val, &mut v);
            }
        }
    }

    fn max(&self) -> &T {
        self.elements.iter().max().unwrap()
    }

    fn top_n(&self) -> &[T; N] {
        &self.elements
    }
}

impl<const N: usize, T: Ord + std::fmt::Debug> FromIterator<T> for TopN<N, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut topn = {
            let mut arr = [UninitAssoc::UNINIT; N];
            let mut i = 0;
            iter.by_ref().take(N).for_each(|v| {
                arr[i].write(v);
                i += 1;
            });
            assert!(i == N, "Not enough elements");
            TopN {
                elements: arr.map(|e| unsafe { e.assume_init() }),
            }
        };
        iter.for_each(|v| topn.push(v));
        topn
    }
}

fn read_stdin() -> Result<&'static str, ()> {
    static mut INPUT_BUFFER: &'static mut [u8] = &mut [0; 100000];
    let mut buffer_size = 0;
    let mut stdin = std::io::stdin();
    loop {
        let read = unsafe {
            stdin
                .read(&mut INPUT_BUFFER[buffer_size..])
                .map_err(|_| ())?
        };
        if read == 0 {
            return unsafe {
                Ok(std::str::from_utf8(&INPUT_BUFFER[..buffer_size]).map_err(|_| ())?)
            };
        }
        buffer_size += read;
    }
}

fn main() {
    let contents = read_stdin().unwrap();
    let top_3 = contents
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|l| str::parse::<u64>(l).unwrap_or(0))
                .sum()
        })
        .collect::<TopN<3, u64>>();
    println!("Max calories: {}", top_3.max());
    println!("Top 3 calories: {}", top_3.top_n().iter().sum::<u64>());
}
