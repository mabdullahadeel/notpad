use iced::theme::{self, Theme};
use iced::widget::Row;
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor, tooltip};
use iced::{executor, keyboard, Subscription};
use iced::{Alignment, Application, Command, Element, Font, Length, Settings};

use editor_stylesheet::EditorStyleSheet;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
mod editor_stylesheet;

pub fn main() -> iced::Result {
    NotPad::run(Settings {
        default_font: Font::DEFAULT,
        ..Settings::default()
    })
}
fn generate_random_string() -> String {
    uuid::Uuid::new_v4().to_string()
}

struct TabInfo {
    name: String,
    content: text_editor::Content,
    is_dirty: bool,
    file_path: Option<PathBuf>,
}

impl Default for TabInfo {
    fn default() -> Self {
        Self {
            name: "New file".to_string(),
            content: text_editor::Content::new(),
            is_dirty: false,
            file_path: None,
        }
    }
}

struct NotPad {
    current_tab: String,
    content: HashMap<String, TabInfo>,
    is_loading: bool,
}

impl NotPad {
    fn get_current_content(&self) -> &text_editor::Content {
        &self.content.get(&self.current_tab).unwrap().content
    }
    fn add_new_tab(&mut self) {
        let new_tab = generate_random_string();
        self.content.insert(new_tab.clone(), TabInfo::default());
        self.current_tab = new_tab;
    }
}

#[derive(Debug, Clone)]
enum Message {
    ActionPerformed(text_editor::Action),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<(String, PathBuf), Error>),
    SwitchTab(String),
    RemoveTab(String),
}

impl Application for NotPad {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let mut content = HashMap::new();
        let current_tab = generate_random_string();
        content.insert(current_tab.clone(), TabInfo::default());
        (
            Self {
                current_tab,
                content,
                is_loading: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("NotPad")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ActionPerformed(action) => {
                match action {
                    text_editor::Action::Edit(_) => {
                        if action.is_edit() {
                            self.content.get_mut(&self.current_tab).unwrap().is_dirty = true;
                        }
                    }
                    _ => {}
                };
                self.content
                    .get_mut(&self.current_tab)
                    .unwrap()
                    .content
                    .perform(action);
                Command::none()
            }
            Message::NewFile => {
                self.add_new_tab();

                Command::none()
            }
            Message::RemoveTab(tab) => {
                if self.content.len() == 1 {
                    return Command::none();
                }
                self.content.remove(&tab);
                if self.current_tab == tab {
                    self.current_tab = self.content.keys().next().unwrap().clone();
                }
                Command::none()
            }
            Message::OpenFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(open_file(), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;

                if let Ok((path, contents)) = result {
                    for (_, tab) in self.content.iter() {
                        if tab.file_path.clone().is_some_and(|p| p == path) {
                            return Command::none();
                        }
                    }
                    let id = generate_random_string();
                    self.content.insert(
                        id.clone(),
                        TabInfo {
                            name: path.file_name().unwrap().to_str().unwrap().to_string(),
                            content: text_editor::Content::with_text(&contents),
                            is_dirty: false,
                            file_path: Some(path),
                        },
                    );
                    self.current_tab = id;
                }

                Command::none()
            }
            Message::SaveFile => {
                let current_tab = self.content.get(&self.current_tab).unwrap();
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(
                        save_file(
                            self.current_tab.clone(),
                            current_tab.file_path.clone(),
                            current_tab.content.text(),
                        ),
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    let tab = self.content.get_mut(&path.0).unwrap();
                    tab.is_dirty = false;
                    tab.name = path
                        .1
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .split('.')
                        .next()
                        .unwrap()
                        .to_string();
                    tab.file_path = Some(path.1);
                }

                Command::none()
            }
            Message::SwitchTab(tab) => {
                self.current_tab = tab;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("s") if modifiers.command() => Some(Message::SaveFile),
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let mut tabs_row = Row::new();
        for (tab, info) in self.content.iter() {
            let tab = tab.clone();
            tabs_row = tabs_row.push(row![
                button(text(format!(
                    "{}{}",
                    info.name.clone(),
                    if info.is_dirty { " *" } else { "" }
                )))
                .style(if tab.clone() == self.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                })
                .on_press(Message::SwitchTab(tab.clone())),
                button(text("X"))
                    .style(if tab.clone() == self.current_tab {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    })
                    .on_press(Message::RemoveTab(tab.clone()))
            ]);
        }
        let controls = row![
            menu_action("New", "New file", Some(Message::NewFile)),
            menu_action(
                "Open",
                "Open file",
                (!self.is_loading).then_some(Message::OpenFile)
            ),
            horizontal_space(),
        ]
        .align_items(Alignment::Center);

        let status = row![
            text(String::from("New file")),
            horizontal_space(),
            text({
                let (line, column) = self.get_current_content().cursor_position();

                format!("{}:{}", line + 1, column + 1)
            })
        ]
        .padding(10)
        .spacing(10);

        column![
            column![controls, tabs_row],
            text_editor(&self.get_current_content())
                .height(Length::Fill)
                .style(EditorStyleSheet)
                .on_action(Message::ActionPerformed),
            status,
        ]
        .spacing(10)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(
    tab_id: String,
    path: Option<PathBuf>,
    contents: String,
) -> Result<(String, PathBuf), Error> {
    let mut path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    if path.extension().is_none() {
        path.set_extension("notpad");
    }

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((tab_id, path))
}

fn menu_action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x()).style(theme::Button::Text);

    if let Some(on_press) = on_press {
        tooltip(
            row![action.on_press(on_press)],
            label,
            tooltip::Position::Right,
        )
        .style(theme::Container::Box)
        .into()
    } else {
        action.into()
    }
}
