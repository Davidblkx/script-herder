use std::collections::{HashMap, HashSet};

/// Abstracts a collection of providers with priorities.
/// 
/// # Examples
/// let mut src = PriorityProvider::new();
/// src.add("potato");
/// src.add("tomato");
/// src.add("carrot");
/// src.add_top("onion");
/// 
/// let mut iter = src.iter();
/// assert_eq!(iter.next(), Some(&"onion"));
/// assert_eq!(iter.next(), Some(&"potato"));
/// assert_eq!(iter.next(), Some(&"tomato"));
/// assert_eq!(iter.next(), Some(&"carrot"));
pub struct PriorityProvider<T> {
    providers: HashMap<i64, T>,
    priority: HashSet<i64>,
    lowest_priority: i64,
    upper_priority: i64,
}

/// Iterator for PriorityProvider.
pub struct PriorityProviderIterator<'a, T> {
    providers: &'a HashMap<i64, T>,
    sorted_keys: Vec<i64>,
    index: usize,
}

impl<'a, T> PriorityProviderIterator<'a, T> {
    pub fn new(src: &'a PriorityProvider<T>) -> PriorityProviderIterator<'a, T> {
        let mut sorted_keys: Vec<i64> = src.priority.iter().map(|&p| p).collect();
        sorted_keys.sort();

        PriorityProviderIterator {
            providers: &src.providers,
            sorted_keys,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for PriorityProviderIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.sorted_keys.len() {
            return None;
        }

        let key = self.sorted_keys[self.index];
        self.index += 1;

        match self.providers.get(&key) {
            Some(provider) => Some(provider),
            None => None,
        }
    }
}

impl<T> PriorityProvider<T> {
    pub fn new() -> PriorityProvider<T> {
        PriorityProvider {
            providers: HashMap::new(),
            priority: HashSet::new(),
            lowest_priority: 0,
            upper_priority: 0,
        }
    }

    pub fn get_lowest_priority(&self) -> i64 {
        self.lowest_priority
    }

    pub fn get_upper_priority(&self) -> i64 {
        self.upper_priority
    }

    /// Returns an iterator for the providers.
    pub fn iter(&self) -> PriorityProviderIterator<T> {
        PriorityProviderIterator::new(self)
    }

    /// Returns the provider at the given position.
    pub fn get_at(&self, pos: i64) -> Option<&T> {
        self.providers.get(&pos)
    }

    pub fn get_at_mut(&mut self, pos: i64) -> Option<&mut T> {
        self.providers.get_mut(&pos)
    }

    /// Sets the provider at the given position.
    pub fn set_at(&mut self, pos: i64, provider: T) {
        self.providers.insert(pos, provider);
        self.priority.insert(pos);

        if pos > self.upper_priority {
            self.upper_priority = pos;
        }

        if pos < self.lowest_priority {
            self.lowest_priority = pos;
        }
    }
    
    /// Adds a provider to the end of the list.
    pub fn add(&mut self, provider: T) {
        let pos = self.upper_priority + 1;
        self.set_at(pos, provider);
    }

    /// Adds a provider to the top of the list.
    pub fn add_top(&mut self, provider: T) {
        let pos = self.lowest_priority - 1;
        self.set_at(pos, provider);
    }

    /// Returns the first provider that matches the given filter.
    pub fn first(&self, filter: &dyn Fn(&T) -> bool) -> Option<&T> {
        for provider in self.iter() {
            if filter(provider) {
                return Some(provider);
            }
        }

        None
    }

    pub fn first_mut(&mut self, filter: &dyn Fn(&T) -> bool) -> Option<&mut T> {
        let mut sorted_keys: Vec<i64> = self.priority.iter().map(|&p| p).collect();
        sorted_keys.sort();
        for key in sorted_keys {
            if filter(self.providers.get(&key).unwrap()) {
                return self.providers.get_mut(&key);
            }
        }

        None
    }

    pub fn map_first<U>(&self, filter: &dyn Fn(&T) -> Option<U>) -> Option<U> {
        for provider in self.iter() {
            match filter(provider) {
                Some(value) => return Some(value),
                None => continue,
            }
        }

        None
    }

    pub fn map_first_mut<U>(&mut self, filter: &dyn Fn(&T) -> Option<U>) -> Option<U> {
        let mut sorted_keys: Vec<i64> = self.priority.iter().map(|&p| p).collect();
        sorted_keys.sort();
        for key in sorted_keys {
            match filter(self.providers.get(&key).unwrap()) {
                Some(value) => return Some(value),
                None => continue,
            }
        }

        None
    }

    pub fn map_mut<U>(&mut self, filter: &dyn Fn(&mut T) -> Option<U>) -> Vec<U> {
        let mut result = Vec::new();
        let mut sorted_keys: Vec<i64> = self.priority.iter().map(|&p| p).collect();
        sorted_keys.sort();
        for key in sorted_keys {
            match filter(self.providers.get_mut(&key).unwrap()) {
                Some(value) => result.push(value),
                None => continue,
            }
        }

        result
    }

    pub fn each_mut(&mut self, filter: &dyn Fn(&mut T)) {
        let mut sorted_keys: Vec<i64> = self.priority.iter().map(|&p| p).collect();
        sorted_keys.sort();
        for key in sorted_keys {
            filter(self.providers.get_mut(&key).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn iter_returns_sorted_iterator() {
        let mut src = PriorityProvider::new();
        src.set_at(1, "1");
        src.set_at(3, "2");
        src.set_at(2, "3");

        let mut iter = src.iter();
        assert_eq!(iter.next(), Some(&"1"));
        assert_eq!(iter.next(), Some(&"3"));
        assert_eq!(iter.next(), Some(&"2"));
    }

    #[test]
    pub fn set_at_sets_lowest_priority() {
        let mut src = PriorityProvider::new();
        src.set_at(1, "1");
        src.set_at(3, "2");
        src.set_at(2, "3");

        assert_eq!(src.get_upper_priority(), 3);
    }

    #[test]
    pub fn set_at_overwrites_provider() {
        let mut src = PriorityProvider::new();
        src.set_at(1, "1");
        src.set_at(1, "2");

        assert_eq!(src.get_upper_priority(), 1);
        assert_eq!(src.get_at(1), Some(&"2"));
    }

    #[test]
    pub fn add_adds_provider() {
        let mut src = PriorityProvider::new();
        src.add("1");
        src.add("2");

        assert_eq!(src.get_upper_priority(), 2);
        assert_eq!(src.get_at(1), Some(&"1"));
        assert_eq!(src.get_at(2), Some(&"2"));
    }

    #[test]
    pub fn add_top_adds_provider() {
        let mut src = PriorityProvider::new();
        src.add("3");
        src.add_top("1");
        src.add_top("2");

        assert_eq!(src.get_lowest_priority(), -2);
        assert_eq!(src.get_at(-1), Some(&"1"));
        assert_eq!(src.get_at(-2), Some(&"2"));
    }

    #[test]
    pub fn first_returns_first_provider_that_matches_filter() {
        let mut src = PriorityProvider::new();
        src.add("1");
        src.add("2");
        src.add("3");

        let provider = src.first(&|p| *p == "2");
        assert_eq!(provider, Some(&"2"));
    }

    #[test]
    pub fn map_first_returns_first_result_with_value() {
        let mut src = PriorityProvider::new();
        src.add(1);
        src.add(2);
        src.add(3);

        let provider = src.map_first(&|p| {
            if *p == 2 {
                Some(p * 3)
            } else {
                None
            }
        });
        assert_eq!(provider, Some(6));
    }
}