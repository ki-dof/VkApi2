use std::collections::HashMap;

pub struct ParamGrid {
    pub data: HashMap<String, String>,
}

impl Default for ParamGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl ParamGrid {
    pub fn new() -> ParamGrid {
        Self {
            data: HashMap::new(),
        }
    }
}

impl ParamGrid {
    pub fn from_vecs<K>(keys: Vec<String>, values: Vec<K>) -> ParamGrid
    where
        K: ToString,
    {
        let mut data = HashMap::new();

        for (key, value) in keys.into_iter().zip(values.into_iter()) {
            data.insert(key.to_string(), value.to_string());
        }

        ParamGrid { data }
    }
}

impl<T: ToString> From<HashMap<String, T>> for ParamGrid {
    fn from(map: HashMap<String, T>) -> Self {
        let mut data = HashMap::new();

        for (key, value) in map {
            data.insert(key, value.to_string());
        }

        ParamGrid { data }
    }
}

impl<T: ToString> From<Vec<(String, T)>> for ParamGrid {
    fn from(vec: Vec<(String, T)>) -> Self {
        let mut data = HashMap::new();

        for (key, value) in vec {
            data.insert(key, value.to_string());
        }

        ParamGrid { data }
    }
}

impl ParamGrid {
    pub fn insert_if_not_exists<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: ToString,
    {
        self.data.entry(key.into()).or_insert(value.to_string());
    }
}
