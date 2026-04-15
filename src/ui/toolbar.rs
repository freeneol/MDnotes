use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length, Padding};

use crate::app::{Message, Mode};

pub fn view(has_file: bool, mode: &Mode) -> Element<'static, Message> {
    let open_btn = button(text("打开文件").size(14))
        .on_press(Message::OpenFile)
        .padding(Padding::from([4, 12]));

    let save_btn = {
        let btn = button(text("保存").size(14)).padding(Padding::from([4, 12]));
        if has_file {
            btn.on_press(Message::SaveFile)
        } else {
            btn
        }
    };

    let mode_label = match mode {
        Mode::Reading => "编辑",
        Mode::Editing => "阅读",
    };
    let mode_btn = {
        let btn = button(text(mode_label).size(14)).padding(Padding::from([4, 12]));
        if has_file {
            btn.on_press(Message::ToggleMode)
        } else {
            btn
        }
    };

    let toolbar_row = row![open_btn, save_btn, mode_btn]
        .spacing(8)
        .align_y(Alignment::Center);

    container(toolbar_row)
        .width(Length::Fill)
        .padding(Padding::from([8, 12]))
        .into()
}
