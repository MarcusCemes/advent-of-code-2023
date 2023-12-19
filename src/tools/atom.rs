use std::{collections::HashMap, hash::Hash};

pub type Atom = usize;

pub struct AtomTable<T: Eq + Hash> {
    counter: usize,
    atoms: HashMap<T, usize>,
}

impl<T: Eq + Hash> AtomTable<T> {
    pub fn new() -> Self {
        Self {
            counter: 0,
            atoms: HashMap::new(),
        }
    }

    pub fn create(&mut self, key: T) -> usize {
        match self.atoms.try_insert(key, self.counter) {
            Ok(atom) => {
                self.counter += 1;
                *atom
            }
            Err(occupied) => *occupied.entry.get(),
        }
    }

    pub fn get(&self, key: T) -> Option<usize> {
        self.atoms.get(&key).copied()
    }
}

impl<T: Eq + Hash> Default for AtomTable<T> {
    fn default() -> Self {
        Self::new()
    }
}
