#![feature(binary_heap_into_iter_sorted)]

use std::io::Read;

fn main() {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents).unwrap();
    let heap = contents
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|l| str::parse::<u64>(l).unwrap_or(0))
                .sum()
        })
        .collect::<std::collections::BinaryHeap<u64>>();
    println!("Max calories: {}", heap.peek().expect("Empty heap"));
    println!(
        "Top 3 calories: {}",
        heap.into_iter_sorted().take(3).sum::<u64>()
    );
}
