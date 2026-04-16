use iced::widget::{container, horizontal_rule, text, Column};
use iced::{Element, Length, Padding};

use crate::app::Message;

pub fn view(status: String) -> Element<'static, Message> {
    Column::new()
        .push(horizontal_rule(1))
        .push(
            container(text(status).size(12))
                .width(Length::Fill)
                .padding(Padding::from([4, 12])),
        )
        .into()
}
