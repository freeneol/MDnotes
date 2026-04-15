use iced::widget::{
    column, container, rich_text, scrollable, span, text, text_editor, Column,
};
use iced::font::{Family, Weight};
use iced::{Element, Font, Length, Padding};

use crate::app::{Message, Mode};
use crate::markdown::model::*;

pub fn view<'a>(
    document: &Document,
    mode: &Mode,
    _content_text: &'a str,
    editor_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    match mode {
        Mode::Reading => view_reading(document),
        Mode::Editing => view_editing(editor_content),
    }
}

fn view_reading(document: &Document) -> Element<'static, Message> {
    let mut col = Column::new().spacing(8).padding(Padding::from([16, 20]));

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
            rich_text(iced_spans).size(size).into()
        }
        Block::Paragraph(spans) => {
            let iced_spans: Vec<_> = spans.iter().map(|s| to_iced_span(s, 15.0)).collect();
            rich_text(iced_spans).size(15.0).into()
        }
        Block::List(items) => {
            let mut col = Column::new().spacing(2);
            for (i, item) in items.iter().enumerate() {
                let prefix = if item.ordered {
                    format!("{}. ", i + 1)
                } else {
                    "• ".to_string()
                };
                let mut iced_spans = vec![span(prefix).size(15.0)];
                iced_spans.extend(item.spans.iter().map(|s| to_iced_span(s, 15.0)));
                col = col.push(rich_text(iced_spans).size(15.0));
            }
            col.into()
        }
        Block::BlockQuote(blocks) => {
            let mut col = Column::new().spacing(4);
            for b in blocks {
                col = col.push(render_block(b));
            }
            container(col)
                .padding(Padding::from([4, 12]))
                .style(container::bordered_box)
                .into()
        }
        Block::CodeBlock(cb) => {
            let code_font = Font {
                family: Family::Monospace,
                ..Font::DEFAULT
            };
            container(text(&cb.content).size(13.0).font(code_font))
                .padding(Padding::from([8, 12]))
                .width(Length::Fill)
                .style(container::bordered_box)
                .into()
        }
        Block::Image(img) => text(format!("[图片: {}]", img.alt)).size(14.0).into(),
        Block::HorizontalRule => iced::widget::horizontal_rule(1).into(),
    }
}

fn to_iced_span(
    s: &InlineSpan,
    size: f32,
) -> iced::widget::span::Span<'static, iced::Theme, iced::Renderer> {
    let mut sp = span(s.text.clone()).size(size);

    if s.bold {
        sp = sp.font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        });
    }
    if s.italic {
        sp = sp.font(Font {
            style: iced::font::Style::Italic,
            ..Font::DEFAULT
        });
    }
    if s.code {
        sp = sp.font(Font {
            family: Family::Monospace,
            ..Font::DEFAULT
        });
    }

    sp
}
