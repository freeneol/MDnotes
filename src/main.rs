mod markdown;
mod io;
mod app;
mod ui;

use app::App;
use iced::font::Family;
use iced::Font;

#[cfg(target_os = "macos")]
const CJK_FONT: &str = "PingFang SC";

#[cfg(target_os = "windows")]
const CJK_FONT: &str = "Microsoft YaHei";

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const CJK_FONT: &str = "Noto Sans CJK SC";

fn main() -> iced::Result {
    let default_font = Font {
        family: Family::Name(CJK_FONT),
        ..Font::DEFAULT
    };

    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .default_font(default_font)
        .window_size((1000.0, 700.0))
        .run_with(App::new)
}
