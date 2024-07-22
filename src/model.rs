use serde_json::{Map, Value};

pub const ACCESS_TOKEN: &str = ".";

pub struct Model(Map<String, Value>);

impl Model {
    pub fn new(val: Map<String, Value>) -> Self {
        Self(val)
    }

    pub fn empty() -> Self {
        Self(Map::new())
    }

    pub fn insert(self, key: String, val: Value) -> Self {
        let mut value = self.0;
        let mut it = key.split(ACCESS_TOKEN).peekable();
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

    pub fn remove(self, key: String) -> Self {
        let mut value = self.0;
        let mut it = key.split(ACCESS_TOKEN).peekable();
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
