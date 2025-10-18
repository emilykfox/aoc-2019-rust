use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Memory {
    table: HashMap<usize, isize>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            table: HashMap::new(),
        }
    }

    pub fn load(&mut self, program: &[isize]) {
        self.table = program
            .iter()
            .copied()
            .enumerate()
            .collect::<HashMap<_, _>>();
    }
}

impl Index<usize> for Memory {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        self.table.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.table.entry(index).or_default()
    }
}
