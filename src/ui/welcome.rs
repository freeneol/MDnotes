use iced::widget::{center, column, text};
use iced::Element;

use crate::app::Message;

pub fn view() -> Element<'static, Message> {
    center(
        column![
            text("MDnotes").size(36),
            text("选择或打开一个文件开始").size(16),
        ]
        .spacing(12)
        .align_x(iced::Alignment::Center),
    )
    .into()
}
