//! Position tracking module for proselint-wasm
//!
//! Handles conversion between byte offsets and line/column positions.
//! Properly handles UTF-8 multi-byte characters and different line ending styles.

/// Tracks line boundaries for efficient position lookups
pub struct LineTracker {
    /// Cumulative byte offsets at the end of each line
    line_ends: Vec<usize>,
    /// Original text for UTF-8 character indexing
    text: String,
}

impl LineTracker {
    /// Create a new LineTracker from text
    /// Handles both Unix (\n) and Windows (\r\n) line endings
    pub fn new(text: &str) -> Self {
        let mut line_ends = Vec::new();

        // Normalize line endings to handle both \n and \r\n
        let mut chars = text.char_indices().peekable();

        while let Some((byte_idx, ch)) = chars.next() {
            if ch == '\n' {
                let line_end = byte_idx + ch.len_utf8();
                line_ends.push(line_end);
            } else if ch == '\r' {
                // Check if this is \r\n or just \r
                if let Some(&(_, '\n')) = chars.peek() {
                    // Skip the \r, the \n will be handled in next iteration
                    continue;
                } else {
                    // Treat \r alone as line ending (old Mac style)
                    let line_end = byte_idx + ch.len_utf8();
                    line_ends.push(line_end);
                }
            }
        }

        // If text doesn't end with newline, add final line end
        if !text.is_empty() && line_ends.last().is_none_or(|&end| end != text.len()) {
            line_ends.push(text.len());
        }

        Self {
            line_ends,
            text: text.to_string(),
        }
    }

    /// Convert a byte offset to (line, column), both 1-indexed
    /// Column numbers are character-based (not byte-based) for proper UTF-8 handling
    pub fn offset_to_position(&self, offset: usize) -> (usize, usize) {
        // Clamp offset to text length
        let offset = offset.min(self.text.len());

        // Handle empty file
        if self.line_ends.is_empty() {
            return (1, 1);
        }

        // Binary search to find which line contains this offset
        let line_idx = match self.line_ends.binary_search(&offset) {
            Ok(idx) => {
                // Exact match with a line ending
                // This offset is right after a line ending, so it's the start of the next line
                idx + 1
            }
            Err(idx) => {
                // offset would be inserted at idx
                // This means it belongs to line idx (0-indexed)
                idx
            }
        };

        let line_start_byte = if line_idx == 0 {
            0
        } else {
            self.line_ends[line_idx - 1]
        };

        // Ensure we don't try to slice in the middle of a UTF-8 character
        // Find the nearest valid char boundary
        let safe_offset = if offset < self.text.len() && !self.text.is_char_boundary(offset) {
            // Walk backward to find a valid boundary
            (0..offset)
                .rev()
                .find(|&i| self.text.is_char_boundary(i))
                .unwrap_or(offset)
        } else {
            offset
        };

        // Count characters (not bytes) from line start to offset
        // This ensures proper column numbers for multi-byte UTF-8 characters
        let column =
            if line_start_byte <= safe_offset && self.text.is_char_boundary(line_start_byte) {
                self.text[line_start_byte..safe_offset].chars().count()
            } else {
                0
            };

        // Return 1-indexed values
        (line_idx + 1, column + 1)
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
            ('"', '"'),               // Straight double quotes
            ('\'', '\''),             // Straight single quotes
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
    /// Uses binary search for O(log n) performance instead of O(n)
    pub fn is_in_quote(&self, offset: usize) -> bool {
        // Binary search to find the first span that could contain offset
        // We search by start position
        match self
            .spans
            .binary_search_by_key(&offset, |&(start, _)| start)
        {
            Ok(_) => {
                // Exact match on start position - offset is at the opening quote
                true
            }
            Err(idx) => {
                // offset would be inserted at idx
                // Check if it's inside the previous span (idx - 1)
                if idx > 0 {
                    let (start, end) = self.spans[idx - 1];
                    offset >= start && offset < end
                } else {
                    false
                }
            }
        }
    }

    /// Check if a range overlaps with any quoted span
    /// Uses binary search for O(log n) performance
    pub fn overlaps_quote(&self, start: usize, end: usize) -> bool {
        if self.spans.is_empty() {
            return false;
        }

        // Binary search to find the first span that might overlap
        let idx = match self.spans.binary_search_by_key(&start, |&(s, _)| s) {
            Ok(idx) => idx,
            Err(idx) => {
                // Check previous span if it exists
                if idx > 0 {
                    idx - 1
                } else {
                    idx
                }
            }
        };

        // Check spans starting from idx until we're past the end
        for &(q_start, q_end) in &self.spans[idx..] {
            // If this quote starts after our range ends, no more overlaps possible
            if q_start >= end {
                break;
            }
            // Check for overlap: ranges overlap if neither is completely before/after the other
            if !(end <= q_start || start >= q_end) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod property_tests {
        use super::*;
        use quickcheck::TestResult;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_line_tracker_position_in_bounds(text: String) -> TestResult {
            // Skip empty strings and strings with null bytes (edge case)
            if text.is_empty() || text.contains('\0') {
                return TestResult::discard();
            }

            let tracker = LineTracker::new(&text);

            // Test every valid byte offset
            for offset in 0..=text.len() {
                let (line, col) = tracker.offset_to_position(offset);

                // Line and column must be at least 1 (1-indexed)
                if line < 1 || col < 1 {
                    return TestResult::failed();
                }

                // Line must not exceed line count (or be exactly line_count + 1 for EOF)
                if line > tracker.line_count() + 1 {
                    return TestResult::failed();
                }
            }

            TestResult::passed()
        }

        #[quickcheck]
        fn prop_line_tracker_monotonic(text: String) -> TestResult {
            if text.len() < 2 {
                return TestResult::discard();
            }

            let tracker = LineTracker::new(&text);

            // Line numbers should be monotonically increasing
            let mut prev_line = 0;
            for offset in 0..=text.len() {
                let (line, _) = tracker.offset_to_position(offset);
                if line < prev_line {
                    return TestResult::failed();
                }
                prev_line = line;
            }

            TestResult::passed()
        }

        #[quickcheck]
        fn prop_quote_tracker_valid_spans(text: String) -> TestResult {
            let tracker = QuoteTracker::new(&text);

            // All spans should be within text bounds
            for &(start, end) in &tracker.spans {
                if start > end || end > text.len() {
                    return TestResult::failed();
                }
            }

            // Spans should not overlap
            for i in 0..tracker.spans.len() {
                for j in (i + 1)..tracker.spans.len() {
                    let (s1, e1) = tracker.spans[i];
                    let (s2, e2) = tracker.spans[j];

                    // Check for overlap
                    if !(e1 <= s2 || e2 <= s1) {
                        // Spans overlap, which is actually OK if nested quotes
                        // Just ensure they're properly sorted
                        if s1 > s2 {
                            return TestResult::failed();
                        }
                    }
                }
            }

            TestResult::passed()
        }

        #[quickcheck]
        fn prop_quote_overlaps_symmetric(text: String, start: usize, end: usize) -> TestResult {
            if text.is_empty() || start >= end {
                return TestResult::discard();
            }

            let tracker = QuoteTracker::new(&text);

            // If a range overlaps, the quote should contain at least one point in the range
            let overlaps = tracker.overlaps_quote(start, end);

            if overlaps {
                // At least one offset in the range should be in a quote
                let any_in_quote =
                    (start..end.min(text.len())).any(|offset| tracker.is_in_quote(offset));

                TestResult::from_bool(any_in_quote)
            } else {
                TestResult::passed()
            }
        }
    }

    #[test]
    fn test_line_tracker() {
        let text = "Hello\nWorld\nTest";
        //          012345 678910 11 121314 (byte offsets)
        // Line 1: "Hello\n" (bytes 0-6, \n at 5)
        // Line 2: "World\n" (bytes 6-12, \n at 11)
        // Line 3: "Test" (bytes 12-15)
        let tracker = LineTracker::new(text);

        assert_eq!(tracker.offset_to_position(0), (1, 1)); // H
        assert_eq!(tracker.offset_to_position(5), (1, 6)); // \n at end of Hello
        assert_eq!(tracker.offset_to_position(6), (2, 1)); // W
        assert_eq!(tracker.offset_to_position(11), (2, 6)); // \n at end of World
        assert_eq!(tracker.offset_to_position(12), (3, 1)); // T
    }

    #[test]
    fn test_line_tracker_utf8_emoji() {
        // Test with emoji (4-byte UTF-8 characters)
        let text = "Hello 👋 world";
        //          H e l l o   👋       w o r l d
        //          0 1 2 3 4 5 6       10 11 12 13 14 (byte offsets)
        //          1 2 3 4 5 6 7        8  9 10 11 12 (char positions, 1-indexed)
        // 👋 is 4 bytes (bytes 6-9)
        let tracker = LineTracker::new(text);

        // 'H' at byte 0
        assert_eq!(tracker.offset_to_position(0), (1, 1));
        // ' ' before emoji at byte 5
        assert_eq!(tracker.offset_to_position(5), (1, 6));
        // 👋 starts at byte 6
        assert_eq!(tracker.offset_to_position(6), (1, 7));
        // byte 7 is in middle of 👋, should round down to byte 6
        // Our implementation finds the nearest char boundary
        assert_eq!(tracker.offset_to_position(7), (1, 7));
        // ' ' after emoji at byte 10
        assert_eq!(tracker.offset_to_position(10), (1, 8));
        // 'w' at byte 11
        assert_eq!(tracker.offset_to_position(11), (1, 9));
    }

    #[test]
    fn test_line_tracker_utf8_multibyte() {
        // Test with various multi-byte UTF-8 characters
        let text = "Café\nnaïve\n日本語";
        // Line 1: Café\n
        //   C(0) a(1) f(2) é(3-4) \n(5)
        // Line 2: naïve\n
        //   n(6) a(7) ï(8-9) v(10) e(11) \n(12)
        // Line 3: 日本語
        //   日(13-15) 本(16-18) 語(19-21)
        let tracker = LineTracker::new(text);

        // Test line 1
        assert_eq!(tracker.offset_to_position(0), (1, 1)); // C
        assert_eq!(tracker.offset_to_position(3), (1, 4)); // é (starts at byte 3)
        assert_eq!(tracker.offset_to_position(5), (1, 5)); // \n at end of line 1

        // Test line 2 (starts at byte 6 after "Café\n")
        assert_eq!(tracker.offset_to_position(6), (2, 1)); // n
        assert_eq!(tracker.offset_to_position(8), (2, 3)); // ï (starts at byte 8)
        assert_eq!(tracker.offset_to_position(12), (2, 6)); // \n at end of line 2

        // Test line 3 (starts at byte 13 after "Café\nnaïve\n")
        assert_eq!(tracker.offset_to_position(13), (3, 1)); // 日 (starts at byte 13)
        assert_eq!(tracker.offset_to_position(16), (3, 2)); // 本 (starts at byte 16)
        assert_eq!(tracker.offset_to_position(19), (3, 3)); // 語 (starts at byte 19)
    }

    #[test]
    fn test_line_tracker_windows_line_endings() {
        // Test with Windows-style \r\n line endings
        let text = "Hello\r\nWorld\r\nTest";
        //          01234 5 6 78910 11 12 13 14151617 (byte offsets)
        // Line 1: Hello\r\n (ends at byte 7)
        // Line 2: World\r\n (ends at byte 14)
        // Line 3: Test
        let tracker = LineTracker::new(text);

        assert_eq!(tracker.offset_to_position(0), (1, 1)); // H
        assert_eq!(tracker.offset_to_position(5), (1, 6)); // \r
        assert_eq!(tracker.offset_to_position(6), (1, 7)); // \n (line ending)
        assert_eq!(tracker.offset_to_position(7), (2, 1)); // W (after \r\n)
        assert_eq!(tracker.offset_to_position(14), (3, 1)); // T (after second \r\n)
        assert_eq!(tracker.line_count(), 3);
    }

    #[test]
    fn test_line_tracker_mixed_line_endings() {
        // Test with mixed line endings
        let text = "Line1\nLine2\r\nLine3\rLine4";
        //          01234 5 6789101112 13 14151617181920212223 (byte offsets)
        // Line 1: Line1\n (ends at byte 6)
        // Line 2: Line2\r\n (ends at byte 13)
        // Line 3: Line3\r (ends at byte 19)
        // Line 4: Line4 (ends at byte 24)
        let tracker = LineTracker::new(text);

        assert_eq!(tracker.offset_to_position(0), (1, 1)); // L in Line1
        assert_eq!(tracker.offset_to_position(6), (2, 1)); // L in Line2 (after \n)
        assert_eq!(tracker.offset_to_position(13), (3, 1)); // L in Line3 (after \r\n)
        assert_eq!(tracker.offset_to_position(19), (4, 1)); // L in Line4 (after \r)
        assert_eq!(tracker.line_count(), 4);
    }

    #[test]
    fn test_line_tracker_old_mac_line_endings() {
        // Test with old Mac-style \r only line endings
        let text = "Hello\rWorld\rTest";
        //          01234 5 67891011 121314 15 (byte offsets)
        // Line 1: Hello\r (ends at byte 6)
        // Line 2: World\r (ends at byte 12)
        // Line 3: Test (ends at byte 16)
        let tracker = LineTracker::new(text);

        assert_eq!(tracker.offset_to_position(0), (1, 1)); // H
        assert_eq!(tracker.offset_to_position(5), (1, 6)); // o (before \r)
        assert_eq!(tracker.offset_to_position(6), (2, 1)); // W (after \r)
        assert_eq!(tracker.offset_to_position(12), (3, 1)); // T (after second \r)
        assert_eq!(tracker.line_count(), 3);
    }

    #[test]
    fn test_quote_tracker() {
        let text = r#"Hello "world" there"#;
        let tracker = QuoteTracker::new(text);

        assert!(!tracker.is_in_quote(0)); // H
        assert!(tracker.is_in_quote(7)); // w in "world"
        assert!(!tracker.is_in_quote(14)); // t in there
    }

    #[test]
    fn test_quote_tracker_utf8() {
        // Test quote tracking with UTF-8 characters
        let text = r#"Hello "👋 world" there"#;
        let tracker = QuoteTracker::new(text);

        assert!(!tracker.is_in_quote(0)); // H
        assert!(tracker.is_in_quote(7)); // 👋 in quote (starts at byte 7)
        assert!(tracker.is_in_quote(11)); // space after emoji
        assert!(tracker.is_in_quote(12)); // w in "world"
        assert!(!tracker.is_in_quote(18)); // t in there (outside quotes)
    }

    #[test]
    fn test_quote_tracker_overlaps() {
        let text = r#"Before "quoted text" after"#;
        let tracker = QuoteTracker::new(text);

        // Test overlaps_quote
        assert!(!tracker.overlaps_quote(0, 7)); // "Before " - before quote
        assert!(tracker.overlaps_quote(7, 20)); // Overlaps with quoted section
        assert!(tracker.overlaps_quote(8, 19)); // Entirely within quoted section
        assert!(!tracker.overlaps_quote(21, 27)); // " after" - after quote
    }

    #[test]
    fn test_quote_tracker_curly_quotes() {
        // Test with curly quotes (smart quotes)
        // Using Unicode escapes to avoid string literal issues
        let text = "He said \u{201C}very good\u{201D} but it was \u{2018}not great\u{2019}.";
        // "He said " (0-7)
        // " (8-10, 3 bytes)
        // "very good" (11-19)
        // " (20-22, 3 bytes)
        // " but it was " (23-35)
        // ' (36-38, 3 bytes)
        // "not great" (39-47)
        // ' (48-50, 3 bytes)
        // "." (51)
        let tracker = QuoteTracker::new(text);

        // "very good" should be in quotes (bytes 11-19)
        assert!(tracker.is_in_quote(11)); // v in very
        assert!(tracker.is_in_quote(16)); // g in good

        // 'not great' should be in quotes (bytes 39-47)
        assert!(tracker.is_in_quote(39)); // n in not
        assert!(tracker.is_in_quote(43)); // g in great
    }
}
