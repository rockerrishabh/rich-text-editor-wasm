/// Represents a position in the document as a character offset
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub offset: usize,
}

impl Position {
    /// Creates a new Position at the specified offset
    pub fn new(offset: usize) -> Self {
        Self { offset }
    }

    /// Returns the offset value
    pub fn offset(&self) -> usize {
        self.offset
    }
}

/// Represents a range in the document with start and end positions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    /// Creates a new Range with the specified start and end positions
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Creates a Range from offset values
    pub fn from_offsets(start: usize, end: usize) -> Self {
        Self {
            start: Position::new(start),
            end: Position::new(end),
        }
    }

    /// Returns true if the range is empty (start == end)
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns true if the range contains the specified position
    pub fn contains(&self, pos: Position) -> bool {
        let (start, end) = self.normalized_offsets();
        pos.offset >= start && pos.offset < end
    }

    /// Returns the length of the range
    pub fn len(&self) -> usize {
        let (start, end) = self.normalized_offsets();
        end - start
    }

    /// Returns a normalized version of the range where start <= end
    pub fn normalize(&self) -> Self {
        if self.start.offset <= self.end.offset {
            *self
        } else {
            Self {
                start: self.end,
                end: self.start,
            }
        }
    }

    /// Returns the normalized start and end offsets as a tuple
    fn normalized_offsets(&self) -> (usize, usize) {
        if self.start.offset <= self.end.offset {
            (self.start.offset, self.end.offset)
        } else {
            (self.end.offset, self.start.offset)
        }
    }

    /// Returns true if this range overlaps with another range
    pub fn overlaps(&self, other: &Range) -> bool {
        let (self_start, self_end) = self.normalized_offsets();
        let (other_start, other_end) = other.normalized_offsets();
        self_start < other_end && other_start < self_end
    }

    /// Returns the start offset (normalized)
    pub fn start_offset(&self) -> usize {
        self.start.offset.min(self.end.offset)
    }

    /// Returns the end offset (normalized)
    pub fn end_offset(&self) -> usize {
        self.start.offset.max(self.end.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(42);
        assert_eq!(pos.offset, 42);
        assert_eq!(pos.offset(), 42);
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position::new(10);
        let pos2 = Position::new(10);
        let pos3 = Position::new(20);
        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_range_creation() {
        let range = Range::new(Position::new(5), Position::new(10));
        assert_eq!(range.start.offset, 5);
        assert_eq!(range.end.offset, 10);
    }

    #[test]
    fn test_range_from_offsets() {
        let range = Range::from_offsets(5, 10);
        assert_eq!(range.start.offset, 5);
        assert_eq!(range.end.offset, 10);
    }

    #[test]
    fn test_range_is_empty() {
        let empty_range = Range::new(Position::new(5), Position::new(5));
        let non_empty_range = Range::new(Position::new(5), Position::new(10));
        assert!(empty_range.is_empty());
        assert!(!non_empty_range.is_empty());
    }

    #[test]
    fn test_range_contains() {
        let range = Range::new(Position::new(5), Position::new(10));
        assert!(range.contains(Position::new(5)));
        assert!(range.contains(Position::new(7)));
        assert!(!range.contains(Position::new(10)));
        assert!(!range.contains(Position::new(3)));
        assert!(!range.contains(Position::new(15)));
    }

    #[test]
    fn test_range_len() {
        let range = Range::new(Position::new(5), Position::new(10));
        assert_eq!(range.len(), 5);

        let empty_range = Range::new(Position::new(5), Position::new(5));
        assert_eq!(empty_range.len(), 0);
    }

    #[test]
    fn test_range_normalize() {
        let forward_range = Range::new(Position::new(5), Position::new(10));
        let backward_range = Range::new(Position::new(10), Position::new(5));

        let normalized_forward = forward_range.normalize();
        let normalized_backward = backward_range.normalize();

        assert_eq!(normalized_forward.start.offset, 5);
        assert_eq!(normalized_forward.end.offset, 10);
        assert_eq!(normalized_backward.start.offset, 5);
        assert_eq!(normalized_backward.end.offset, 10);
    }

    #[test]
    fn test_range_normalize_with_backward_range() {
        let backward_range = Range::new(Position::new(10), Position::new(5));
        assert_eq!(backward_range.len(), 5);
        assert!(backward_range.contains(Position::new(7)));
    }

    #[test]
    fn test_range_overlaps() {
        let range1 = Range::new(Position::new(5), Position::new(10));
        let range2 = Range::new(Position::new(8), Position::new(15));
        let range3 = Range::new(Position::new(15), Position::new(20));

        assert!(range1.overlaps(&range2));
        assert!(range2.overlaps(&range1));
        assert!(!range1.overlaps(&range3));
        assert!(!range3.overlaps(&range1));
    }

    #[test]
    fn test_range_start_end_offsets() {
        let forward_range = Range::new(Position::new(5), Position::new(10));
        assert_eq!(forward_range.start_offset(), 5);
        assert_eq!(forward_range.end_offset(), 10);

        let backward_range = Range::new(Position::new(10), Position::new(5));
        assert_eq!(backward_range.start_offset(), 5);
        assert_eq!(backward_range.end_offset(), 10);
    }
}
