use std::{collections::HashMap, hash::Hash, time::{Instant, Duration}};

/// Wrapper for a [HashMap] where all values only live 12 hours. After that time
/// they are removed from the map.
/// 
/// When a new entry is inserted, the map is checked if the netries are still
/// valid.
/// 
#[derive(Clone, Debug)]
pub struct TimedCache<K, V>(HashMap<K, (V, Instant)>);

impl<K, V> TimedCache<K, V>
    where K: Copy + Eq + Hash {

    /// Lifespan of an entry in the map. The lifespan is half a day.
    /// 
    /// Seconds * Minutes * Day
    /// 
    const LIFESPAN: Duration = Duration::new(60 * 60 * 12, 0);

    /// Creates an empty [TimedCache].
    /// 
    /// See the [HashMap] for more information on the internal work.
    /// 
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Inserts the given value with the given key into the underlying map.
    /// 
    pub fn insert(
        &mut self,
        key:   K,
        value: V,
    ) {
        self.remove_expired();

        let deadline = Instant::now() + Self::LIFESPAN;
        self.0.insert(key, (value, deadline));
    }

    /// Gets an entry from the underlying map.
    /// 
    pub fn get(
        &self,
        key: K
    ) -> Option<&V> {
        let entry = self.0.get(&key)?;
        if entry.1 < Instant::now() {
            None
        } else {
            Some(&entry.0)
        }
    }

    /// Checks all entries and removes them if they are expired.
    /// 
    fn remove_expired(
        &mut self,
    ) {
        let now = Instant::now();

        let mut expired = Vec::new();
        for (k, (_, deadline)) in self.0.iter() {
            if *deadline < now {
                expired.push(*k);
            }
        }

        for expire in expired {
            self.0.remove(&expire);
        }
    }
}
