use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd, CodeBlockKind};
use super::model::*;

pub fn parse(markdown: &str) -> Document {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown, opts);
    let events: Vec<Event> = parser.collect();
    let blocks = parse_blocks(&events, &mut 0, false);
    Document { blocks }
}

fn parse_blocks(events: &[Event], pos: &mut usize, nested: bool) -> Vec<Block> {
    let mut blocks = Vec::new();
    while *pos < events.len() {
        match &events[*pos] {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading { level, .. } => {
                        let level = *level as u8;
                        *pos += 1;
                        let spans = collect_inline_spans(events, pos);
                        blocks.push(Block::Heading(level, spans));
                    }
                    Tag::Paragraph => {
                        *pos += 1;
                        let spans = collect_inline_spans(events, pos);
                        blocks.push(Block::Paragraph(spans));
                    }
                    Tag::List(start_num) => {
                        let ordered = start_num.is_some();
                        *pos += 1;
                        let items = collect_list_items(events, pos, ordered);
                        blocks.push(Block::List(items));
                    }
                    Tag::BlockQuote(_) => {
                        *pos += 1;
                        let inner = parse_blocks(events, pos, true);
                        blocks.push(Block::BlockQuote(inner));
                    }
                    Tag::CodeBlock(kind) => {
                        let language = match kind {
                            CodeBlockKind::Fenced(lang) => {
                                let l = lang.to_string();
                                if l.is_empty() { None } else { Some(l) }
                            }
                            CodeBlockKind::Indented => None,
                        };
                        *pos += 1;
                        let content = collect_text(events, pos);
                        blocks.push(Block::CodeBlock(CodeBlock { language, content }));
                    }
                    Tag::Table(_) => {
                        *pos += 1;
                        let table = collect_table(events, pos);
                        blocks.push(Block::Table(table));
                    }
                    _ => {
                        *pos += 1;
                        skip_to_end(events, pos);
                    }
                }
            }
            Event::End(_) => {
                *pos += 1;
                if nested {
                    break;
                }
            }
            Event::Rule => {
                blocks.push(Block::HorizontalRule);
                *pos += 1;
            }
            Event::Text(t) => {
                blocks.push(Block::Paragraph(vec![InlineSpan::new_text(&t.to_string())]));
                *pos += 1;
            }
            Event::Html(html) => {
                let text = html.to_string().trim().to_string();
                if !text.is_empty() {
                    blocks.push(Block::Paragraph(vec![InlineSpan::new_text(&text)]));
                }
                *pos += 1;
            }
            _ => {
                *pos += 1;
            }
        }
    }
    blocks
}

fn skip_to_end(events: &[Event], pos: &mut usize) {
    let mut depth = 1;
    while *pos < events.len() && depth > 0 {
        match &events[*pos] {
            Event::Start(_) => depth += 1,
            Event::End(_) => depth -= 1,
            _ => {}
        }
        *pos += 1;
    }
}

fn collect_inline_spans(events: &[Event], pos: &mut usize) -> Vec<InlineSpan> {
    let mut spans: Vec<InlineSpan> = Vec::new();
    let mut bold = false;
    let mut italic = false;
    let mut link: Option<String> = None;

    while *pos < events.len() {
        match &events[*pos] {
            Event::Text(t) => {
                spans.push(InlineSpan {
                    text: t.to_string(),
                    bold, italic, code: false,
                    link: link.clone(),
                });
                *pos += 1;
            }
            Event::Code(t) => {
                spans.push(InlineSpan {
                    text: t.to_string(),
                    bold, italic, code: true,
                    link: link.clone(),
                });
                *pos += 1;
            }
            Event::SoftBreak | Event::HardBreak => {
                spans.push(InlineSpan::new_text(" "));
                *pos += 1;
            }
            Event::Start(Tag::Strong) => { bold = true; *pos += 1; }
            Event::End(TagEnd::Strong) => { bold = false; *pos += 1; }
            Event::Start(Tag::Emphasis) => { italic = true; *pos += 1; }
            Event::End(TagEnd::Emphasis) => { italic = false; *pos += 1; }
            Event::Start(Tag::Link { dest_url, .. }) => {
                link = Some(dest_url.to_string());
                *pos += 1;
            }
            Event::End(TagEnd::Link) => { link = None; *pos += 1; }
            Event::Start(Tag::Image { dest_url, .. }) => {
                let url = dest_url.to_string();
                spans.push(InlineSpan {
                    text: format!("[image: {}]", url),
                    bold: false, italic: false, code: false,
                    link: Some(url),
                });
                *pos += 1;
                while *pos < events.len() {
                    if matches!(events[*pos], Event::End(TagEnd::Image)) {
                        *pos += 1;
                        break;
                    }
                    *pos += 1;
                }
            }
            Event::InlineHtml(html) => {
                spans.push(InlineSpan::new_text(&html.to_string()));
                *pos += 1;
            }
            Event::End(_) => { *pos += 1; break; }
            _ => { *pos += 1; }
        }
    }
    merge_adjacent_spans(spans)
}

fn collect_list_items(events: &[Event], pos: &mut usize, ordered: bool) -> Vec<ListItem> {
    let mut items = Vec::new();
    while *pos < events.len() {
        match &events[*pos] {
            Event::Start(Tag::Item) => {
                *pos += 1;
                let all_spans;
                if *pos < events.len() && matches!(events[*pos], Event::Start(Tag::Paragraph)) {
                    *pos += 1;
                    all_spans = collect_inline_spans(events, pos);
                    while *pos < events.len() {
                        if matches!(events[*pos], Event::End(TagEnd::Item)) {
                            *pos += 1;
                            break;
                        }
                        *pos += 1;
                    }
                } else {
                    all_spans = collect_inline_spans(events, pos);
                }
                items.push(ListItem { ordered, spans: all_spans });
            }
            Event::End(TagEnd::List(_)) => { *pos += 1; break; }
            _ => { *pos += 1; }
        }
    }
    items
}

fn collect_table(events: &[Event], pos: &mut usize) -> Table {
    let mut header: Vec<Vec<InlineSpan>> = Vec::new();
    let mut rows: Vec<Vec<Vec<InlineSpan>>> = Vec::new();

    while *pos < events.len() {
        match &events[*pos] {
            Event::Start(Tag::TableHead) => {
                *pos += 1;
                header = collect_table_row(events, pos);
            }
            Event::Start(Tag::TableRow) => {
                *pos += 1;
                let row = collect_table_row(events, pos);
                rows.push(row);
            }
            Event::End(TagEnd::Table) => {
                *pos += 1;
                break;
            }
            _ => { *pos += 1; }
        }
    }

    Table { header, rows }
}

fn collect_table_row(events: &[Event], pos: &mut usize) -> Vec<Vec<InlineSpan>> {
    let mut cells = Vec::new();

    while *pos < events.len() {
        match &events[*pos] {
            Event::Start(Tag::TableCell) => {
                *pos += 1;
                let spans = collect_inline_spans(events, pos);
                cells.push(spans);
            }
            Event::End(TagEnd::TableHead) | Event::End(TagEnd::TableRow) => {
                *pos += 1;
                break;
            }
            _ => { *pos += 1; }
        }
    }

    cells
}

fn collect_text(events: &[Event], pos: &mut usize) -> String {
    let mut text = String::new();
    while *pos < events.len() {
        match &events[*pos] {
            Event::Text(t) => { text.push_str(t); *pos += 1; }
            Event::End(_) => { *pos += 1; break; }
            _ => { *pos += 1; }
        }
    }
    text
}

fn merge_adjacent_spans(spans: Vec<InlineSpan>) -> Vec<InlineSpan> {
    let mut merged: Vec<InlineSpan> = Vec::new();
    for span in spans {
        if let Some(last) = merged.last_mut() {
            if last.bold == span.bold && last.italic == span.italic
                && last.code == span.code && last.link == span.link
            {
                last.text.push_str(&span.text);
                continue;
            }
        }
        merged.push(span);
    }
    merged
}
