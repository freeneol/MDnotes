use std::path::PathBuf;

use iced::widget::{button, container, horizontal_rule, scrollable, text, Column};
use iced::{Element, Length, Padding};

use crate::app::Message;
use crate::io::file::FileEntry;

pub fn view(
    file_tree: &[FileEntry],
    recent_files: &[PathBuf],
    has_main_folder: bool,
) -> Element<'static, Message> {
    let mut col = Column::new().spacing(4).width(Length::Fill);

    if has_main_folder {
        col = col.push(text("主文件夹").size(13));
        col = render_tree(col, file_tree, 0);
    }

    col = col.push(
        button(text("打开文件夹").size(13))
            .on_press(Message::OpenFolder)
            .padding(Padding::from([3, 8])),
    );

    col = col.push(horizontal_rule(1));
    col = col.push(text("最近文件").size(13));

    if recent_files.is_empty() {
        col = col.push(text("暂无记录").size(12));
    } else {
        for path in recent_files.iter() {
            let display = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let path_clone = path.clone();
            col = col.push(
                button(text(display).size(12))
                    .on_press(Message::SelectFile(path_clone))
                    .padding(Padding::from([2, 6]))
                    .style(button::text),
            );
        }
    }

    container(scrollable(
        container(col).padding(Padding::from([8, 10])),
    ))
    .height(Length::Fill)
    .into()
}

fn render_tree(
    mut col: Column<'static, Message>,
    entries: &[FileEntry],
    depth: usize,
) -> Column<'static, Message> {
    let indent = "  ".repeat(depth);
    for entry in entries {
        match entry {
            FileEntry::Directory {
                name, children, ..
            } => {
                col = col.push(text(format!("{}{}/", indent, name)).size(12));
                col = render_tree(col, children, depth + 1);
            }
            FileEntry::File { name, path } => {
                let path_clone = path.clone();
                col = col.push(
                    button(text(format!("{}{}", indent, name)).size(12))
                        .on_press(Message::SelectFile(path_clone))
                        .padding(Padding::from([1, 4]))
                        .style(button::text),
                );
            }
        }
    }
    col
}
