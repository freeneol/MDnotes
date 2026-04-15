mod markdown;
mod io;
mod app;
mod ui;

use app::App;

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .window_size((1000.0, 700.0))
        .run_with(App::new)
}
