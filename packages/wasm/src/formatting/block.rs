use serde::{Deserialize, Serialize};

/// Represents block-level formatting types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockType {
    /// Standard paragraph block
    Paragraph,
    /// Heading block with level 1-6
    Heading { level: u8 },
    /// Unordered (bullet) list item
    BulletList,
    /// Ordered (numbered) list item
    NumberedList,
    /// Block quote
    BlockQuote,
    /// Code block
    CodeBlock,
}

impl BlockType {
    /// Creates a new Heading block type with the specified level (1-6)
    /// Panics if level is not in the range 1-6
    pub fn heading(level: u8) -> Self {
        assert!(
            (1..=6).contains(&level),
            "Heading level must be between 1 and 6"
        );
        BlockType::Heading { level }
    }

    /// Returns true if this is a Heading block type
    pub fn is_heading(&self) -> bool {
        matches!(self, BlockType::Heading { .. })
    }

    /// Returns the heading level if this is a Heading, None otherwise
    pub fn heading_level(&self) -> Option<u8> {
        match self {
            BlockType::Heading { level } => Some(*level),
            _ => None,
        }
    }
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Paragraph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_type_creation() {
        let paragraph = BlockType::Paragraph;
        assert_eq!(paragraph, BlockType::Paragraph);

        let heading = BlockType::heading(1);
        assert_eq!(heading, BlockType::Heading { level: 1 });

        let bullet = BlockType::BulletList;
        assert_eq!(bullet, BlockType::BulletList);
    }

    #[test]
    fn test_heading_levels() {
        for level in 1..=6 {
            let heading = BlockType::heading(level);
            assert!(heading.is_heading());
            assert_eq!(heading.heading_level(), Some(level));
        }
    }

    #[test]
    #[should_panic(expected = "Heading level must be between 1 and 6")]
    fn test_invalid_heading_level_zero() {
        BlockType::heading(0);
    }

    #[test]
    #[should_panic(expected = "Heading level must be between 1 and 6")]
    fn test_invalid_heading_level_seven() {
        BlockType::heading(7);
    }

    #[test]
    fn test_is_heading() {
        assert!(BlockType::heading(1).is_heading());
        assert!(!BlockType::Paragraph.is_heading());
        assert!(!BlockType::BulletList.is_heading());
    }

    #[test]
    fn test_heading_level() {
        assert_eq!(BlockType::heading(3).heading_level(), Some(3));
        assert_eq!(BlockType::Paragraph.heading_level(), None);
        assert_eq!(BlockType::BulletList.heading_level(), None);
    }

    #[test]
    fn test_default() {
        let default_block = BlockType::default();
        assert_eq!(default_block, BlockType::Paragraph);
    }

    #[test]
    fn test_clone() {
        let heading = BlockType::heading(2);
        let cloned = heading.clone();
        assert_eq!(heading, cloned);
    }

    #[test]
    fn test_equality() {
        assert_eq!(BlockType::Paragraph, BlockType::Paragraph);
        assert_eq!(BlockType::heading(1), BlockType::heading(1));
        assert_ne!(BlockType::heading(1), BlockType::heading(2));
        assert_ne!(BlockType::Paragraph, BlockType::BulletList);
    }
}
