#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    String(String),
    Object(Map),
    Array(Vec<Value>),
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    map: Vec<MapEntry>,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEntry {
    key: String,
    val: Value,
}

impl From<(String, Value)> for MapEntry {
    fn from(value: (String, Value)) -> Self {
        Self {
            key: value.0,
            val: value.1,
        }
    }
}

impl From<MapEntry> for (String, Value) {
    fn from(value: MapEntry) -> Self {
        (value.key, value.val)
    }
}

impl Map {
    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn lookup(key: &String, map: &[MapEntry]) -> Option<Value> {
        match map {
            [] => None,
            [x, ..] if x.key == *key => Some(x.val.clone()),
            [_, xs @ ..] => Self::lookup(key, xs),
        }
    }

    fn lookup_index(key: &String, map: &[MapEntry]) -> Option<usize> {
        fn lookup_index_helper(key: &String, xs: &[MapEntry], i: usize) -> Option<usize> {
            match xs {
                [] => None,
                [x, ..] if x.key == *key => Some(i),
                [_, xs @ ..] => lookup_index_helper(key, xs, i + 1),
            }
        }

        lookup_index_helper(key, map, 0)
    }

    pub fn get(&self, key: &String) -> Option<Value> {
        Self::lookup(key, &self.map[..])
    }

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        match Self::lookup_index(&key, &self.map[..]) {
            Some(i) => {
                let old_val = Some(self.map.swap_remove(i).val);
                self.map.push((key, value).into());
                old_val
            }
            None => {
                self.map.push((key, value).into());
                None
            }
        }
    }

    pub fn remove(&mut self, key: &String) -> Option<Value> {
        Self::lookup_index(&key, &self.map[..]).and_then(|i| Some(self.map.swap_remove(i).val))
    }
}

pub struct MapIterator {
    current: Option<(String, Value)>,
    remainder: Vec<(String, Value)>,
}

impl MapIterator {
    pub fn advance(&mut self) -> Option<(String, Value)> {
        let mut remainder = self.remainder.clone();
        let prev = self.current.clone();
        self.current = remainder.pop();
        self.remainder = remainder;

        return prev;
    }
}

impl IntoIterator for Map {
    type Item = (String, Value);

    type IntoIter = MapIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut remainder = self
            .map
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<(_, _)>>();
        let current: Option<(String, Value)> = remainder.pop();

        MapIterator { current, remainder }
    }
}

impl Iterator for MapIterator {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}
