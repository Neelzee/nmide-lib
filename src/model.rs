use serde_json::{Map, Value};

use crate::Location;

pub const ACCESS_TOKEN: &str = ".";

#[repr(C)]
pub struct Model(Map<String, Value>);

impl Model {
    pub fn new(val: Map<String, Value>) -> Self {
        Self(val)
    }

    pub fn empty() -> Self {
        Self(Map::new())
    }

    pub fn add_location(self, location: Location) -> Self {
        let key = location.to_string();

        self.push_to_array(key.clone(), Value::String(key))
    }

    pub fn push_to_array(self, key: impl Into<String>, val: Value) -> Self {
        let str_key: String = key.into();
        match self.0.get(&str_key) {
            Some(Value::Array(arr)) => {
                let mut arrs = arr.clone();
                let mut map = self.0;
                arrs.push(val);
                map.insert(str_key, Value::Array(arrs.to_vec()));

                Model::new(map)
            }
            _ => self,
        }
    }

    pub fn insert(self, key: impl Into<String>, val: Value) -> Self {
        let mut value = self.0;
        let str_key: String = key.into();
        let mut it = str_key.split(ACCESS_TOKEN).peekable();
        while let Some(k) = it.next() {
            match value.get(k) {
                Some(Value::Object(m)) => value = m.clone(),
                None => {
                    value.insert(k.to_string(), Value::Object(Map::new()));
                }
                _ => (),
            }

            // Is on the last iteration
            if it.peek().is_none() {
                // Copies the element, but this only happends once
                value.insert(k.to_string(), val.clone());
            }
        }

        Self(value)
    }

    pub fn remove(self, key: impl Into<String>) -> Self {
        let mut value = self.0;
        let str_key: String = key.into();
        let mut it = str_key.split(ACCESS_TOKEN).peekable();
        while let Some(k) = it.next() {
            // Is on the last iteration
            if it.peek().is_none() {
                value.remove(k);
            }
            match value.get(k) {
                Some(Value::Object(m)) => value = m.clone(),
                _ => continue,
            }
        }

        Self(value)
    }
}
