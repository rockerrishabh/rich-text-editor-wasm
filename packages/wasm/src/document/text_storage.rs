/// Gap buffer implementation for efficient text storage.
///
/// The gap buffer maintains a contiguous buffer with a "gap" that can be moved
/// to the insertion/deletion point for O(1) operations at the cursor position.
///
/// # Performance Characteristics
///
/// ## Time Complexity
/// - **Insert at gap**: O(1) - no data movement required
/// - **Insert elsewhere**: O(n) where n is distance from gap to insertion point
/// - **Delete at gap**: O(1) - just expand the gap
/// - **Delete elsewhere**: O(n) where n is distance from gap to deletion point
/// - **Get character**: O(1) - direct array access
/// - **Get slice**: O(m) where m is slice length
/// - **Get full text**: O(n) where n is document length
///
/// ## Space Complexity
/// - Memory usage: O(n + g) where n is text length, g is gap size
/// - Gap size: 16-32 characters typically
/// - Overhead: ~4 bytes per character (UTF-32 storage)
///
/// ## Performance Notes
/// - Sequential insertions at the same position are highly optimized
/// - Random insertions cause gap movement (O(n) cost)
/// - Gap automatically expands when needed (amortized O(1))
/// - Best for cursor-based editing (typing, backspace)
/// - Less efficient for random access editing
///
/// # Example
/// ```
/// use rich_text_editor_wasm::document::text_storage::TextStorage;
///
/// let mut storage = TextStorage::new();
/// storage.insert(0, "Hello");
/// assert_eq!(storage.get_text(), "Hello");
/// ```
#[derive(Debug, Clone)]
pub struct TextStorage {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
    #[cfg(feature = "metrics")]
    gap_moves: usize,
    #[cfg(feature = "metrics")]
    gap_expansions: usize,
}

impl TextStorage {
    /// Initial gap size when creating a new buffer
    const INITIAL_GAP_SIZE: usize = 32;

    /// Minimum gap size to maintain
    const MIN_GAP_SIZE: usize = 16;

    /// Creates a new empty TextStorage with an initial gap
    pub fn new() -> Self {
        Self {
            buffer: vec!['\0'; Self::INITIAL_GAP_SIZE],
            gap_start: 0,
            gap_end: Self::INITIAL_GAP_SIZE,
            #[cfg(feature = "metrics")]
            gap_moves: 0,
            #[cfg(feature = "metrics")]
            gap_expansions: 0,
        }
    }

    /// Creates a TextStorage from existing text
    pub fn from_text(text: &str) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        let gap_size = Self::INITIAL_GAP_SIZE;

        let mut buffer = Vec::with_capacity(len + gap_size);
        buffer.extend_from_slice(&chars);
        buffer.resize(len + gap_size, '\0');

        Self {
            buffer,
            gap_start: len,
            gap_end: len + gap_size,
            #[cfg(feature = "metrics")]
            gap_moves: 0,
            #[cfg(feature = "metrics")]
            gap_expansions: 0,
        }
    }

    /// Returns the length of the text (excluding the gap)
    pub fn len(&self) -> usize {
        self.buffer.len() - self.gap_size()
    }

    /// Returns true if the text storage is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the size of the gap
    fn gap_size(&self) -> usize {
        self.gap_end - self.gap_start
    }

    /// Moves the gap to the specified position
    fn move_gap(&mut self, pos: usize) {
        if pos == self.gap_start {
            return;
        }

        #[cfg(feature = "metrics")]
        {
            self.gap_moves += 1;
        }

        if pos < self.gap_start {
            // Move gap left
            let distance = self.gap_start - pos;
            let new_gap_end = self.gap_end - distance;

            // Use copy_within for better performance
            self.buffer.copy_within(pos..self.gap_start, new_gap_end);

            self.gap_start = pos;
            self.gap_end = new_gap_end;
        } else {
            // Move gap right
            let distance = pos - self.gap_start;

            // Use copy_within for better performance
            self.buffer
                .copy_within(self.gap_end..self.gap_end + distance, self.gap_start);

            self.gap_start = pos;
            self.gap_end = self.gap_end + distance;
        }
    }

    /// Expands the gap to accommodate more insertions
    fn expand_gap(&mut self, min_size: usize) {
        let current_gap = self.gap_size();
        if current_gap >= min_size {
            return;
        }

        #[cfg(feature = "metrics")]
        {
            self.gap_expansions += 1;
        }

        let additional_size = (min_size - current_gap).max(Self::MIN_GAP_SIZE);
        let new_size = self.buffer.len() + additional_size;

        // Create new buffer with expanded gap
        let mut new_buffer = Vec::with_capacity(new_size);

        // Copy text before gap
        new_buffer.extend_from_slice(&self.buffer[..self.gap_start]);

        // Add gap space
        new_buffer.resize(self.gap_start + current_gap + additional_size, '\0');

        // Copy text after gap
        new_buffer.extend_from_slice(&self.buffer[self.gap_end..]);

        self.buffer = new_buffer;
        self.gap_end = self.gap_start + current_gap + additional_size;
    }

    /// Inserts text at the specified position
    pub fn insert(&mut self, pos: usize, text: &str) {
        if pos > self.len() {
            panic!(
                "Insert position {} is out of bounds (len: {})",
                pos,
                self.len()
            );
        }

        let chars: Vec<char> = text.chars().collect();
        let insert_len = chars.len();

        if insert_len == 0 {
            return;
        }

        // Move gap to insertion point
        self.move_gap(pos);

        // Expand gap if needed
        if self.gap_size() < insert_len {
            self.expand_gap(insert_len);
        }

        // Insert characters into the gap
        for (i, ch) in chars.iter().enumerate() {
            self.buffer[self.gap_start + i] = *ch;
        }

        self.gap_start += insert_len;
    }

    /// Deletes text in the specified range [start, end)
    pub fn delete(&mut self, start: usize, end: usize) {
        if start > end {
            panic!("Invalid range: start {} > end {}", start, end);
        }
        if end > self.len() {
            panic!(
                "Delete end position {} is out of bounds (len: {})",
                end,
                self.len()
            );
        }

        if start == end {
            return;
        }

        // Move gap to start of deletion
        self.move_gap(start);

        // Expand gap to include deleted range
        self.gap_end += end - start;
    }

    /// Returns the character at the specified position
    pub fn get_char(&self, pos: usize) -> Option<char> {
        if pos >= self.len() {
            return None;
        }

        let actual_pos = if pos < self.gap_start {
            pos
        } else {
            pos + self.gap_size()
        };

        Some(self.buffer[actual_pos])
    }

    /// Returns a slice of text from the specified range [start, end)
    pub fn get_slice(&self, start: usize, end: usize) -> String {
        if start > end {
            panic!("Invalid range: start {} > end {}", start, end);
        }
        if end > self.len() {
            panic!(
                "Slice end position {} is out of bounds (len: {})",
                end,
                self.len()
            );
        }

        let mut result = String::with_capacity(end - start);

        for pos in start..end {
            if let Some(ch) = self.get_char(pos) {
                result.push(ch);
            }
        }

        result
    }

    /// Returns the entire text content as a String
    pub fn get_text(&self) -> String {
        self.get_slice(0, self.len())
    }

    /// Returns performance metrics for the gap buffer
    #[cfg(feature = "metrics")]
    pub fn get_metrics(&self) -> (usize, usize, usize, usize) {
        (
            self.gap_moves,
            self.gap_expansions,
            self.gap_size(),
            self.buffer.capacity(),
        )
    }

    /// Returns the current gap position (for debugging/optimization)
    pub fn gap_position(&self) -> usize {
        self.gap_start
    }
}

impl Default for TextStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_storage() {
        let storage = TextStorage::new();
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_from_text() {
        let storage = TextStorage::from_text("Hello");
        assert_eq!(storage.len(), 5);
        assert_eq!(storage.get_text(), "Hello");
    }

    #[test]
    fn test_insert_at_start() {
        let mut storage = TextStorage::new();
        storage.insert(0, "Hello");
        assert_eq!(storage.len(), 5);
        assert_eq!(storage.get_text(), "Hello");
    }

    #[test]
    fn test_insert_at_end() {
        let mut storage = TextStorage::from_text("Hello");
        storage.insert(5, " World");
        assert_eq!(storage.len(), 11);
        assert_eq!(storage.get_text(), "Hello World");
    }

    #[test]
    fn test_insert_in_middle() {
        let mut storage = TextStorage::from_text("Hello World");
        storage.insert(5, " Beautiful");
        assert_eq!(storage.get_text(), "Hello Beautiful World");
    }

    #[test]
    fn test_multiple_inserts() {
        let mut storage = TextStorage::new();
        storage.insert(0, "a");
        storage.insert(1, "b");
        storage.insert(2, "c");
        assert_eq!(storage.get_text(), "abc");
    }

    #[test]
    fn test_delete_range() {
        let mut storage = TextStorage::from_text("Hello World");
        storage.delete(5, 11);
        assert_eq!(storage.get_text(), "Hello");
    }

    #[test]
    fn test_delete_from_start() {
        let mut storage = TextStorage::from_text("Hello World");
        storage.delete(0, 6);
        assert_eq!(storage.get_text(), "World");
    }

    #[test]
    fn test_delete_in_middle() {
        let mut storage = TextStorage::from_text("Hello World");
        storage.delete(5, 6);
        assert_eq!(storage.get_text(), "HelloWorld");
    }

    #[test]
    fn test_get_char() {
        let storage = TextStorage::from_text("Hello");
        assert_eq!(storage.get_char(0), Some('H'));
        assert_eq!(storage.get_char(4), Some('o'));
        assert_eq!(storage.get_char(5), None);
    }

    #[test]
    fn test_get_slice() {
        let storage = TextStorage::from_text("Hello World");
        assert_eq!(storage.get_slice(0, 5), "Hello");
        assert_eq!(storage.get_slice(6, 11), "World");
        assert_eq!(storage.get_slice(0, 11), "Hello World");
    }

    #[test]
    fn test_insert_and_delete_sequence() {
        let mut storage = TextStorage::new();
        storage.insert(0, "Hello");
        storage.insert(5, " World");
        storage.delete(5, 6);
        assert_eq!(storage.get_text(), "HelloWorld");
        storage.insert(5, " ");
        assert_eq!(storage.get_text(), "Hello World");
    }

    #[test]
    fn test_gap_movement() {
        let mut storage = TextStorage::new();
        storage.insert(0, "abc");
        storage.insert(0, "123");
        assert_eq!(storage.get_text(), "123abc");
    }

    #[test]
    fn test_large_text() {
        let mut storage = TextStorage::new();
        let text = "a".repeat(1000);
        storage.insert(0, &text);
        assert_eq!(storage.len(), 1000);
        assert_eq!(storage.get_text(), text);
    }

    #[test]
    fn test_unicode_characters() {
        let mut storage = TextStorage::new();
        storage.insert(0, "Hello 世界 🌍");
        assert_eq!(storage.get_text(), "Hello 世界 🌍");
        assert_eq!(storage.len(), 10); // 6 ASCII + 2 Chinese + 2 emoji chars
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_insert_out_of_bounds() {
        let mut storage = TextStorage::from_text("Hello");
        storage.insert(10, "World");
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_delete_out_of_bounds() {
        let mut storage = TextStorage::from_text("Hello");
        storage.delete(0, 10);
    }

    #[test]
    fn test_empty_operations() {
        let mut storage = TextStorage::from_text("Hello");
        storage.insert(2, "");
        assert_eq!(storage.get_text(), "Hello");
        storage.delete(2, 2);
        assert_eq!(storage.get_text(), "Hello");
    }
}
