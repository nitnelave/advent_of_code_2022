fn mix(numbers: &[(usize, i64)], num_iterations: usize, decryption_key: i64) -> i64 {
    let mut list = numbers.to_vec();
    for _ in 0..num_iterations {
        for to_add in numbers.iter().copied() {
            let index = list.iter().position(|n| *n == to_add).unwrap();
            list.remove(index);
            let new_index =
                (index as i64 + to_add.1 * decryption_key).rem_euclid(list.len() as i64);
            list.insert(new_index as usize, to_add);
        }
    }
    let index_0 = list.iter().position(|n| n.1 == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| list.get((index_0 + i) % list.len()).unwrap().1 * decryption_key)
        .sum::<i64>()
}

fn main() {
    let numbers = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();
    println!("{}", mix(&numbers, 1, 1));
    println!("{}", mix(&numbers, 10, 811589153));
}
