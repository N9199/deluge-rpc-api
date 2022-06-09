use std::{
    collections::{hash_set, HashSet},
    hash::{Hash, Hasher},
    mem::discriminant,
};

use serde::{ser::SerializeMap, Serialize};

#[derive(Debug)]
pub struct EnumMap<E>(HashSet<EnumWrapper<E>>);

impl<E> IntoIterator for EnumMap<E> {
    type Item = E;

    type IntoIter = IntoIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().into()
    }
}
impl<'a, E> IntoIterator for &'a EnumMap<E> {
    type Item = &'a E;

    type IntoIter = Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().into()
    }
}

impl<E> Serialize for EnumMap<E>
where
    E: SerializableEnum,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for item in self.into_iter() {
            let k = item.get_key();
            let v = item.get_value();
            map.serialize_entry(&k, &v)?;
        }
        map.end()
    }
}
impl<E> EnumMap<E> {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    pub fn insert(&mut self, item: E) -> Option<E> {
        self.0.replace(item.into()).map(|x| x.0)
    }
    pub fn get(&self, item: E) -> Option<&E> {
        self.0.get(&item.into()).map(|x| &x.0)
    }
    pub fn remove(&mut self, item: E) -> Option<E> {
        self.0.take(&item.into()).map(|x| x.0)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<E> Default for EnumMap<E> {
    fn default() -> Self {
        Self::new()
    }
}

struct EnumWrapper<E>(E);

impl<E> EnumWrapper<E> {
    pub fn into_internal(self) -> E {
        self.0
    }
    pub fn get_ref(&self) -> &E {
        &self.0
    }
}

impl<E> std::fmt::Debug for EnumWrapper<E>
where
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.0)
    }
}
impl<E> Hash for EnumWrapper<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(&self.0).hash(state);
    }
}
impl<E> PartialEq for EnumWrapper<E> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(&self.0) == discriminant(&other.0)
    }
}
impl<E> Eq for EnumWrapper<E> {}

impl<E> From<E> for EnumWrapper<E> {
    fn from(internal: E) -> Self {
        Self(internal)
    }
}

pub struct IntoIter<E>(hash_set::IntoIter<EnumWrapper<E>>);
impl<E> Iterator for IntoIter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.into_internal())
    }
}

impl<E> From<hash_set::IntoIter<EnumWrapper<E>>> for IntoIter<E> {
    fn from(set: hash_set::IntoIter<EnumWrapper<E>>) -> Self {
        Self(set)
    }
}

pub struct Iter<'a, E>(hash_set::Iter<'a, EnumWrapper<E>>);
impl<'a, E> Iterator for Iter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.get_ref())
    }
}

impl<'a, E> From<hash_set::Iter<'a, EnumWrapper<E>>> for Iter<'a, E>
where
    E: 'a,
{
    fn from(set: hash_set::Iter<'a, EnumWrapper<E>>) -> Self {
        Self(set)
    }
}

pub trait SerializableEnum {
    type K: Serialize;
    type V: Serialize;
    fn get_key(&self) -> Self::K;
    fn get_value(&self) -> Self::V;
}
