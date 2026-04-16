use std::collections::HashSet;
use std::path::PathBuf;

use iced::widget::{button, container, horizontal_rule, scrollable, text, Column};
use iced::{Element, Length, Padding};

use crate::app::Message;
use crate::io::file::FileEntry;

pub fn view(
    file_tree: &[FileEntry],
    expanded: &HashSet<PathBuf>,
    main_folder: &Option<PathBuf>,
) -> Element<'static, Message> {
    let mut col = Column::new().spacing(0).width(Length::Fill);

    let folder_name = main_folder
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "未选择文件夹".to_string());

    col = col.push(
        container(text(folder_name).size(13))
            .padding(Padding::from([8, 10]))
            .width(Length::Fill),
    );
    col = col.push(horizontal_rule(1));

    let mut tree_col = Column::new().spacing(0).width(Length::Fill);
    tree_col = render_tree(tree_col, file_tree, expanded, 0);

    col = col.push(
        scrollable(container(tree_col).padding(Padding::from([4, 0])))
            .height(Length::Fill),
    );

    col = col.push(horizontal_rule(1));
    col = col.push(
        container(
            button(text("+ 添加文件夹").size(12))
                .on_press(Message::OpenFolder)
                .padding(Padding::from([4, 8]))
                .style(button::text),
        )
        .padding(Padding::from([4, 6])),
    );

    container(col).height(Length::Fill).width(Length::Fill).into()
}

fn render_tree(
    mut col: Column<'static, Message>,
    entries: &[FileEntry],
    expanded: &HashSet<PathBuf>,
    depth: usize,
) -> Column<'static, Message> {
    let left_pad = (depth as u16) * 16 + 8;

    for entry in entries {
        match entry {
            FileEntry::Directory { name, path, children } => {
                let is_expanded = expanded.contains(path);
                let arrow = if is_expanded { "▾" } else { "▸" };
                let label = format!("{} {}", arrow, name);
                let path_clone = path.clone();

                col = col.push(
                    button(text(label).size(13))
                        .on_press(Message::ToggleFolder(path_clone))
                        .padding(Padding {
                            top: 3.0,
                            right: 6.0,
                            bottom: 3.0,
                            left: left_pad as f32,
                        })
                        .width(Length::Fill)
                        .style(button::text),
                );

                if is_expanded {
                    col = render_tree(col, children, expanded, depth + 1);
                }
            }
            FileEntry::File { name, path } => {
                let path_clone = path.clone();
                col = col.push(
                    button(text(name.clone()).size(12))
                        .on_press(Message::SelectFile(path_clone))
                        .padding(Padding {
                            top: 3.0,
                            right: 6.0,
                            bottom: 3.0,
                            left: (left_pad + 16) as f32,
                        })
                        .width(Length::Fill)
                        .style(button::text),
                );
            }
        }
    }
    col
}
