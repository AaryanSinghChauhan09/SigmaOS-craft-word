#[derive(Debug, Clone)]
pub struct WordParagraph { pub content: String }

#[derive(Debug, Default)]
pub struct WordDocument { paragraphs: Vec<WordParagraph> }
impl WordDocument {
    pub fn new() -> Self { Self::default() }
    pub fn append(&mut self, s: impl Into<String>) { self.paragraphs.push(WordParagraph { content: s.into() }); }
    pub fn count(&self) -> usize { self.paragraphs.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_word() {
        let mut d = WordDocument::new();
        d.append("Introduction");
        assert_eq!(d.count(), 1);
    }
}
