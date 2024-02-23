use iced::{
    widget::text_editor::{self, StyleSheet},
    Border, Theme,
};

pub struct EditorStyleSheet;

impl StyleSheet for EditorStyleSheet {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: iced::Background::Color(iced::Color::from_rgb(
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
            )),
            border: Border {
                width: 0.0,
                ..Default::default()
            },
        }
    }
    fn disabled(&self, _: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: iced::Background::Color(iced::Color::from_rgb(
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
            )),
            border: Border {
                width: 0.0,
                ..Default::default()
            },
        }
    }
    fn disabled_color(&self, _: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.6, 0.6, 0.6)
    }
    fn placeholder_color(&self, _: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn focused(&self, _: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: iced::Background::Color(iced::Color::from_rgb(
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
            )),
            border: Border {
                ..Default::default()
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: iced::Background::Color(iced::Color::from_rgb(
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
                39.0f32 / 255.0f32,
            )),
            border: Border {
                ..Default::default()
            },
        }
    }

    fn selection_color(&self, _: &Self::Style) -> iced::Color {
        iced::Color::from_rgb(0.5, 0.5, 1.0)
    }

    fn value_color(&self, _: &Self::Style) -> iced::Color {
        iced::Color::WHITE
    }
}

impl Into<iced::theme::TextEditor> for EditorStyleSheet {
    fn into(self) -> iced::theme::TextEditor {
        iced::theme::TextEditor::Custom(Box::new(EditorStyleSheet))
    }
}
