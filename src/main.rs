use iced::widget::{center, text};
use iced::Element;

fn main() -> iced::Result {
    iced::application("MDnotes", App::update, App::view)
        .window_size((1000.0, 700.0))
        .run_with(App::new)
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn new() -> (Self, iced::Task<Message>) {
        (Self, iced::Task::none())
    }

    fn update(&mut self, _message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        center(text("MDnotes").size(32)).into()
    }
}
