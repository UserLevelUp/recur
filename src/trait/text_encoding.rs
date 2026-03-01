//! Text encoding helpers for UTF-8 normalization.
//!
//! Centralizes BOM stripping so command implementations do not duplicate
//! encoding normalization logic.

/// Strip a UTF-8 BOM prefix (U+FEFF) when present.
///
/// Returns the original slice when no BOM is present.
pub fn strip_utf8_bom(content: &str) -> &str {
    content.strip_prefix('\u{FEFF}').unwrap_or(content)
}

/// Trait for commands that need UTF-8 BOM normalization.
pub trait Utf8BomCapable {
    /// Normalize UTF-8 text by removing a leading BOM if present.
    fn normalize_utf8_bom(content: &str) -> &str {
        strip_utf8_bom(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCommand;
    impl Utf8BomCapable for TestCommand {}

    #[test]
    fn strip_utf8_bom_removes_prefix_when_present() {
        let content = "\u{FEFF}{\"ok\":true}";
        assert_eq!(strip_utf8_bom(content), "{\"ok\":true}");
    }

    #[test]
    fn strip_utf8_bom_keeps_content_when_absent() {
        let content = "{\"ok\":true}";
        assert_eq!(strip_utf8_bom(content), "{\"ok\":true}");
    }

    #[test]
    fn trait_normalize_utf8_bom_uses_shared_logic() {
        let content = "\u{FEFF}abc";
        assert_eq!(TestCommand::normalize_utf8_bom(content), "abc");
    }
}

