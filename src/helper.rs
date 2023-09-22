use std::fmt::Display;

pub trait IsDue {
    /// Returns whether or not a date is past it
    fn due(&self) -> bool;
}

#[derive(Clone, Debug, Default)]
pub struct Map<K: PartialEq, V: PartialEq> {
    pub data: Vec<(K, V)>,
}

impl<K: PartialEq + Display, V: PartialEq + Display> Display for Map<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return Ok(());
        }

        let end = self.data.len() - 1;
        for (i, (k, v)) in self.data.iter().enumerate() {
            write!(f, "{k}:{v}")?;

            if i != end {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

impl<K: PartialEq, V: PartialEq> Map<K, V> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn insert(&mut self, key: K, val: V) {
        for pair in self.data.iter_mut() {
            if pair.0 == key {
                pair.1 = val;
                return;
            }
        }

        self.data.push((key, val));
    }

    pub fn remove(&mut self, key: &K) {
        let mut remove_index = None;
        for (i, (k, _)) in self.data.iter().enumerate() {
            if k == key {
                remove_index = Some(i);
            }
        }

        if let Some(i) = remove_index {
            self.data.remove(i);
        }
    }
}
