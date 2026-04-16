use super::model::*;

pub fn serialize(doc: &Document) -> String {
    let mut out = String::new();
    for (i, block) in doc.blocks.iter().enumerate() {
        if i > 0 { out.push('\n'); }
        serialize_block(block, &mut out, "");
    }
    out
}

fn serialize_block(block: &Block, out: &mut String, prefix: &str) {
    match block {
        Block::Heading(level, spans) => {
            for _ in 0..*level { out.push('#'); }
            out.push(' ');
            serialize_spans(spans, out);
            out.push('\n');
        }
        Block::Paragraph(spans) => {
            out.push_str(prefix);
            serialize_spans(spans, out);
            out.push('\n');
        }
        Block::List(items) => {
            for (idx, item) in items.iter().enumerate() {
                if item.ordered {
                    out.push_str(&format!("{}. ", idx + 1));
                } else {
                    out.push_str("- ");
                }
                serialize_spans(&item.spans, out);
                out.push('\n');
            }
        }
        Block::BlockQuote(blocks) => {
            for block in blocks {
                serialize_block(block, out, "> ");
            }
        }
        Block::CodeBlock(cb) => {
            out.push_str("```");
            if let Some(lang) = &cb.language { out.push_str(lang); }
            out.push('\n');
            out.push_str(&cb.content);
            out.push_str("```\n");
        }
        Block::Table(table) => {
            out.push_str("| ");
            for (i, cell) in table.header.iter().enumerate() {
                if i > 0 { out.push_str(" | "); }
                serialize_spans(cell, out);
            }
            out.push_str(" |\n");
            for _ in 0..table.header.len() {
                out.push_str("| --- ");
            }
            out.push_str("|\n");
            for row in &table.rows {
                out.push_str("| ");
                for (i, cell) in row.iter().enumerate() {
                    if i > 0 { out.push_str(" | "); }
                    serialize_spans(cell, out);
                }
                out.push_str(" |\n");
            }
        }
        Block::Image(img) => {
            out.push_str(&format!("![{}]({})\n", img.alt, img.url));
        }
        Block::HorizontalRule => { out.push_str("---\n"); }
    }
}

fn serialize_spans(spans: &[InlineSpan], out: &mut String) {
    for span in spans {
        let mut text = span.text.clone();
        if span.code {
            text = format!("`{}`", text);
        } else {
            if span.bold { text = format!("**{}**", text); }
            if span.italic { text = format!("*{}*", text); }
        }
        if let Some(url) = &span.link {
            text = format!("[{}]({})", text, url);
        }
        out.push_str(&text);
    }
}
