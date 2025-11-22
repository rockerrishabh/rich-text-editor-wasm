use std::collections::HashMap;
use std::sync::Arc;

/// A string interner that deduplicates strings to save memory
/// Uses Arc for efficient cloning and sharing of interned strings
#[derive(Debug, Clone)]
pub struct StringInterner {
    strings: HashMap<String, Arc<str>>,
}

impl StringInterner {
    /// Creates a new empty string interner
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
        }
    }

    /// Interns a string, returning an Arc to the shared string
    /// If the string already exists, returns the existing Arc
    pub fn intern(&mut self, s: &str) -> Arc<str> {
        if let Some(interned) = self.strings.get(s) {
            Arc::clone(interned)
        } else {
            let arc: Arc<str> = Arc::from(s);
            self.strings.insert(s.to_string(), Arc::clone(&arc));
            arc
        }
    }

    /// Returns the number of unique strings in the interner
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns true if the interner is empty
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Clears all interned strings
    pub fn clear(&mut self) {
        self.strings.clear();
    }

    /// Returns true if the interner contains the given string
    pub fn contains(&self, s: &str) -> bool {
        self.strings.contains_key(s)
    }

    /// Estimates the memory usage of the interner in bytes
    ///
    /// This includes:
    /// - HashMap overhead (~48 bytes + capacity * 24 bytes)
    /// - String keys (actual string data)
    /// - Arc<str> values (pointer + string data, shared)
    ///
    /// # Returns
    /// Estimated memory usage in bytes
    ///
    /// # Note
    /// This is an approximation. Actual memory usage may vary due to
    /// allocator overhead and internal HashMap implementation details.
    pub fn estimated_memory(&self) -> usize {
        // HashMap base overhead
        let hashmap_overhead = 48;

        // HashMap capacity overhead (each entry ~24 bytes)
        let capacity_overhead = self.strings.capacity() * 24;

        // String data (keys + values)
        let string_data: usize = self
            .strings
            .keys()
            .map(|s| {
                // Key: String overhead (24 bytes) + actual string data
                let key_size = 24 + s.len();
                // Value: Arc overhead (16 bytes) + string data (shared with key)
                let value_size = 16 + s.len();
                key_size + value_size
            })
            .sum();

        hashmap_overhead + capacity_overhead + string_data
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interner_creation() {
        let interner = StringInterner::new();
        assert_eq!(interner.len(), 0);
        assert!(interner.is_empty());
    }

    #[test]
    fn test_intern_string() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello");
        assert_eq!(interner.len(), 1);
        assert_eq!(&*s1, "hello");
    }

    #[test]
    fn test_intern_duplicate() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello");
        let s2 = interner.intern("hello");

        // Should only have one entry
        assert_eq!(interner.len(), 1);

        // Both should point to the same Arc
        assert!(Arc::ptr_eq(&s1, &s2));
    }

    #[test]
    fn test_intern_multiple_strings() {
        let mut interner = StringInterner::new();
        interner.intern("hello");
        interner.intern("world");
        interner.intern("rust");

        assert_eq!(interner.len(), 3);
        assert!(interner.contains("hello"));
        assert!(interner.contains("world"));
        assert!(interner.contains("rust"));
        assert!(!interner.contains("other"));
    }

    #[test]
    fn test_intern_colors() {
        let mut interner = StringInterner::new();

        // Simulate repeated color values
        let red1 = interner.intern("#FF0000");
        let red2 = interner.intern("#FF0000");
        let blue = interner.intern("#0000FF");

        assert_eq!(interner.len(), 2);
        assert!(Arc::ptr_eq(&red1, &red2));
        assert!(!Arc::ptr_eq(&red1, &blue));
    }

    #[test]
    fn test_intern_urls() {
        let mut interner = StringInterner::new();

        // Simulate repeated URLs
        let url1 = interner.intern("https://example.com");
        let url2 = interner.intern("https://example.com");
        let url3 = interner.intern("https://other.com");

        assert_eq!(interner.len(), 2);
        assert!(Arc::ptr_eq(&url1, &url2));
        assert!(!Arc::ptr_eq(&url1, &url3));
    }

    #[test]
    fn test_clear() {
        let mut interner = StringInterner::new();
        interner.intern("hello");
        interner.intern("world");

        assert_eq!(interner.len(), 2);

        interner.clear();
        assert_eq!(interner.len(), 0);
        assert!(interner.is_empty());
    }

    #[test]
    fn test_memory_efficiency() {
        let mut interner = StringInterner::new();

        // Intern the same string many times
        let mut arcs = Vec::new();
        for _ in 0..1000 {
            arcs.push(interner.intern("#FF0000"));
        }

        // Should only have one unique string
        assert_eq!(interner.len(), 1);

        // All Arcs should point to the same memory
        for i in 1..arcs.len() {
            assert!(Arc::ptr_eq(&arcs[0], &arcs[i]));
        }
    }
}
