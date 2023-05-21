use std::collections::HashMap;

use common::names::{Animal, Color, Name};
use once_cell::sync::Lazy;
use rand::Rng;

/// All possible names
static NAMES: Lazy<Vec<Name>> = Lazy::new(|| {
    let mut names = Vec::new();

    for color in Color::iter() {
        for animal in Animal::iter() {
            names.push(Name { color, animal });
        }
    }

    names
});

/// A collection of names
///
/// Underlying data structure is two HashMaps, one from k to name, and one from name to k
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Names<K>
where
    K: std::hash::Hash + Eq + Clone,
{
    names: HashMap<K, Name>,
    rev: HashMap<Name, K>,
}

impl<K> Names<K>
where
    K: std::hash::Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            names: HashMap::with_capacity(NAMES.len()),
            rev: HashMap::with_capacity(NAMES.len()),
        }
    }

    pub fn get(&self, k: &K) -> Option<&Name> {
        self.names.get(&k)
    }

    pub fn insert(&mut self, k: K, name: Name) {
        self.names.insert(k.clone(), name);
        self.rev.insert(name, k);
    }

    pub fn remove(&mut self, k: &K) {
        if let Some(name) = self.names.remove(k) {
            self.rev.remove(&name);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &Name)> {
        self.names.iter()
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.names.contains_key(k)
    }

    pub fn contains_name(&self, name: &Name) -> bool {
        self.rev.contains_key(name)
    }

    /// Creates a new name that is not already in the collection
    ///
    /// Returns None if all names have been taken
    pub fn generate(&mut self) -> Option<Name> {
        let mut rng = rand::thread_rng();

        // Try generating a random name 10 times
        for _ in 0..10 {
            let name = rng.gen();

            // If the name is not already in the collection, return it
            if !self.contains_name(&name) {
                return Some(name);
            }
        }

        // If we couldn't find a name, search the entire collection for a name that isn't taken
        for name in NAMES.iter() {
            if !self.contains_name(name) {
                return Some(*name);
            }
        }

        None
    }
}

impl<K> Extend<(K, Name)> for Names<K>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn extend<T: IntoIterator<Item = (K, Name)>>(&mut self, iter: T) {
        for (k, name) in iter {
            self.insert(k, name);
        }
    }
}

impl<K> FromIterator<(K, Name)> for Names<K>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn from_iter<T: IntoIterator<Item = (K, Name)>>(iter: T) -> Self {
        let mut names = Self::new();
        names.extend(iter);
        names
    }
}

impl<K> Default for Names<K>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
