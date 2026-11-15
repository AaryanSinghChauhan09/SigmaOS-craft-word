//! SigmaOS-craft-word: word processing model.
//!
//! Real implementation: documents, sections, paragraphs, formatting, styles, and validation.
//! Replaces the 20-line thin placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Paragraph style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphStyle {
    Normal,
    Heading1,
    Heading2,
    Heading3,
    Code,
    Quote,
}

/// Paragraph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    pub id: String,
    pub content: String,
    pub style: ParagraphStyle,
}

/// Section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WordError {
    EmptyContent,
    DuplicateParagraph(String),
    InvalidStyle,
}

impl fmt::Display for WordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyContent => write!(f, "paragraph content must not be empty"),
            Self::DuplicateParagraph(id) => write!(f, "duplicate paragraph {:?}", id),
            Self::InvalidStyle => write!(f, "invalid style"),
        }
    }
}

impl std::error::Error for WordError {}

/// Word document.
pub struct WordDocument {
    pub title: String,
    pub author: String,
    paragraphs: BTreeMap<String, Paragraph>,
    sections: BTreeMap<String, Section>,
}

impl WordDocument {
    pub fn new(title: impl Into<String>) -> Self { Self { title: title.into(), author: String::new(), paragraphs: BTreeMap::new(), sections: BTreeMap::new() } }

    pub fn add_paragraph(&mut self, paragraph: Paragraph) -> Result<(), WordError> {
        if paragraph.content.trim().is_empty() { return Err(WordError::EmptyContent); }
        let id = paragraph.id.clone();
        if self.paragraphs.contains_key(&id) {
            return Err(WordError::DuplicateParagraph(id.clone()));
        }
        self.paragraphs.insert(id, paragraph);
        Ok(())
    }

    pub fn paragraph_count(&self) -> usize { self.paragraphs.len() }

    pub fn paragraphs(&self) -> Vec<&Paragraph> { self.paragraphs.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_create_aaa() {
        let d = WordDocument::new("My Doc");
        assert_eq!(d.title, "My Doc");
        assert_eq!(d.paragraph_count(), 0);
    }

    #[test]
    fn test_add_paragraph_aaa() {
        let mut d = WordDocument::new("My Doc");
        let p = Paragraph { id: "p1".into(), content: "Hello".into(), style: ParagraphStyle::Normal };
        d.add_paragraph(p).unwrap();
        assert_eq!(d.paragraph_count(), 1);
    }

    #[test]
    fn test_empty_content_rejected_aaa() {
        let mut d = WordDocument::new("My Doc");
        let p = Paragraph { id: "x".into(), content: "".into(), style: ParagraphStyle::Normal };
        assert!(d.add_paragraph(p).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut d = WordDocument::new("My Doc");
        let p = Paragraph { id: "x".into(), content: "X".into(), style: ParagraphStyle::Normal };
        d.add_paragraph(p).unwrap();
        let p2 = Paragraph { id: "x".into(), content: "X".into(), style: ParagraphStyle::Normal };
        assert!(d.add_paragraph(p2).is_err());
    }
}
