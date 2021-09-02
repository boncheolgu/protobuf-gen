use std::fmt;
use std::iter::FromIterator;
use std::ops::{Deref, Index, IndexMut};
use std::result;

pub type Key = usize;

/// This is stable, because removing elements does not affect the already allocated keys.
/// Use this, if you want stable and safe keys for objects.
#[derive(Clone, Serialize, Deserialize)]
pub struct StableVec<T> {
    slots: Vec<Option<T>>,
    free_list: Vec<Key>,
}

impl<T> Default for StableVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for StableVec<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        write!(f, "{:?}", self.iter().collect::<Vec<_>>())
    }
}

impl<T, I> Index<I> for StableVec<T>
where
    I: Into<Key> + fmt::Display + Copy,
{
    type Output = T;
    fn index(&self, k: I) -> &Self::Output {
        self.slots[k.into()].as_ref().unwrap_or_else(|| panic!("no entry for {}", k))
    }
}

impl<T, I> IndexMut<I> for StableVec<T>
where
    I: Into<Key> + fmt::Display + Copy,
{
    fn index_mut(&mut self, k: I) -> &mut Self::Output {
        self.slots[k.into()].as_mut().unwrap_or_else(|| panic!("no entry for {}", k))
    }
}

impl<T> PartialEq<StableVec<T>> for StableVec<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.slots.iter().zip(other.slots.iter()).all(|(x, y)| x == y)
    }
}

impl<T> FromIterator<T> for StableVec<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut v = Self::new();
        for value in iter.into_iter() {
            v.insert(value);
        }
        v
    }
}

impl<T> StableVec<T> {
    pub fn new() -> Self {
        Self { slots: Vec::new(), free_list: Vec::new() }
    }

    pub fn insert(&mut self, v: T) -> Key {
        if let Some(key) = self.free_list.pop() {
            self.slots[key] = Some(v);
            key
        } else {
            let key = self.slots.len();
            self.slots.push(Some(v));
            key
        }
    }

    pub fn insert_at(&mut self, k: Key, v: T) -> Option<T> {
        if let Some(pos) = self.free_list.iter().position(|&x| x == k) {
            self.free_list.remove(pos);
        }
        self.slots[k].replace(v)
    }

    pub fn push_back(&mut self, v: T) -> Key {
        let key = self.slots.len();
        self.slots.push(Some(v));
        key
    }

    pub fn remove(&mut self, k: Key) -> Option<T> {
        if self.slots[k].is_some() {
            self.free_list.push(k);
        }
        self.slots[k].take()
    }

    pub fn get(&self, k: Key) -> Option<&T> {
        if let Some(v) = self.slots.get(k) {
            v.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, k: Key) -> Option<&mut T> {
        if let Some(v) = self.slots.get_mut(k) {
            v.as_mut()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.slots.len() - self.free_list.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = Key> + '_ {
        (0..self.slots.len()).filter(move |&key| self.slots[key].is_some())
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.slots.iter().filter_map(Option::as_ref)
    }

    pub fn contains_key(&self, key: Key) -> bool {
        key < self.slots.len() && self.slots[key].is_some()
    }

    pub fn drain_all(&mut self) -> impl Iterator<Item = (Key, T)> + '_ {
        self.free_list.clear();
        self.slots.drain(..).enumerate().filter_map(|(key, slot)| slot.map(|value| (key, value)))
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.slots.iter_mut().filter_map(Option::as_mut)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Key, &T)> {
        self.keys().zip(self.values())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Key, &mut T)> {
        (self.slots.iter_mut().enumerate())
            .filter_map(|(key, slot)| slot.as_mut().map(|value| (key, value)))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn transaction(&mut self) -> Transaction<'_, T> {
        Transaction::new(self)
    }

    pub fn with_transaction<F, R, E>(&mut self, f: F) -> result::Result<R, E>
    where
        F: Fn(&mut Transaction<'_, T>) -> result::Result<R, E>,
    {
        let mut trx = Transaction::new(self);
        let result = f(&mut trx);
        if result.is_err() {
            trx.rollback();
        }
        result
    }
}
