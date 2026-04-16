use std::path::PathBuf;

use iced::widget::{column, container, row, text_editor};
use iced::{Element, Length, Task};

use crate::io::config::AppConfig;
use crate::io::file::{self, FileEntry};
use crate::markdown::model::Document;
use crate::markdown::{parser, serializer};

#[derive(Debug, Clone)]
pub enum Mode {
    Reading,
    Editing,
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
    pub editor_content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    FileOpened(Option<(PathBuf, String)>),
    SaveFile,
    FileSaved(#[allow(dead_code)] Result<(), String>),
    ToggleMode,
    OpenFolder,
    FolderSelected(Option<PathBuf>),
    SelectFile(PathBuf),
    ToggleSidebar,
    #[allow(dead_code)]
    ContentChanged(String),
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
            .map(|f| file::list_md_files(f))
            .unwrap_or_default();

        let app = Self {
            config,
            current_file: None,
            document: None,
            original_content: None,
            content_text: String::new(),
            mode: Mode::Reading,
            file_tree,
            sidebar_visible: true,
            editor_content: text_editor::Content::new(),
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
                        Mode::Editing => {
                            let doc = parser::parse(&self.content_text);
                            self.document = Some(doc.clone());
                            serializer::serialize(&doc)
                        }
                        Mode::Reading => self
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
            Message::ToggleMode => match self.mode {
                Mode::Reading => {
                    if self.document.is_some() {
                        self.content_text = self
                            .document
                            .as_ref()
                            .map(|d| serializer::serialize(d))
                            .unwrap_or_default();
                        self.editor_content =
                            text_editor::Content::with_text(&self.content_text);
                        self.mode = Mode::Editing;
                    }
                }
                Mode::Editing => {
                    let doc = parser::parse(&self.content_text);
                    self.document = Some(doc);
                    self.mode = Mode::Reading;
                }
            },
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
                    self.file_tree = file::list_md_files(&path);
                    self.config.main_folder = Some(path);
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
            Message::ContentChanged(new_content) => {
                self.content_text = new_content;
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
                        Key::Character(c) if c.as_str() == "e" => {
                            return self.update(Message::ToggleMode);
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
        self.mode = Mode::Reading;
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
            Mode::Reading => "阅读",
            Mode::Editing => "编辑",
        };

        format!("{} | {} 字 | {}", path_str, word_count, mode_str)
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|key, modifiers| {
            Some(Message::KeyPressed(key, modifiers))
        })
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar = crate::ui::toolbar::view(self.current_file.is_some(), &self.mode);

        let sidebar: Element<Message> = if self.sidebar_visible {
            crate::ui::sidebar::view(
                &self.file_tree,
                &self.config.recent_files,
                self.config.main_folder.is_some(),
            )
        } else {
            column![].into()
        };

        let main_content: Element<Message> = if self.document.is_some() {
            crate::ui::content::view(
                self.document.as_ref().unwrap(),
                &self.mode,
                &self.content_text,
                &self.editor_content,
            )
        } else {
            crate::ui::welcome::view()
        };

        let sidebar_width = if self.sidebar_visible {
            self.config.sidebar_width
        } else {
            0.0
        };

        let middle = row![
            container(sidebar).width(Length::Fixed(sidebar_width)).height(Length::Fill),
            container(main_content).width(Length::Fill).height(Length::Fill),
        ]
        .height(Length::Fill);

        let statusbar = crate::ui::statusbar::view(self.status_text());

        column![toolbar, middle, statusbar]
            .height(Length::Fill)
            .into()
    }
}
