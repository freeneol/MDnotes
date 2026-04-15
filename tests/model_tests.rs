use mdnotes::markdown::model::*;

#[test]
fn test_document_default_is_empty() {
    let doc = Document::default();
    assert!(doc.blocks.is_empty());
}

#[test]
fn test_inline_span_plain_text() {
    let span = InlineSpan::new_text("hello");
    assert_eq!(span.text, "hello");
    assert!(!span.bold);
    assert!(!span.italic);
    assert!(!span.code);
    assert!(span.link.is_none());
}

#[test]
fn test_inline_span_bold() {
    let span = InlineSpan::new_text("bold").with_bold();
    assert!(span.bold);
}

#[test]
fn test_block_paragraph() {
    let block = Block::Paragraph(vec![InlineSpan::new_text("hello world")]);
    match &block {
        Block::Paragraph(spans) => assert_eq!(spans.len(), 1),
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn test_heading_levels() {
    for level in 1..=6u8 {
        let block = Block::Heading(level, vec![InlineSpan::new_text("title")]);
        match &block {
            Block::Heading(l, _) => assert_eq!(*l, level),
            _ => panic!("expected heading"),
        }
    }
}

#[test]
fn test_document_word_count() {
    let doc = Document {
        blocks: vec![
            Block::Paragraph(vec![InlineSpan::new_text("hello world")]),
            Block::Paragraph(vec![InlineSpan::new_text("foo bar baz")]),
        ],
    };
    assert_eq!(doc.word_count(), 5);
}
