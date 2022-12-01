#![feature(binary_heap_into_iter_sorted)]

fn main() {
    let path_name = std::env::args().nth(1).expect("Please give an input file");
    let path = std::path::Path::new(&path_name);
    let heap = std::fs::read_to_string(path)
        .expect("Error reading")
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
