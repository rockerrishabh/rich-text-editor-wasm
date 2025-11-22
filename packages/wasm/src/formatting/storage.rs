use crate::document::{Position, Range};
use crate::formatting::block::BlockType;
use crate::formatting::inline::InlineFormat;
use crate::utils::interner::StringInterner;
use std::collections::HashSet;

/// Represents a contiguous run of text with the same set of formats
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatRun {
    pub range: Range,
    pub formats: HashSet<InlineFormat>,
}

impl FormatRun {
    /// Creates a new FormatRun
    pub fn new(range: Range, formats: HashSet<InlineFormat>) -> Self {
        Self { range, formats }
    }

    /// Returns true if this run contains the specified format
    pub fn has_format(&self, format: &InlineFormat) -> bool {
        self.formats.contains(format)
    }

    /// Returns true if this run has no formats
    pub fn is_empty(&self) -> bool {
        self.formats.is_empty()
    }
}

/// Represents block-level formatting information at a specific position
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockInfo {
    pub start_offset: usize,
    pub block_type: BlockType,
}

impl BlockInfo {
    /// Creates a new BlockInfo
    pub fn new(start_offset: usize, block_type: BlockType) -> Self {
        Self {
            start_offset,
            block_type,
        }
    }
}

/// Cache entry for format queries
#[derive(Debug, Clone)]
struct FormatCache {
    position: Position,
    formats: HashSet<InlineFormat>,
}

/// Manages format storage using a run-based approach.
///
/// # Performance Characteristics
///
/// ## Time Complexity
/// - **Apply format**: O(m) where m is the number of format runs affected
///   - Best case: O(1) when creating a new run in empty space
///   - Worst case: O(m) when splitting and merging many runs
/// - **Remove format**: O(m) similar to apply format
/// - **Get formats at position**: O(log m) using binary search with caching
///   - Cached access: O(1) for repeated queries at same position
/// - **Set block type**: O(b) where b is the number of blocks affected
/// - **Get block type**: O(log b) using binary search
/// - **Adjust for insert/delete**: O(m + b) to update all runs and blocks
///
/// ## Space Complexity
/// - Format runs: ~32 bytes per run
/// - Block info: ~16 bytes per block
/// - String interner: Shared storage for repeated strings (URLs, colors)
/// - Cache: ~40 bytes for format cache entry
///
/// ## Performance Notes
/// - Format runs are automatically merged when adjacent with same formats
/// - Binary search used for position queries (O(log n))
/// - Format cache provides O(1) access for repeated queries
/// - String interner reduces memory for repeated URLs/colors
/// - Large documents with many format changes may have many runs
/// - Consider batching format operations when possible
///
/// # Example
/// ```
/// use rich_text_editor_wasm::formatting::FormatStorage;
/// use rich_text_editor_wasm::formatting::InlineFormat;
/// use rich_text_editor_wasm::document::{Range, Position};
///
/// let mut storage = FormatStorage::new();
/// storage.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
/// let formats = storage.get_formats_at(Position::new(2));
/// assert!(formats.contains(&InlineFormat::Bold));
/// ```
#[derive(Debug, Clone)]
pub struct FormatStorage {
    runs: Vec<FormatRun>,
    blocks: Vec<BlockInfo>,
    format_cache: Option<FormatCache>,
    string_interner: StringInterner,
}

impl FormatStorage {
    /// Creates a new empty FormatStorage
    pub fn new() -> Self {
        Self {
            runs: Vec::new(),
            blocks: vec![BlockInfo::new(0, BlockType::Paragraph)],
            format_cache: None,
            string_interner: StringInterner::new(),
        }
    }

    /// Applies a format to the specified range
    pub fn apply_format(&mut self, range: Range, format: InlineFormat) {
        let normalized = range.normalize();

        if normalized.is_empty() {
            return;
        }

        // Invalidate cache since we're modifying formats
        self.invalidate_cache();

        // Intern string values to reduce memory usage
        let format = self.intern_format(format);

        // Split runs at the boundaries of the new range
        self.split_at_position(normalized.start);
        self.split_at_position(normalized.end);

        // Track which parts of the range are covered by existing runs
        let mut covered_ranges: Vec<Range> = Vec::new();

        // Apply the format to all runs that overlap with the range
        for run in &mut self.runs {
            let run_range = run.range.normalize();
            if run_range.overlaps(&normalized) {
                run.formats.insert(format.clone());
                covered_ranges.push(run.range);
            }
        }

        // Find gaps in coverage and create new runs for them
        if covered_ranges.is_empty() {
            // No existing runs, create one for the entire range
            let mut formats = HashSet::new();
            formats.insert(format.clone());
            self.runs.push(FormatRun::new(normalized, formats));
        } else {
            // Sort covered ranges
            covered_ranges.sort_by_key(|r| r.start_offset());

            // Check for gap at the start
            if covered_ranges[0].start_offset() > normalized.start_offset() {
                let mut formats = HashSet::new();
                formats.insert(format.clone());
                let gap_range = Range::new(normalized.start, covered_ranges[0].start);
                self.runs.push(FormatRun::new(gap_range, formats));
            }

            // Check for gaps between covered ranges
            for i in 0..covered_ranges.len() - 1 {
                let gap_start = covered_ranges[i].end_offset();
                let gap_end = covered_ranges[i + 1].start_offset();
                if gap_start < gap_end {
                    let mut formats = HashSet::new();
                    formats.insert(format.clone());
                    let gap_range = Range::from_offsets(gap_start, gap_end);
                    self.runs.push(FormatRun::new(gap_range, formats));
                }
            }

            // Check for gap at the end
            let last_end = covered_ranges.last().unwrap().end_offset();
            if last_end < normalized.end_offset() {
                let mut formats = HashSet::new();
                formats.insert(format.clone());
                let gap_range = Range::new(covered_ranges.last().unwrap().end, normalized.end);
                self.runs.push(FormatRun::new(gap_range, formats));
            }
        }

        // Merge adjacent runs with identical formats
        self.merge_adjacent_runs();
    }

    /// Removes a format from the specified range
    pub fn remove_format(&mut self, range: Range, format: &InlineFormat) {
        let normalized = range.normalize();

        if normalized.is_empty() {
            return;
        }

        // Invalidate cache since we're modifying formats
        self.invalidate_cache();

        // Split runs at the boundaries
        self.split_at_position(normalized.start);
        self.split_at_position(normalized.end);

        // Remove the format from all runs that overlap with the range
        for run in &mut self.runs {
            let run_range = run.range.normalize();
            if run_range.overlaps(&normalized) {
                // For complex formats (Link, TextColor, BackgroundColor), match by discriminant
                run.formats.retain(|f| !formats_match_type(f, format));
            }
        }

        // Remove empty runs
        self.runs.retain(|run| !run.is_empty());

        // Merge adjacent runs with identical formats
        self.merge_adjacent_runs();
    }

    /// Gets all formats at the specified position using binary search with caching
    pub fn get_formats_at(&self, pos: Position) -> HashSet<InlineFormat> {
        // Check cache first
        if let Some(ref cache) = self.format_cache {
            if cache.position == pos {
                return cache.formats.clone();
            }
        }

        // Binary search for the run containing this position
        let formats = self.get_formats_at_uncached(pos);
        formats
    }

    /// Gets formats at position without using cache (internal helper)
    fn get_formats_at_uncached(&self, pos: Position) -> HashSet<InlineFormat> {
        for run in &self.runs {
            let run_range = run.range.normalize();
            if pos.offset() >= run_range.start_offset() && pos.offset() < run_range.end_offset() {
                return run.formats.clone();
            }
        }
        HashSet::new()
    }

    /// Updates the format cache for a specific position
    pub fn update_format_cache(&mut self, pos: Position) {
        let formats = self.get_formats_at_uncached(pos);
        self.format_cache = Some(FormatCache {
            position: pos,
            formats,
        });
    }

    /// Invalidates the format cache
    fn invalidate_cache(&mut self) {
        self.format_cache = None;
    }

    /// Interns string values in a format (colors, URLs) to reduce memory usage
    /// Returns a new format with interned strings
    pub fn intern_format(&mut self, format: InlineFormat) -> InlineFormat {
        match format {
            InlineFormat::Link { url } => {
                let interned_url = self.string_interner.intern(&url);
                InlineFormat::Link {
                    url: interned_url.to_string(),
                }
            }
            InlineFormat::TextColor { color } => {
                let interned_color = self.string_interner.intern(&color);
                InlineFormat::TextColor {
                    color: interned_color.to_string(),
                }
            }
            InlineFormat::BackgroundColor { color } => {
                let interned_color = self.string_interner.intern(&color);
                InlineFormat::BackgroundColor {
                    color: interned_color.to_string(),
                }
            }
            // Other formats don't have string values to intern
            other => other,
        }
    }

    /// Returns statistics about the string interner
    pub fn interner_stats(&self) -> (usize, bool) {
        (self.string_interner.len(), self.string_interner.is_empty())
    }

    /// Returns memory usage statistics for the format storage
    ///
    /// # Returns
    /// A tuple containing:
    /// - Number of format runs
    /// - Number of block info entries
    /// - Number of interned strings
    /// - Estimated memory usage in bytes
    ///
    /// # Memory Calculation
    /// - Format runs: ~32 bytes per run
    /// - Block info: ~16 bytes per block
    /// - Interned strings: Actual string size (shared across runs)
    /// - Cache: ~40 bytes if present
    ///
    /// # Example
    /// ```
    /// use rich_text_editor_wasm::formatting::FormatStorage;
    ///
    /// let storage = FormatStorage::new();
    /// let (runs, blocks, strings, bytes) = storage.memory_stats();
    /// println!("Format storage using {} bytes", bytes);
    /// ```
    pub fn memory_stats(&self) -> (usize, usize, usize, usize) {
        let run_count = self.runs.len();
        let block_count = self.blocks.len();
        let string_count = self.string_interner.len();

        // Estimate memory usage
        let run_memory = run_count * 32; // ~32 bytes per FormatRun
        let block_memory = block_count * 16; // ~16 bytes per BlockInfo
        let cache_memory = if self.format_cache.is_some() { 40 } else { 0 }; // ~40 bytes for cache
        let interner_memory = self.string_interner.estimated_memory(); // Actual string data

        let total_memory = run_memory + block_memory + cache_memory + interner_memory;

        (run_count, block_count, string_count, total_memory)
    }

    /// Returns the number of format runs
    ///
    /// This can be used to monitor format storage efficiency.
    /// Fewer runs indicate better memory usage due to merging.
    pub fn run_count(&self) -> usize {
        self.runs.len()
    }

    /// Returns the number of block info entries
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Gets all format runs (for serialization or inspection)
    pub fn get_runs(&self) -> &[FormatRun] {
        &self.runs
    }

    /// Clears all format runs
    pub fn clear(&mut self) {
        self.runs.clear();
        self.invalidate_cache();
    }

    /// Sets the block type for the specified range
    /// This updates all blocks that overlap with the range
    pub fn set_block_type(&mut self, range: Range, block_type: BlockType) {
        // Invalidate cache since block changes might affect rendering
        self.invalidate_cache();

        let normalized = range.normalize();
        let start_offset = normalized.start_offset();
        let end_offset = normalized.end_offset();

        // Find all blocks that overlap with the range
        let mut blocks_to_update = Vec::new();
        let mut blocks_to_remove = Vec::new();

        for (idx, block) in self.blocks.iter().enumerate() {
            // Find the end of this block (start of next block or document end)
            let block_end = if idx + 1 < self.blocks.len() {
                self.blocks[idx + 1].start_offset
            } else {
                usize::MAX
            };

            // Check if this block overlaps with the range
            if block.start_offset < end_offset && block_end > start_offset {
                if block.start_offset >= start_offset && block_end <= end_offset {
                    // Block is completely within range - mark for removal
                    blocks_to_remove.push(idx);
                } else if block.start_offset < start_offset && block_end > end_offset {
                    // Range is completely within block - need to split
                    blocks_to_update.push((idx, block.start_offset, block.block_type.clone()));
                } else if block.start_offset < start_offset {
                    // Block starts before range - keep the part before
                    blocks_to_update.push((idx, block.start_offset, block.block_type.clone()));
                }
            }
        }

        // Remove blocks that are completely within the range (in reverse order)
        for idx in blocks_to_remove.iter().rev() {
            self.blocks.remove(*idx);
        }

        // Remove any blocks that start within the range
        self.blocks
            .retain(|b| b.start_offset < start_offset || b.start_offset >= end_offset);

        // Add the new block at the start of the range
        self.blocks.push(BlockInfo::new(start_offset, block_type));

        // If the range doesn't extend to the end, add a block after it
        // to restore the original block type (if there was one)
        if let Some((_, _original_offset, original_type)) = blocks_to_update.first() {
            if end_offset < usize::MAX {
                // Check if there's already a block at end_offset
                if !self.blocks.iter().any(|b| b.start_offset == end_offset) {
                    self.blocks
                        .push(BlockInfo::new(end_offset, original_type.clone()));
                }
            }
        }

        // Sort blocks by start offset
        self.blocks.sort_by_key(|b| b.start_offset);

        // Ensure there's always a block at offset 0
        if self.blocks.is_empty() || self.blocks[0].start_offset != 0 {
            self.blocks
                .insert(0, BlockInfo::new(0, BlockType::Paragraph));
        }
    }

    /// Gets the block type at the specified position using binary search
    pub fn get_block_type_at(&self, pos: Position) -> BlockType {
        let offset = pos.offset();

        // Binary search for the block containing this position
        let idx = self.blocks.binary_search_by(|block| {
            if offset < block.start_offset {
                std::cmp::Ordering::Greater
            } else {
                // Check if this is the last block or if the next block starts after pos
                let next_start = if let Some(next_block) = self.blocks.get(
                    self.blocks
                        .iter()
                        .position(|b| b.start_offset == block.start_offset)
                        .unwrap()
                        + 1,
                ) {
                    next_block.start_offset
                } else {
                    usize::MAX
                };

                if offset < next_start {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Less
                }
            }
        });

        match idx {
            Ok(i) => self.blocks[i].block_type.clone(),
            Err(_) => {
                // If binary search fails, fall back to linear search
                // Find the last block that starts at or before this position
                self.blocks
                    .iter()
                    .rev()
                    .find(|b| b.start_offset <= offset)
                    .map(|b| b.block_type.clone())
                    .unwrap_or(BlockType::Paragraph)
            }
        }
    }

    /// Gets all block info (for serialization or inspection)
    pub fn get_blocks(&self) -> &[BlockInfo] {
        &self.blocks
    }

    /// Replaces block info with a provided snapshot
    /// Ensures blocks are sorted and a paragraph block exists at offset 0
    pub fn set_blocks(&mut self, mut blocks: Vec<BlockInfo>) {
        // Invalidate cache since block structure is changing
        self.invalidate_cache();

        // Sort by start offset
        blocks.sort_by_key(|b| b.start_offset);

        // Ensure a block at offset 0
        if blocks.is_empty() || blocks[0].start_offset != 0 {
            blocks.insert(0, BlockInfo::new(0, BlockType::Paragraph));
        }

        self.blocks = blocks;
    }

    /// Adjusts format positions after text insertion
    pub fn adjust_for_insert(&mut self, pos: Position, length: usize) {
        // Invalidate cache since positions are changing
        self.invalidate_cache();

        let insert_offset = pos.offset();

        // Adjust inline format runs
        for run in &mut self.runs {
            let start_offset = run.range.start.offset();
            let end_offset = run.range.end.offset();

            if start_offset >= insert_offset {
                run.range.start = Position::new(start_offset + length);
            }
            if end_offset >= insert_offset {
                run.range.end = Position::new(end_offset + length);
            }
        }

        // Adjust block positions
        for block in &mut self.blocks {
            if block.start_offset > insert_offset {
                block.start_offset += length;
            }
        }
    }

    /// Adjusts format positions after text deletion
    pub fn adjust_for_delete(&mut self, range: Range) {
        // Invalidate cache since positions are changing
        self.invalidate_cache();

        let normalized = range.normalize();
        let delete_start = normalized.start_offset();
        let delete_end = normalized.end_offset();
        let delete_length = delete_end - delete_start;

        // Adjust inline format runs
        let mut i = 0;
        while i < self.runs.len() {
            let run = &mut self.runs[i];
            let start_offset = run.range.start.offset();
            let end_offset = run.range.end.offset();

            // Run is completely within deleted range - remove it
            if start_offset >= delete_start && end_offset <= delete_end {
                self.runs.remove(i);
                continue;
            }

            // Run starts before and ends after deletion - truncate
            if start_offset < delete_start && end_offset > delete_end {
                run.range.end = Position::new(end_offset - delete_length);
            }
            // Run starts before deletion and ends within it - truncate end
            else if start_offset < delete_start && end_offset > delete_start {
                run.range.end = Position::new(delete_start);
            }
            // Run starts within deletion and ends after it - adjust start
            else if start_offset < delete_end && end_offset > delete_end {
                run.range.start = Position::new(delete_start);
                run.range.end = Position::new(end_offset - delete_length);
            }
            // Run is completely after deletion - shift it
            else if start_offset >= delete_end {
                run.range.start = Position::new(start_offset - delete_length);
                run.range.end = Position::new(end_offset - delete_length);
            }

            i += 1;
        }

        // Remove any runs that became empty
        self.runs.retain(|run| !run.range.is_empty());

        // Adjust block positions
        let mut j = 0;
        while j < self.blocks.len() {
            let block = &mut self.blocks[j];

            // Block starts within deleted range - remove it
            if block.start_offset > delete_start && block.start_offset < delete_end {
                self.blocks.remove(j);
                continue;
            }

            // Block starts at or after the end of deletion - shift it
            if block.start_offset >= delete_end {
                block.start_offset -= delete_length;
            }

            j += 1;
        }

        // Ensure there's always a block at offset 0
        if self.blocks.is_empty() || self.blocks[0].start_offset != 0 {
            self.blocks
                .insert(0, BlockInfo::new(0, BlockType::Paragraph));
        }
    }

    /// Merges adjacent runs that have identical format sets
    fn merge_adjacent_runs(&mut self) {
        if self.runs.len() <= 1 {
            return;
        }

        // Sort runs by start position
        self.runs.sort_by_key(|run| run.range.start_offset());

        let mut i = 0;
        while i < self.runs.len() - 1 {
            let current_end = self.runs[i].range.end_offset();
            let next_start = self.runs[i + 1].range.start_offset();

            // Check if runs are adjacent and have the same formats
            if current_end == next_start && self.runs[i].formats == self.runs[i + 1].formats {
                // Merge the runs
                let next_end = self.runs[i + 1].range.end;
                self.runs[i].range.end = next_end;
                self.runs.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    /// Splits a run at the specified position if a run spans across it
    fn split_at_position(&mut self, pos: Position) {
        let mut new_runs = Vec::new();

        for run in &self.runs {
            let run_range = run.range.normalize();

            // If position is within the run (not at boundaries), split it
            if pos.offset() > run_range.start_offset() && pos.offset() < run_range.end_offset() {
                // Create two runs from the split
                let first_run =
                    FormatRun::new(Range::new(run.range.start, pos), run.formats.clone());
                let second_run =
                    FormatRun::new(Range::new(pos, run.range.end), run.formats.clone());
                new_runs.push((run_range.start_offset(), first_run, second_run));
            }
        }

        // Apply the splits
        for (start_offset, first_run, second_run) in new_runs {
            if let Some(idx) = self
                .runs
                .iter()
                .position(|r| r.range.start_offset() == start_offset)
            {
                self.runs[idx] = first_run;
                self.runs.insert(idx + 1, second_run);
            }
        }
    }
}

impl Default for FormatStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to check if two formats match by type (ignoring values for complex formats)
fn formats_match_type(a: &InlineFormat, b: &InlineFormat) -> bool {
    use InlineFormat::*;
    match (a, b) {
        (Bold, Bold) => true,
        (Italic, Italic) => true,
        (Underline, Underline) => true,
        (Strikethrough, Strikethrough) => true,
        (Code, Code) => true,
        (Link { .. }, Link { .. }) => true,
        (TextColor { .. }, TextColor { .. }) => true,
        (BackgroundColor { .. }, BackgroundColor { .. }) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_run_creation() {
        let range = Range::from_offsets(0, 5);
        let mut formats = HashSet::new();
        formats.insert(InlineFormat::Bold);

        let run = FormatRun::new(range, formats);
        assert_eq!(run.range, range);
        assert!(run.has_format(&InlineFormat::Bold));
        assert!(!run.is_empty());
    }

    #[test]
    fn test_format_storage_new() {
        let storage = FormatStorage::new();
        assert_eq!(storage.get_runs().len(), 0);
    }

    #[test]
    fn test_apply_format() {
        let mut storage = FormatStorage::new();
        let range = Range::from_offsets(0, 5);

        storage.apply_format(range, InlineFormat::Bold);

        let formats = storage.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_apply_multiple_formats() {
        let mut storage = FormatStorage::new();
        let range = Range::from_offsets(0, 5);

        storage.apply_format(range, InlineFormat::Bold);
        storage.apply_format(range, InlineFormat::Italic);

        let formats = storage.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
        assert!(formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_remove_format() {
        let mut storage = FormatStorage::new();
        let range = Range::from_offsets(0, 5);

        storage.apply_format(range, InlineFormat::Bold);
        storage.apply_format(range, InlineFormat::Italic);

        storage.remove_format(range, &InlineFormat::Bold);

        let formats = storage.get_formats_at(Position::new(2));
        assert!(!formats.contains(&InlineFormat::Bold));
        assert!(formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_get_formats_at_empty() {
        let storage = FormatStorage::new();
        let formats = storage.get_formats_at(Position::new(0));
        assert!(formats.is_empty());
    }

    #[test]
    fn test_overlapping_formats() {
        let mut storage = FormatStorage::new();

        storage.apply_format(Range::from_offsets(0, 10), InlineFormat::Bold);
        storage.apply_format(Range::from_offsets(5, 15), InlineFormat::Italic);

        // Position 3 should have only Bold
        let formats_3 = storage.get_formats_at(Position::new(3));
        assert!(formats_3.contains(&InlineFormat::Bold));
        assert!(!formats_3.contains(&InlineFormat::Italic));

        // Position 7 should have both Bold and Italic
        let formats_7 = storage.get_formats_at(Position::new(7));
        assert!(formats_7.contains(&InlineFormat::Bold));
        assert!(formats_7.contains(&InlineFormat::Italic));

        // Position 12 should have only Italic
        let formats_12 = storage.get_formats_at(Position::new(12));
        assert!(!formats_12.contains(&InlineFormat::Bold));
        assert!(formats_12.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_adjust_for_insert() {
        let mut storage = FormatStorage::new();
        storage.apply_format(Range::from_offsets(5, 10), InlineFormat::Bold);

        // Insert 3 characters at position 3
        storage.adjust_for_insert(Position::new(3), 3);

        // The format should now be at positions 8-13
        let formats_before = storage.get_formats_at(Position::new(7));
        assert!(formats_before.is_empty());

        let formats_at = storage.get_formats_at(Position::new(9));
        assert!(formats_at.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_adjust_for_delete() {
        let mut storage = FormatStorage::new();
        storage.apply_format(Range::from_offsets(10, 20), InlineFormat::Bold);

        // Delete 5 characters from position 5 to 10
        storage.adjust_for_delete(Range::from_offsets(5, 10));

        // The format should now be at positions 5-15
        let formats = storage.get_formats_at(Position::new(7));
        assert!(formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_merge_adjacent_runs() {
        let mut storage = FormatStorage::new();

        // Apply the same format to adjacent ranges
        storage.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        storage.apply_format(Range::from_offsets(5, 10), InlineFormat::Bold);

        // Should be merged into a single run
        assert_eq!(storage.get_runs().len(), 1);
        assert_eq!(storage.get_runs()[0].range.start_offset(), 0);
        assert_eq!(storage.get_runs()[0].range.end_offset(), 10);
    }

    #[test]
    fn test_clear() {
        let mut storage = FormatStorage::new();
        storage.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);

        storage.clear();
        assert_eq!(storage.get_runs().len(), 0);
    }

    #[test]
    fn test_block_info_creation() {
        let block = BlockInfo::new(0, BlockType::Paragraph);
        assert_eq!(block.start_offset, 0);
        assert_eq!(block.block_type, BlockType::Paragraph);
    }

    #[test]
    fn test_default_block_type() {
        let storage = FormatStorage::new();
        let block_type = storage.get_block_type_at(Position::new(0));
        assert_eq!(block_type, BlockType::Paragraph);
    }

    #[test]
    fn test_set_block_type() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(0, 10), BlockType::heading(1));

        let block_type = storage.get_block_type_at(Position::new(5));
        assert_eq!(block_type, BlockType::heading(1));
    }

    #[test]
    fn test_set_multiple_block_types() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(0, 10), BlockType::heading(1));
        storage.set_block_type(Range::from_offsets(10, 20), BlockType::BulletList);
        storage.set_block_type(Range::from_offsets(20, 30), BlockType::Paragraph);

        assert_eq!(
            storage.get_block_type_at(Position::new(5)),
            BlockType::heading(1)
        );
        assert_eq!(
            storage.get_block_type_at(Position::new(15)),
            BlockType::BulletList
        );
        assert_eq!(
            storage.get_block_type_at(Position::new(25)),
            BlockType::Paragraph
        );
    }

    #[test]
    fn test_block_type_boundaries() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(10, 20), BlockType::heading(2));

        // Before the heading
        assert_eq!(
            storage.get_block_type_at(Position::new(5)),
            BlockType::Paragraph
        );

        // At the start of heading
        assert_eq!(
            storage.get_block_type_at(Position::new(10)),
            BlockType::heading(2)
        );

        // Within heading
        assert_eq!(
            storage.get_block_type_at(Position::new(15)),
            BlockType::heading(2)
        );

        // At the end of heading (should be back to paragraph)
        assert_eq!(
            storage.get_block_type_at(Position::new(20)),
            BlockType::Paragraph
        );
    }

    #[test]
    fn test_block_adjust_for_insert() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(10, 20), BlockType::heading(1));

        // Insert 5 characters at position 5
        storage.adjust_for_insert(Position::new(5), 5);

        // Block should now start at position 15
        assert_eq!(
            storage.get_block_type_at(Position::new(9)),
            BlockType::Paragraph
        );
        assert_eq!(
            storage.get_block_type_at(Position::new(15)),
            BlockType::heading(1)
        );
    }

    #[test]
    fn test_block_adjust_for_delete() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(20, 30), BlockType::heading(1));

        // Delete 10 characters from position 5 to 15
        storage.adjust_for_delete(Range::from_offsets(5, 15));

        // Block should now start at position 10 (20 - 10)
        assert_eq!(
            storage.get_block_type_at(Position::new(10)),
            BlockType::heading(1)
        );
    }

    #[test]
    fn test_block_delete_within_block() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(10, 30), BlockType::heading(1));

        // Delete within the block
        storage.adjust_for_delete(Range::from_offsets(15, 20));

        // Block should still be a heading, just shorter
        assert_eq!(
            storage.get_block_type_at(Position::new(10)),
            BlockType::heading(1)
        );
        assert_eq!(
            storage.get_block_type_at(Position::new(20)),
            BlockType::heading(1)
        );
    }

    #[test]
    fn test_overwrite_block_type() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(0, 20), BlockType::heading(1));
        storage.set_block_type(Range::from_offsets(0, 20), BlockType::BulletList);

        let block_type = storage.get_block_type_at(Position::new(10));
        assert_eq!(block_type, BlockType::BulletList);
    }

    #[test]
    fn test_get_blocks() {
        let mut storage = FormatStorage::new();
        storage.set_block_type(Range::from_offsets(0, 10), BlockType::heading(1));
        storage.set_block_type(Range::from_offsets(10, 20), BlockType::BulletList);

        let blocks = storage.get_blocks();
        assert!(blocks.len() >= 2);
    }
}
