use serde::{Deserialize, Serialize};

/// Represents inline formatting that can be applied to text ranges
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InlineFormat {
    /// Bold text formatting
    Bold,
    /// Italic text formatting
    Italic,
    /// Underline text formatting
    Underline,
    /// Strikethrough text formatting
    Strikethrough,
    /// Inline code formatting
    Code,
    /// Hyperlink with URL
    Link { url: String },
    /// Text color with color value (e.g., "#FF0000" or "red")
    TextColor { color: String },
    /// Background color with color value (e.g., "#FFFF00" or "yellow")
    BackgroundColor { color: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_format_equality() {
        assert_eq!(InlineFormat::Bold, InlineFormat::Bold);
        assert_ne!(InlineFormat::Bold, InlineFormat::Italic);
    }

    #[test]
    fn test_inline_format_link() {
        let link1 = InlineFormat::Link {
            url: "https://example.com".to_string(),
        };
        let link2 = InlineFormat::Link {
            url: "https://example.com".to_string(),
        };
        let link3 = InlineFormat::Link {
            url: "https://other.com".to_string(),
        };

        assert_eq!(link1, link2);
        assert_ne!(link1, link3);
    }

    #[test]
    fn test_inline_format_colors() {
        let text_color = InlineFormat::TextColor {
            color: "#FF0000".to_string(),
        };
        let bg_color = InlineFormat::BackgroundColor {
            color: "#FFFF00".to_string(),
        };

        assert_ne!(text_color, bg_color);
    }

    #[test]
    fn test_inline_format_clone() {
        let format = InlineFormat::Bold;
        let cloned = format.clone();
        assert_eq!(format, cloned);
    }

    #[test]
    fn test_inline_format_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(InlineFormat::Bold);
        set.insert(InlineFormat::Italic);
        set.insert(InlineFormat::Bold); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&InlineFormat::Bold));
        assert!(set.contains(&InlineFormat::Italic));
    }
}
