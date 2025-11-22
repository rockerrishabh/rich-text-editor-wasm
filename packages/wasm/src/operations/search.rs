use crate::{
    document::{Document, Range},
    operations::Command,
};
use regex::Regex;

/// Query parameters for searching text in a document
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// The pattern to search for (literal string or regex pattern)
    pub pattern: String,
    /// Whether the search should be case-sensitive
    pub case_sensitive: bool,
    /// Whether to interpret the pattern as a regular expression
    pub use_regex: bool,
}

impl SearchQuery {
    /// Creates a new SearchQuery with the specified pattern
    pub fn new(pattern: String) -> Self {
        Self {
            pattern,
            case_sensitive: false,
            use_regex: false,
        }
    }

    /// Sets whether the search should be case-sensitive
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    /// Sets whether to use regex pattern matching
    pub fn use_regex(mut self, use_regex: bool) -> Self {
        self.use_regex = use_regex;
        self
    }
}

/// Result of a search operation containing all matches
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    /// Vector of ranges representing all matches found
    pub matches: Vec<Range>,
}

impl SearchResult {
    /// Creates a new empty SearchResult
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
        }
    }

    /// Creates a SearchResult with the specified matches
    pub fn with_matches(matches: Vec<Range>) -> Self {
        Self { matches }
    }

    /// Returns the number of matches found
    pub fn count(&self) -> usize {
        self.matches.len()
    }

    /// Returns true if no matches were found
    pub fn is_empty(&self) -> bool {
        self.matches.is_empty()
    }
}

impl Default for SearchResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Command that performs find and replace operation
#[derive(Debug, Clone)]
pub struct FindAndReplaceCommand {
    query: SearchQuery,
    replacement: String,
    /// Stores the ranges and original text for undo
    replaced_ranges: Option<Vec<(Range, String)>>,
}

impl FindAndReplaceCommand {
    /// Creates a new FindAndReplaceCommand
    pub fn new(query: SearchQuery, replacement: String) -> Self {
        Self {
            query,
            replacement,
            replaced_ranges: None,
        }
    }
}

impl crate::operations::Command for FindAndReplaceCommand {
    fn execute(&mut self, doc: &mut Document) -> crate::operations::CommandResult<()> {
        // Find all matches
        let result = doc.find(&self.query).map_err(|e| {
            crate::operations::CommandError::execution_failed("FindAndReplaceCommand", e)
        })?;

        if result.is_empty() {
            self.replaced_ranges = Some(Vec::new());
            return Ok(());
        }

        // Store original text for each match (for undo)
        let mut replaced = Vec::new();
        for range in &result.matches {
            let original_text = doc.get_text_in_range(*range);
            replaced.push((*range, original_text));
        }

        // Replace matches in reverse order to maintain correct positions
        let mut matches = result.matches;
        matches.sort_by(|a, b| b.start_offset().cmp(&a.start_offset()));

        for range in matches {
            doc.replace_range_direct(range, &self.replacement);
        }

        self.replaced_ranges = Some(replaced);
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> crate::operations::CommandResult<()> {
        if let Some(ref replaced) = self.replaced_ranges {
            // Restore original text in reverse order
            let mut ranges: Vec<_> = replaced.iter().collect();
            ranges.sort_by(|a, b| b.0.start_offset().cmp(&a.0.start_offset()));

            for (original_range, original_text) in ranges {
                // Calculate the current range (after replacement)
                let replacement_len = self.replacement.chars().count();
                let current_range = Range::from_offsets(
                    original_range.start_offset(),
                    original_range.start_offset() + replacement_len,
                );
                doc.replace_range_direct(current_range, original_text);
            }

            Ok(())
        } else {
            Err(crate::operations::CommandError::command_not_executed(
                "FindAndReplaceCommand",
            ))
        }
    }

    fn description(&self) -> String {
        format!(
            "Find and replace '{}' with '{}'",
            self.query.pattern, self.replacement
        )
    }
}

impl Document {
    /// Finds all occurrences of the search query in the document
    ///
    /// # Arguments
    /// * `query` - The search query containing pattern and options
    ///
    /// # Returns
    /// A SearchResult containing all matching ranges
    ///
    /// # Errors
    /// Returns an error if the regex pattern is invalid
    pub fn find(&self, query: &SearchQuery) -> Result<SearchResult, String> {
        let content = self.get_content();
        let mut matches = Vec::new();

        // Handle empty pattern - return no matches to avoid infinite loops
        if query.pattern.is_empty() {
            return Ok(SearchResult::new());
        }

        if query.use_regex {
            // Use regex pattern matching
            let pattern = if query.case_sensitive {
                &query.pattern
            } else {
                // Prepend case-insensitive flag
                &format!("(?i){}", query.pattern)
            };

            let regex = Regex::new(pattern).map_err(|e| format!("Invalid regex pattern: {}", e))?;

            for mat in regex.find_iter(&content) {
                let start_offset = content[..mat.start()].chars().count();
                let end_offset = start_offset + mat.as_str().chars().count();
                matches.push(Range::from_offsets(start_offset, end_offset));
            }
        } else {
            // Use literal string matching
            let search_text = if query.case_sensitive {
                content.clone()
            } else {
                content.to_lowercase()
            };

            let pattern = if query.case_sensitive {
                query.pattern.clone()
            } else {
                query.pattern.to_lowercase()
            };

            let pattern_len = pattern.chars().count();
            let mut start_pos = 0;

            while let Some(relative_pos) = search_text[start_pos..].find(&pattern) {
                // Convert byte position to character position
                let byte_pos = start_pos + relative_pos;
                let char_pos = content[..byte_pos].chars().count();
                let end_pos = char_pos + pattern_len;

                matches.push(Range::from_offsets(char_pos, end_pos));

                // Move past this match
                start_pos = byte_pos + pattern.len();
            }
        }

        Ok(SearchResult::with_matches(matches))
    }

    /// Finds and replaces all occurrences of the search query with the replacement text
    ///
    /// # Arguments
    /// * `query` - The search query containing pattern and options
    /// * `replacement` - The text to replace matches with
    ///
    /// # Returns
    /// The number of replacements made
    ///
    /// # Errors
    /// Returns an error if the regex pattern is invalid or replacement fails
    pub fn find_and_replace(
        &mut self,
        query: &SearchQuery,
        replacement: &str,
    ) -> crate::operations::CommandResult<usize> {
        let mut cmd = Box::new(FindAndReplaceCommand::new(
            query.clone(),
            replacement.to_string(),
        ));
        cmd.execute(self)?;

        let count = cmd.replaced_ranges.as_ref().map(|r| r.len()).unwrap_or(0);

        self.history.push_command(cmd);
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new("test".to_string())
            .case_sensitive(true)
            .use_regex(false);

        assert_eq!(query.pattern, "test");
        assert!(query.case_sensitive);
        assert!(!query.use_regex);
    }

    #[test]
    fn test_search_result_empty() {
        let result = SearchResult::new();
        assert!(result.is_empty());
        assert_eq!(result.count(), 0);
    }

    #[test]
    fn test_search_result_with_matches() {
        let matches = vec![Range::from_offsets(0, 5), Range::from_offsets(10, 15)];
        let result = SearchResult::with_matches(matches.clone());

        assert!(!result.is_empty());
        assert_eq!(result.count(), 2);
        assert_eq!(result.matches, matches);
    }

    #[test]
    fn test_find_literal_case_insensitive() {
        let doc = Document::from_text("Hello World, hello world");
        let query = SearchQuery::new("hello".to_string());

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 2);
        assert_eq!(result.matches[0], Range::from_offsets(0, 5));
        assert_eq!(result.matches[1], Range::from_offsets(13, 18));
    }

    #[test]
    fn test_find_literal_case_sensitive() {
        let doc = Document::from_text("Hello World, hello world");
        let query = SearchQuery::new("hello".to_string()).case_sensitive(true);

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 1);
        assert_eq!(result.matches[0], Range::from_offsets(13, 18));
    }

    #[test]
    fn test_find_no_matches() {
        let doc = Document::from_text("Hello World");
        let query = SearchQuery::new("xyz".to_string());

        let result = doc.find(&query).unwrap();
        assert!(result.is_empty());
        assert_eq!(result.count(), 0);
    }

    #[test]
    fn test_find_overlapping_pattern() {
        let doc = Document::from_text("aaa");
        let query = SearchQuery::new("aa".to_string());

        let result = doc.find(&query).unwrap();
        // Should find "aa" at positions 0-2 and 1-3
        // But our implementation finds non-overlapping matches
        assert_eq!(result.count(), 1);
        assert_eq!(result.matches[0], Range::from_offsets(0, 2));
    }

    #[test]
    fn test_find_regex_simple() {
        let doc = Document::from_text("Hello World 123");
        let query = SearchQuery::new(r"\d+".to_string()).use_regex(true);

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 1);
        assert_eq!(result.matches[0], Range::from_offsets(12, 15));
    }

    #[test]
    fn test_find_regex_case_insensitive() {
        let doc = Document::from_text("Hello World, hello world");
        let query = SearchQuery::new("hello".to_string()).use_regex(true);

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 2);
    }

    #[test]
    fn test_find_regex_case_sensitive() {
        let doc = Document::from_text("Hello World, hello world");
        let query = SearchQuery::new("hello".to_string())
            .use_regex(true)
            .case_sensitive(true);

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 1);
        assert_eq!(result.matches[0], Range::from_offsets(13, 18));
    }

    #[test]
    fn test_find_regex_word_boundary() {
        let doc = Document::from_text("hello helloworld world");
        let query = SearchQuery::new(r"\bhello\b".to_string()).use_regex(true);

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 1);
        assert_eq!(result.matches[0], Range::from_offsets(0, 5));
    }

    #[test]
    fn test_find_regex_multiple_patterns() {
        let doc = Document::from_text("cat dog cat bird dog");
        let query = SearchQuery::new("cat|dog".to_string()).use_regex(true);

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 4);
    }

    #[test]
    fn test_find_regex_invalid_pattern() {
        let doc = Document::from_text("Hello World");
        let query = SearchQuery::new("[invalid".to_string()).use_regex(true);

        let result = doc.find(&query);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_empty_pattern() {
        let doc = Document::from_text("Hello");
        let query = SearchQuery::new("".to_string());

        let result = doc.find(&query).unwrap();
        // Empty pattern should match nothing in literal mode
        // (empty string find returns empty result to avoid infinite loops)
        assert_eq!(result.count(), 0);
    }

    #[test]
    fn test_find_unicode_text() {
        let doc = Document::from_text("Hello 世界 World");
        let query = SearchQuery::new("世界".to_string());

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 1);
        assert_eq!(result.matches[0], Range::from_offsets(6, 8));
    }

    #[test]
    fn test_find_at_document_boundaries() {
        let doc = Document::from_text("test");
        let query = SearchQuery::new("test".to_string());

        let result = doc.find(&query).unwrap();
        assert_eq!(result.count(), 1);
        assert_eq!(result.matches[0], Range::from_offsets(0, 4));
    }

    #[test]
    fn test_find_and_replace_simple() {
        let mut doc = Document::from_text("Hello World");
        let query = SearchQuery::new("World".to_string());

        let count = doc.find_and_replace(&query, "Rust").unwrap();
        assert_eq!(count, 1);
        assert_eq!(doc.get_content(), "Hello Rust");
    }

    #[test]
    fn test_find_and_replace_multiple() {
        let mut doc = Document::from_text("cat dog cat bird cat");
        let query = SearchQuery::new("cat".to_string());

        let count = doc.find_and_replace(&query, "fox").unwrap();
        assert_eq!(count, 3);
        assert_eq!(doc.get_content(), "fox dog fox bird fox");
    }

    #[test]
    fn test_find_and_replace_case_insensitive() {
        let mut doc = Document::from_text("Hello hello HELLO");
        let query = SearchQuery::new("hello".to_string());

        let count = doc.find_and_replace(&query, "hi").unwrap();
        assert_eq!(count, 3);
        assert_eq!(doc.get_content(), "hi hi hi");
    }

    #[test]
    fn test_find_and_replace_case_sensitive() {
        let mut doc = Document::from_text("Hello hello HELLO");
        let query = SearchQuery::new("hello".to_string()).case_sensitive(true);

        let count = doc.find_and_replace(&query, "hi").unwrap();
        assert_eq!(count, 1);
        assert_eq!(doc.get_content(), "Hello hi HELLO");
    }

    #[test]
    fn test_find_and_replace_no_matches() {
        let mut doc = Document::from_text("Hello World");
        let query = SearchQuery::new("xyz".to_string());

        let count = doc.find_and_replace(&query, "abc").unwrap();
        assert_eq!(count, 0);
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_find_and_replace_with_regex() {
        let mut doc = Document::from_text("Hello 123 World 456");
        let query = SearchQuery::new(r"\d+".to_string()).use_regex(true);

        let count = doc.find_and_replace(&query, "NUM").unwrap();
        assert_eq!(count, 2);
        assert_eq!(doc.get_content(), "Hello NUM World NUM");
    }

    #[test]
    fn test_find_and_replace_undo() {
        let mut doc = Document::from_text("Hello World");
        let query = SearchQuery::new("World".to_string());

        doc.find_and_replace(&query, "Rust").unwrap();
        assert_eq!(doc.get_content(), "Hello Rust");

        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_find_and_replace_undo_multiple() {
        let mut doc = Document::from_text("cat dog cat bird cat");
        let query = SearchQuery::new("cat".to_string());

        doc.find_and_replace(&query, "fox").unwrap();
        assert_eq!(doc.get_content(), "fox dog fox bird fox");

        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "cat dog cat bird cat");
    }

    #[test]
    fn test_find_and_replace_redo() {
        let mut doc = Document::from_text("Hello World");
        let query = SearchQuery::new("World".to_string());

        doc.find_and_replace(&query, "Rust").unwrap();
        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        doc.redo().unwrap();
        assert_eq!(doc.get_content(), "Hello Rust");
    }

    #[test]
    fn test_find_and_replace_different_lengths() {
        let mut doc = Document::from_text("a b a c a");
        let query = SearchQuery::new("a".to_string());

        let count = doc.find_and_replace(&query, "longer").unwrap();
        assert_eq!(count, 3);
        assert_eq!(doc.get_content(), "longer b longer c longer");
    }

    #[test]
    fn test_find_and_replace_with_empty_replacement() {
        let mut doc = Document::from_text("Hello World");
        let query = SearchQuery::new(" ".to_string());

        let count = doc.find_and_replace(&query, "").unwrap();
        assert_eq!(count, 1);
        assert_eq!(doc.get_content(), "HelloWorld");
    }

    #[test]
    fn test_find_and_replace_unicode() {
        let mut doc = Document::from_text("Hello 世界");
        let query = SearchQuery::new("世界".to_string());

        let count = doc.find_and_replace(&query, "World").unwrap();
        assert_eq!(count, 1);
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_find_and_replace_command_description() {
        let query = SearchQuery::new("test".to_string());
        let cmd = FindAndReplaceCommand::new(query, "replacement".to_string());

        let desc = cmd.description();
        assert!(desc.contains("test"));
        assert!(desc.contains("replacement"));
    }
}
