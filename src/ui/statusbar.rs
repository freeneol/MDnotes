use iced::widget::{container, text};
use iced::{Element, Length, Padding};

use crate::app::Message;

pub fn view(status: String) -> Element<'static, Message> {
    container(text(status).size(12))
        .width(Length::Fill)
        .padding(Padding::from([4, 12]))
        .into()
}
