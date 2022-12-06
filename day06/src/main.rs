use std::{collections::HashMap, io::Read};

struct PacketStartIterator<I: Iterator<Item = u8>, const N: usize> {
    position: usize,
    iter: I,
    packet_buffer: [u8; N],
}

impl<I: Iterator<Item = u8>, const N: usize> Iterator for PacketStartIterator<I, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            let v = match self.iter.next() {
                None => return None,
                Some(v) => v,
            };
            self.packet_buffer[self.position % self.packet_buffer.len()] = v;
            self.position += 1;
            if self.position < self.packet_buffer.len() {
                continue;
            }
            for i in 0..(self.packet_buffer.len() - 1) {
                for j in (i + 1)..self.packet_buffer.len() {
                    if self.packet_buffer[i] == self.packet_buffer[j] {
                        continue 'outer;
                    }
                }
            }
            return Some(self.position);
        }
    }
}

impl<I: Iterator<Item = u8>, const N: usize> From<I> for PacketStartIterator<I, N> {
    fn from(it: I) -> Self {
        Self {
            iter: it,
            position: 0,
            packet_buffer: [0; N],
        }
    }
}

struct MessageStartIterator<I: Iterator<Item = u8>, const N: usize> {
    position: usize,
    iter: I,
    packet_buffer: [u8; N],
    byte_counts: HashMap<u8, u8>,
    different_byte_count: u8,
}

impl<I: Iterator<Item = u8>, const N: usize> Iterator for MessageStartIterator<I, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let v = match self.iter.next() {
                None => return None,
                Some(v) => v,
            };
            let entry = self.byte_counts.entry(v).or_default();
            *entry += 1u8;
            if *entry == 1 {
                self.different_byte_count += 1;
            }
            let previous_value = self.packet_buffer[self.position % self.packet_buffer.len()];
            self.packet_buffer[self.position % self.packet_buffer.len()] = v;
            self.position += 1;
            if self.position < self.packet_buffer.len() + 1 {
                continue;
            }
            let previous_count = self.byte_counts.get_mut(&previous_value).unwrap();
            if *previous_count == 1 {
                self.different_byte_count -= 1;
            }
            *previous_count -= 1;
            /*
            assert_eq!(
                self.byte_counts.values().sum::<u8>() as usize,
                self.packet_buffer.len()
            );
            */
            if self.different_byte_count == self.packet_buffer.len() as u8 {
                return Some(self.position);
            }
        }
    }
}

impl<I: Iterator<Item = u8>, const N: usize> From<I> for MessageStartIterator<I, N> {
    fn from(it: I) -> Self {
        Self {
            iter: it,
            position: 0,
            packet_buffer: [0; N],
            byte_counts: HashMap::new(),
            different_byte_count: 0,
        }
    }
}

fn main() {
    let contents = {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .expect("Error reading stdin");
        buf
    };
    println!(
        "First packet start: {}",
        Into::<PacketStartIterator<_, 4>>::into(contents.bytes())
            .next()
            .expect("No packet start detected")
    );
    println!(
        "First message start: {}",
        Into::<MessageStartIterator<_, 14>>::into(contents.bytes())
            .next()
            .expect("No message start detected")
    );
}
