use iced::{widget::{column, pick_list, text}, Element, Length::Shrink};
use crate::app::{Message, SERP};

pub fn view(app: &SERP) -> Element<Message> {
    column![
        text("Configs").size(50),
        column![
            text("Theme:"),
            pick_list(iced::Theme::ALL, Some(&app.theme), Message::ThemeChanged)
                .width(Shrink),
        ]
        .spacing(10),
    ]
    .spacing(20)
    .into()
}
