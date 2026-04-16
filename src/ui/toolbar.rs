use iced::widget::{button, container, horizontal_rule, row, text, Column};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{Message, Mode};

pub fn view(has_file: bool, mode: &Mode, breadcrumb: &[String]) -> Element<'static, Message> {
    let sidebar_btn = button(text("≡").size(18))
        .on_press(Message::ToggleSidebar)
        .padding(Padding::from([4, 10]))
        .style(button::text);

    let open_btn = button(text("打开").size(13))
        .on_press(Message::OpenFile)
        .padding(Padding::from([4, 8]))
        .style(button::text);

    let save_btn = {
        let btn = button(text("保存").size(13))
            .padding(Padding::from([4, 8]))
            .style(button::text);
        if has_file {
            btn.on_press(Message::SaveFile)
        } else {
            btn
        }
    };

    let left_group = row![sidebar_btn, open_btn, save_btn]
        .spacing(2)
        .align_y(Alignment::Center);

    let breadcrumb_el: Element<'static, Message> = if breadcrumb.is_empty() {
        text("").into()
    } else {
        let mut r = row![].spacing(0).align_y(Alignment::Center);
        for (i, seg) in breadcrumb.iter().enumerate() {
            if i > 0 {
                r = r.push(text(" > ").size(13));
            }
            r = r.push(text(seg.clone()).size(13));
        }
        container(r).padding(Padding::from([0, 12])).into()
    };

    let preview_style = if *mode == Mode::Preview { button::primary } else { button::secondary };
    let markdown_style = if *mode == Mode::Markdown { button::primary } else { button::secondary };

    let preview_btn = {
        let btn = button(text("Preview").size(13))
            .padding(Padding::from([4, 12]))
            .style(preview_style);
        if has_file { btn.on_press(Message::SwitchToPreview) } else { btn }
    };

    let markdown_btn = {
        let btn = button(text("Markdown").size(13))
            .padding(Padding::from([4, 12]))
            .style(markdown_style);
        if has_file { btn.on_press(Message::SwitchToMarkdown) } else { btn }
    };

    let right_group = row![preview_btn, markdown_btn]
        .spacing(4)
        .align_y(Alignment::Center);

    let toolbar_row = row![
        left_group,
        container(breadcrumb_el).width(Length::Fill),
        right_group,
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    Column::new()
        .push(
            container(toolbar_row)
                .width(Length::Fill)
                .padding(Padding::from([6, 12])),
        )
        .push(horizontal_rule(1))
        .into()
}
