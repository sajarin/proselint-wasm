//! Position tracking module for proselint-wasm
//!
//! Handles conversion between byte offsets and line/column positions.

/// Tracks line boundaries for efficient position lookups
pub struct LineTracker {
    /// Cumulative byte offsets at the end of each line
    line_ends: Vec<usize>,
}

impl LineTracker {
    /// Create a new LineTracker from text
    pub fn new(text: &str) -> Self {
        let mut line_ends = Vec::new();
        let mut offset = 0;

        for line in text.split('\n') {
            offset += line.len() + 1; // +1 for newline
            line_ends.push(offset);
        }

        // Adjust last line (no trailing newline)
        if !text.ends_with('\n') && !line_ends.is_empty() {
            let last = line_ends.len() - 1;
            line_ends[last] -= 1;
        }

        Self { line_ends }
    }

    /// Convert a byte offset to (line, column), both 1-indexed
    pub fn offset_to_position(&self, offset: usize) -> (usize, usize) {
        // Binary search to find the line
        // If offset is exactly at a line boundary, it belongs to the next line
        let line = match self.line_ends.binary_search(&offset) {
            Ok(idx) => idx + 1, // Exact match means start of next line
            Err(idx) => idx,
        };

        let line_start = if line == 0 {
            0
        } else {
            self.line_ends[line - 1]
        };

        let column = offset - line_start;

        // Return 1-indexed values
        (line + 1, column + 1)
    }

    /// Get the total number of lines
    pub fn line_count(&self) -> usize {
        self.line_ends.len()
    }
}

/// Quote span tracker for filtering matches inside quotes
pub struct QuoteTracker {
    /// List of (start, end) byte offsets for quoted spans
    spans: Vec<(usize, usize)>,
}

impl QuoteTracker {
    /// Create a new QuoteTracker from text
    pub fn new(text: &str) -> Self {
        let mut spans = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let bytes: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();

        // Track different quote types
        let quote_pairs = [
            ('"', '"'),      // Straight double quotes
            ('\'', '\''),    // Straight single quotes
            ('\u{201C}', '\u{201D}'), // Curly double quotes
            ('\u{2018}', '\u{2019}'), // Curly single quotes
        ];

        for (open, close) in quote_pairs {
            let mut i = 0;
            while i < chars.len() {
                if chars[i] == open {
                    let start = bytes[i];
                    i += 1;
                    // Find matching close quote
                    while i < chars.len() && chars[i] != close {
                        i += 1;
                    }
                    if i < chars.len() {
                        let end = bytes[i] + chars[i].len_utf8();
                        spans.push((start, end));
                    }
                }
                i += 1;
            }
        }

        // Sort spans by start position
        spans.sort_by_key(|s| s.0);

        Self { spans }
    }

    /// Check if a byte offset is inside a quoted span
    pub fn is_in_quote(&self, offset: usize) -> bool {
        // Binary search to find if offset is in any span
        self.spans.iter().any(|&(start, end)| offset >= start && offset < end)
    }

    /// Check if a range overlaps with any quoted span
    pub fn overlaps_quote(&self, start: usize, end: usize) -> bool {
        self.spans.iter().any(|&(q_start, q_end)| {
            // Overlap if neither is completely before or after the other
            !(end <= q_start || start >= q_end)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_tracker() {
        let text = "Hello\nWorld\nTest";
        let tracker = LineTracker::new(text);

        assert_eq!(tracker.offset_to_position(0), (1, 1)); // H
        assert_eq!(tracker.offset_to_position(5), (1, 6)); // \n
        assert_eq!(tracker.offset_to_position(6), (2, 1)); // W
        assert_eq!(tracker.offset_to_position(12), (3, 1)); // T
    }

    #[test]
    fn test_quote_tracker() {
        let text = r#"Hello "world" there"#;
        let tracker = QuoteTracker::new(text);

        assert!(!tracker.is_in_quote(0)); // H
        assert!(tracker.is_in_quote(7));  // w in "world"
        assert!(!tracker.is_in_quote(14)); // t in there
    }
}
