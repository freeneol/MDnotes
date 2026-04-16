use iced::widget::{
    container, horizontal_rule, rich_text, scrollable, span, text, text_editor, Column, Row,
};
use iced::font::{Family, Weight};
use iced::{Element, Font, Length, Padding};

use crate::app::{Message, Mode};
use crate::markdown::model::*;

#[cfg(target_os = "macos")]
const CJK_FONT: &str = "PingFang SC";

#[cfg(target_os = "windows")]
const CJK_FONT: &str = "Microsoft YaHei";

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const CJK_FONT: &str = "Noto Sans CJK SC";

fn cjk_font(weight: Weight, style: iced::font::Style) -> Font {
    Font {
        family: Family::Name(CJK_FONT),
        weight,
        style,
        ..Font::DEFAULT
    }
}

pub fn view<'a>(
    document: &Document,
    mode: &Mode,
    _content_text: &'a str,
    editor_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    match mode {
        Mode::Preview => view_reading(document),
        Mode::Markdown => view_editing(editor_content),
    }
}

fn view_reading(document: &Document) -> Element<'static, Message> {
    let mut col: Column<'static, Message> = Column::new().spacing(8).padding(Padding::from([16, 20]));

    for block in &document.blocks {
        col = col.push(render_block(block));
    }

    container(scrollable(col).width(Length::Fill).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn view_editing<'a>(content: &'a text_editor::Content) -> Element<'a, Message> {
    container(
        text_editor(content)
            .on_action(|action| Message::EditorAction(action))
            .padding(Padding::from([16, 20]))
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn render_block(block: &Block) -> Element<'static, Message> {
    match block {
        Block::Heading(level, spans) => {
            let size = match level {
                1 => 28.0,
                2 => 24.0,
                3 => 20.0,
                4 => 18.0,
                5 => 16.0,
                _ => 14.0,
            };
            let iced_spans: Vec<_> = spans.iter().map(|s| to_iced_span(s, size)).collect();
            let el: Element<'static, ()> = rich_text(iced_spans).size(size).into();
            el.map(|_| Message::Noop)
        }
        Block::Paragraph(spans) => {
            let iced_spans: Vec<_> = spans.iter().map(|s| to_iced_span(s, 15.0)).collect();
            let el: Element<'static, ()> = rich_text(iced_spans).size(15.0).into();
            el.map(|_| Message::Noop)
        }
        Block::List(items) => {
            let mut col: Column<'static, Message> = Column::new().spacing(2);
            for (i, item) in items.iter().enumerate() {
                let prefix = if item.ordered {
                    format!("{}. ", i + 1)
                } else {
                    "• ".to_string()
                };
                let mut iced_spans = vec![
                    span(prefix).size(15.0).font(cjk_font(Weight::Normal, iced::font::Style::Normal))
                ];
                iced_spans.extend(item.spans.iter().map(|s| to_iced_span(s, 15.0)));
                let el: Element<'static, ()> = rich_text(iced_spans).size(15.0).into();
                col = col.push(el.map(|_| Message::Noop));
            }
            col.into()
        }
        Block::BlockQuote(blocks) => {
            let mut col: Column<'static, Message> = Column::new().spacing(4);
            for b in blocks {
                col = col.push(render_block(b));
            }
            container(col)
                .padding(Padding::from([4, 12]))
                .style(container::bordered_box)
                .into()
        }
        Block::CodeBlock(cb) => {
            container(text(cb.content.clone()).size(13.0))
                .padding(Padding::from([8, 12]))
                .width(Length::Fill)
                .style(container::bordered_box)
                .into()
        }
        Block::Table(table) => {
            let mut col: Column<'static, Message> = Column::new().spacing(0).width(Length::Fill);

            let header_row = render_table_row(&table.header, true);
            col = col.push(header_row);
            col = col.push(horizontal_rule(1));

            for row_data in &table.rows {
                col = col.push(render_table_row(row_data, false));
            }

            container(col)
                .width(Length::Fill)
                .style(container::bordered_box)
                .padding(Padding::from([4, 0]))
                .into()
        }
        Block::Image(img) => text(format!("[图片: {}]", img.alt)).size(14.0).into(),
        Block::HorizontalRule => horizontal_rule(1).into(),
    }
}

fn render_table_row(cells: &[Vec<InlineSpan>], is_header: bool) -> Element<'static, Message> {
    let mut r: Row<'static, Message> = Row::new().spacing(0);

    for cell_spans in cells {
        let iced_spans: Vec<_> = cell_spans.iter().map(|s| {
            let mut sp = to_iced_span(s, 14.0);
            if is_header {
                sp = sp.font(cjk_font(Weight::Bold, iced::font::Style::Normal));
            }
            sp
        }).collect();

        let cell_content: Element<'static, ()> = rich_text(iced_spans).size(14.0).into();
        let cell_el: Element<'static, Message> = cell_content.map(|_| Message::Noop);

        r = r.push(
            container(cell_el)
                .width(Length::Fill)
                .padding(Padding::from([4, 8]))
        );
    }

    r.width(Length::Fill).into()
}

fn to_iced_span(s: &InlineSpan, size: f32) -> iced::widget::text::Span<'static> {
    let weight = if s.bold { Weight::Bold } else { Weight::Normal };
    let style = if s.italic { iced::font::Style::Italic } else { iced::font::Style::Normal };

    span(s.text.clone())
        .size(size)
        .font(cjk_font(weight, style))
}
