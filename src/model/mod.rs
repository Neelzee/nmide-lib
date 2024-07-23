use crate::model::map::{Map, Value};
use crate::Location;

pub mod map;

pub const ACCESS_TOKEN: &str = ".";

#[repr(C)]
pub struct Model(Map);

impl Model {
    pub fn new(val: Map) -> Self {
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

    /// Inserts a `Value` into the specified `Key`.
    ///
    /// The `Key` can be a JSON "path"
    ///
    /// # Example
    /// ```rust
    /// let mut model: Model = Model::empty();
    ///
    /// model = model.insert("foo.bar.foobar", Value::Bool(true));
    ///
    /// assert_eq!(model.to_json(), "{ \"foo\": { \"bar\": { \"foobar\": true } } }");
    /// ```
    pub fn insert(self, key: impl Into<String>, val: Value) -> Self {
        let mut value = self.0;
        let str_key: String = key.into();
        let mut it = str_key.split(ACCESS_TOKEN).peekable();
        while let Some(k) = it.next() {
            match value.get(&(k.to_string())) {
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
                value.remove(&(k.to_string()));
            }
            match value.get(&(k.to_string())) {
                Some(Value::Object(m)) => value = m.clone(),
                _ => continue,
            }
        }

        Self(value)
    }
}
