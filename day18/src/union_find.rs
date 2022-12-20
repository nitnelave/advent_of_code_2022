struct Cell {
    parent: u16,
    rank: u16,
}

pub struct UnionFind {
    classes: Vec<Cell>,
}

impl UnionFind {
    fn at(&mut self, index: u16) -> &mut Cell {
        &mut self.classes[index as usize]
    }

    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    fn grow(&mut self, size: u16) {
        if self.classes.len() < size as usize {
            let mut i = self.classes.len() as u16;
            self.classes.resize_with(size.into(), || {
                i += 1;
                Cell {
                    parent: i - 1,
                    rank: 0,
                }
            });
        }
    }

    pub fn find_mut(&mut self, c: u16) -> u16 {
        self.grow(c + 1);
        fn find_mut_rec(s: &mut UnionFind, c: u16) -> u16 {
            if s.at(c).parent == c {
                c
            } else {
                let parent = s.at(c).parent;
                assert_ne!(parent, c);
                let rep = find_mut_rec(s, parent);
                s.at(c).parent = rep;
                s.at(c).rank = 0;
                rep
            }
        }
        find_mut_rec(self, c)
    }

    pub fn union(&mut self, c1: u16, c2: u16) {
        let mut i1 = self.find_mut(c1);
        let mut i2 = self.find_mut(c2);
        if i1 == i2 {
            return;
        }
        if self.at(i1).rank < self.at(i2).rank {
            std::mem::swap(&mut i1, &mut i2);
        }
        self.at(i2).parent = i1;
        self.at(i1).rank += u16::from(self.at(i1).rank == self.at(i2).rank);
    }
}
