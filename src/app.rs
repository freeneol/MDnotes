use std::collections::HashSet;
use std::path::PathBuf;

use iced::widget::{
    button, column, container, horizontal_space, mouse_area, row, text, text_editor, vertical_rule,
};
use iced::{Element, Length, Padding, Task};

use crate::io::config::AppConfig;
use crate::io::file::{self, FileEntry};
use crate::markdown::model::Document;
use crate::markdown::{parser, serializer};

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Preview,
    Markdown,
}

pub struct App {
    pub config: AppConfig,
    pub current_file: Option<PathBuf>,
    pub document: Option<Document>,
    pub original_content: Option<String>,
    pub content_text: String,
    pub mode: Mode,
    pub file_tree: Vec<FileEntry>,
    pub sidebar_visible: bool,
    pub expanded_folders: HashSet<PathBuf>,
    pub editor_content: text_editor::Content,
    pub sidebar_dragging: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    FileOpened(Option<(PathBuf, String)>),
    SaveFile,
    #[allow(dead_code)]
    FileSaved(Result<(), String>),
    SwitchToPreview,
    SwitchToMarkdown,
    OpenFolder,
    FolderSelected(Option<PathBuf>),
    SelectFile(PathBuf),
    ToggleSidebar,
    ToggleFolder(PathBuf),
    CloseFile,
    SidebarDragStart,
    SidebarDragMove(f32),
    SidebarDragEnd,
    EditorAction(text_editor::Action),
    KeyPressed(iced::keyboard::Key, iced::keyboard::Modifiers),
    Noop,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let config = AppConfig::load();
        let file_tree = config
            .main_folder
            .as_ref()
            .map(|f| file::list_all_files(f))
            .unwrap_or_default();

        let app = Self {
            config,
            current_file: None,
            document: None,
            original_content: None,
            content_text: String::new(),
            mode: Mode::Preview,
            file_tree,
            sidebar_visible: true,
            expanded_folders: HashSet::new(),
            editor_content: text_editor::Content::new(),
            sidebar_dragging: false,
        };
        (app, Task::none())
    }

    pub fn title(&self) -> String {
        match &self.current_file {
            Some(path) => format!(
                "MDnotes - {}",
                path.file_name().unwrap_or_default().to_string_lossy()
            ),
            None => "MDnotes".to_string(),
        }
    }

    pub fn breadcrumb(&self) -> Vec<String> {
        let Some(file_path) = &self.current_file else {
            return Vec::new();
        };
        if let Some(folder) = &self.config.main_folder {
            if let Ok(rel) = file_path.strip_prefix(folder) {
                return rel
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
                    .collect();
            }
        }
        vec![file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()]
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFile => {
                return Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .add_filter("Markdown", &["md"])
                            .pick_file()
                            .await;
                        match handle {
                            Some(h) => {
                                let path = h.path().to_path_buf();
                                match file::read_file(&path) {
                                    Ok(content) => Some((path, content)),
                                    Err(_) => None,
                                }
                            }
                            None => None,
                        }
                    },
                    Message::FileOpened,
                );
            }
            Message::FileOpened(result) => {
                if let Some((path, content)) = result {
                    self.load_content(path, content);
                }
            }
            Message::SaveFile => {
                if let (Some(path), Some(_doc)) = (&self.current_file, &self.document) {
                    let content = match self.mode {
                        Mode::Markdown => {
                            let doc = parser::parse(&self.content_text);
                            self.document = Some(doc.clone());
                            serializer::serialize(&doc)
                        }
                        Mode::Preview => self
                            .document
                            .as_ref()
                            .map(|d| serializer::serialize(d))
                            .unwrap_or_default(),
                    };
                    let path = path.clone();
                    self.original_content = Some(content.clone());
                    return Task::perform(
                        async move { file::write_file(&path, &content) },
                        Message::FileSaved,
                    );
                }
            }
            Message::FileSaved(_) => {}
            Message::SwitchToPreview => {
                if self.document.is_some() && self.mode == Mode::Markdown {
                    let doc = parser::parse(&self.content_text);
                    self.document = Some(doc);
                    self.mode = Mode::Preview;
                }
            }
            Message::SwitchToMarkdown => {
                if self.document.is_some() && self.mode == Mode::Preview {
                    self.content_text = self
                        .document
                        .as_ref()
                        .map(|d| serializer::serialize(d))
                        .unwrap_or_default();
                    self.editor_content =
                        text_editor::Content::with_text(&self.content_text);
                    self.mode = Mode::Markdown;
                }
            }
            Message::OpenFolder => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_folder()
                            .await
                            .map(|h| h.path().to_path_buf())
                    },
                    Message::FolderSelected,
                );
            }
            Message::FolderSelected(folder) => {
                if let Some(path) = folder {
                    self.file_tree = file::list_all_files(&path);
                    self.config.main_folder = Some(path);
                    self.expanded_folders.clear();
                    self.config.save();
                }
            }
            Message::SelectFile(path) => {
                if self.has_unsaved_changes() {
                    if let (Some(cur_path), Some(doc)) = (&self.current_file, &self.document) {
                        let content = serializer::serialize(doc);
                        let _ = file::write_file(cur_path, &content);
                    }
                }
                match file::read_file(&path) {
                    Ok(content) => self.load_content(path, content),
                    Err(_) => {}
                }
            }
            Message::ToggleSidebar => {
                self.sidebar_visible = !self.sidebar_visible;
            }
            Message::ToggleFolder(path) => {
                if self.expanded_folders.contains(&path) {
                    self.expanded_folders.remove(&path);
                } else {
                    self.expanded_folders.insert(path);
                }
            }
            Message::CloseFile => {
                self.current_file = None;
                self.document = None;
                self.original_content = None;
                self.content_text.clear();
                self.mode = Mode::Preview;
                self.editor_content = text_editor::Content::new();
            }
            Message::SidebarDragStart => {
                if self.sidebar_visible {
                    self.sidebar_dragging = true;
                }
            }
            Message::SidebarDragMove(x) => {
                if self.sidebar_dragging {
                    self.config.sidebar_width = x.clamp(120.0, 500.0);
                }
            }
            Message::SidebarDragEnd => {
                if self.sidebar_dragging {
                    self.sidebar_dragging = false;
                    self.config.save();
                }
            }
            Message::EditorAction(action) => {
                self.editor_content.perform(action);
                self.content_text = self.editor_content.text();
            }
            Message::KeyPressed(key, modifiers) => {
                use iced::keyboard::Key;
                if modifiers.command() {
                    match &key {
                        Key::Character(c) if c.as_str() == "o" => {
                            return self.update(Message::OpenFile);
                        }
                        Key::Character(c) if c.as_str() == "s" => {
                            return self.update(Message::SaveFile);
                        }
                        Key::Character(c) if c.as_str() == "w" => {
                            return self.update(Message::CloseFile);
                        }
                        Key::Character(c) if c.as_str() == "\\" => {
                            return self.update(Message::ToggleSidebar);
                        }
                        _ => {}
                    }
                }
            }
            Message::Noop => {}
        }
        Task::none()
    }

    fn load_content(&mut self, path: PathBuf, content: String) {
        let doc = parser::parse(&content);
        self.document = Some(doc);
        self.original_content = Some(content.clone());
        self.content_text = content;
        self.current_file = Some(path.clone());
        self.mode = Mode::Preview;
        self.config.add_recent_file(path);
        self.config.save();
    }

    pub fn has_unsaved_changes(&self) -> bool {
        match (&self.document, &self.original_content) {
            (Some(doc), Some(original)) => {
                let current = serializer::serialize(doc);
                current != *original
            }
            _ => false,
        }
    }

    pub fn status_text(&self) -> String {
        let path_str = self
            .current_file
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "未打开文件".to_string());

        let word_count = self.document.as_ref().map(|d| d.word_count()).unwrap_or(0);

        let mode_str = match self.mode {
            Mode::Preview => "Preview",
            Mode::Markdown => "Markdown",
        };

        format!("{} | {} 字 | {}", path_str, word_count, mode_str)
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let key_sub = iced::keyboard::on_key_press(|key, modifiers| {
            Some(Message::KeyPressed(key, modifiers))
        });

        if self.sidebar_dragging {
            let drag_sub = iced::event::listen_with(|event, _status, _id| match event {
                iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                    Some(Message::SidebarDragMove(position.x))
                }
                iced::Event::Mouse(iced::mouse::Event::ButtonReleased(
                    iced::mouse::Button::Left,
                )) => Some(Message::SidebarDragEnd),
                _ => None,
            });
            iced::Subscription::batch(vec![key_sub, drag_sub])
        } else {
            key_sub
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar = crate::ui::toolbar::view(
            self.current_file.is_some(),
            &self.mode,
            &self.breadcrumb(),
        );

        let main_content: Element<'_, Message> = if self.document.is_some() {
            let content = crate::ui::content::view(
                self.document.as_ref().unwrap(),
                &self.mode,
                &self.content_text,
                &self.editor_content,
            );

            let close_row = container(
                row![
                    horizontal_space(),
                    button(text("×").size(16))
                        .on_press(Message::CloseFile)
                        .padding(Padding::from([2, 8]))
                        .style(button::text),
                ]
                .width(Length::Fill),
            )
            .padding(Padding::from([2, 4]));

            column![close_row, content]
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
        } else {
            crate::ui::welcome::view()
        };

        let middle = if self.sidebar_visible {
            let sidebar = crate::ui::sidebar::view(
                &self.file_tree,
                &self.expanded_folders,
                &self.config.main_folder,
            );

            let divider = mouse_area(
                container(vertical_rule(1))
                    .width(Length::Fixed(6.0))
                    .height(Length::Fill),
            )
            .on_press(Message::SidebarDragStart);

            row![
                container(sidebar)
                    .width(Length::Fixed(self.config.sidebar_width))
                    .height(Length::Fill),
                divider,
                container(main_content)
                    .width(Length::Fill)
                    .height(Length::Fill),
            ]
            .height(Length::Fill)
        } else {
            row![container(main_content)
                .width(Length::Fill)
                .height(Length::Fill)]
            .height(Length::Fill)
        };

        let statusbar = crate::ui::statusbar::view(self.status_text());

        column![toolbar, middle, statusbar]
            .height(Length::Fill)
            .into()
    }
}
