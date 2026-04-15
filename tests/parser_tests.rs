use mdnotes::markdown::model::*;
use mdnotes::markdown::parser::parse;

#[test]
fn test_parse_heading() {
    let doc = parse("# Hello");
    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Heading(level, spans) => {
            assert_eq!(*level, 1);
            assert_eq!(spans[0].text, "Hello");
        }
        _ => panic!("expected heading"),
    }
}

#[test]
fn test_parse_paragraph() {
    let doc = parse("Hello world");
    assert_eq!(doc.blocks.len(), 1);
    match &doc.blocks[0] {
        Block::Paragraph(spans) => {
            assert_eq!(spans[0].text, "Hello world");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn test_parse_bold() {
    let doc = parse("**bold text**");
    match &doc.blocks[0] {
        Block::Paragraph(spans) => {
            assert!(spans[0].bold);
            assert_eq!(spans[0].text, "bold text");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn test_parse_italic() {
    let doc = parse("*italic text*");
    match &doc.blocks[0] {
        Block::Paragraph(spans) => {
            assert!(spans[0].italic);
            assert_eq!(spans[0].text, "italic text");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn test_parse_inline_code() {
    let doc = parse("`code`");
    match &doc.blocks[0] {
        Block::Paragraph(spans) => {
            assert!(spans[0].code);
            assert_eq!(spans[0].text, "code");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn test_parse_link() {
    let doc = parse("[click](https://example.com)");
    match &doc.blocks[0] {
        Block::Paragraph(spans) => {
            assert_eq!(spans[0].text, "click");
            assert_eq!(spans[0].link.as_deref(), Some("https://example.com"));
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn test_parse_unordered_list() {
    let doc = parse("- item1\n- item2");
    match &doc.blocks[0] {
        Block::List(items) => {
            assert_eq!(items.len(), 2);
            assert!(!items[0].ordered);
            assert_eq!(items[0].spans[0].text, "item1");
        }
        _ => panic!("expected list"),
    }
}

#[test]
fn test_parse_code_block() {
    let doc = parse("```rust\nfn main() {}\n```");
    match &doc.blocks[0] {
        Block::CodeBlock(cb) => {
            assert_eq!(cb.language.as_deref(), Some("rust"));
            assert_eq!(cb.content, "fn main() {}\n");
        }
        _ => panic!("expected code block"),
    }
}

#[test]
fn test_parse_blockquote() {
    let doc = parse("> quoted text");
    match &doc.blocks[0] {
        Block::BlockQuote(blocks) => {
            assert_eq!(blocks.len(), 1);
            match &blocks[0] {
                Block::Paragraph(spans) => assert_eq!(spans[0].text, "quoted text"),
                _ => panic!("expected paragraph inside quote"),
            }
        }
        _ => panic!("expected blockquote"),
    }
}

#[test]
fn test_parse_horizontal_rule() {
    let doc = parse("---");
    assert_eq!(doc.blocks.len(), 1);
    assert!(matches!(doc.blocks[0], Block::HorizontalRule));
}

#[test]
fn test_parse_mixed_document() {
    let md = "# Title\n\nSome **bold** and *italic* text.\n\n- item1\n- item2\n\n> quote\n\n---";
    let doc = parse(md);
    assert!(doc.blocks.len() >= 5);
}
