use mdnotes::markdown::model::*;
use mdnotes::markdown::serializer::serialize;

#[test]
fn test_serialize_heading() {
    let doc = Document {
        blocks: vec![Block::Heading(1, vec![InlineSpan::new_text("Title")])],
    };
    assert_eq!(serialize(&doc), "# Title\n");
}

#[test]
fn test_serialize_paragraph() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![InlineSpan::new_text("Hello world")])],
    };
    assert_eq!(serialize(&doc), "Hello world\n");
}

#[test]
fn test_serialize_bold() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![InlineSpan::new_text("bold").with_bold()])],
    };
    assert_eq!(serialize(&doc), "**bold**\n");
}

#[test]
fn test_serialize_italic() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![InlineSpan::new_text("italic").with_italic()])],
    };
    assert_eq!(serialize(&doc), "*italic*\n");
}

#[test]
fn test_serialize_inline_code() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![InlineSpan::new_text("code").with_code()])],
    };
    assert_eq!(serialize(&doc), "`code`\n");
}

#[test]
fn test_serialize_link() {
    let doc = Document {
        blocks: vec![Block::Paragraph(vec![
            InlineSpan::new_text("click").with_link("https://example.com"),
        ])],
    };
    assert_eq!(serialize(&doc), "[click](https://example.com)\n");
}

#[test]
fn test_serialize_unordered_list() {
    let doc = Document {
        blocks: vec![Block::List(vec![
            ListItem { ordered: false, spans: vec![InlineSpan::new_text("a")] },
            ListItem { ordered: false, spans: vec![InlineSpan::new_text("b")] },
        ])],
    };
    assert_eq!(serialize(&doc), "- a\n- b\n");
}

#[test]
fn test_serialize_code_block() {
    let doc = Document {
        blocks: vec![Block::CodeBlock(CodeBlock {
            language: Some("rust".to_string()),
            content: "fn main() {}\n".to_string(),
        })],
    };
    assert_eq!(serialize(&doc), "```rust\nfn main() {}\n```\n");
}

#[test]
fn test_serialize_blockquote() {
    let doc = Document {
        blocks: vec![Block::BlockQuote(vec![
            Block::Paragraph(vec![InlineSpan::new_text("quoted")]),
        ])],
    };
    assert_eq!(serialize(&doc), "> quoted\n");
}

#[test]
fn test_serialize_horizontal_rule() {
    let doc = Document { blocks: vec![Block::HorizontalRule] };
    assert_eq!(serialize(&doc), "---\n");
}

#[test]
fn test_roundtrip_basic() {
    let original = "# Title\n\nHello **bold** and *italic* text.\n\n- item1\n- item2\n";
    let doc = mdnotes::markdown::parser::parse(original);
    let output = serialize(&doc);
    let doc2 = mdnotes::markdown::parser::parse(&output);
    assert_eq!(doc, doc2);
}
