use iced::{widget::row, widget::column, widget::text, widget::text_input , Element, Length::Fixed, widget::button};
use crate::app::{InputField, ButtonP, Message, SERP};

pub fn view(app: &SERP) -> Element<Message> {
    column![
        row![
            text("Username:"),
            text_input("username here ...", &app.ti_login)
            .on_input(|value| Message::InputChanged(InputField::Username, value))
            .width(Fixed(200.0)),

        ],
        row![
            text("Password:"),
            text_input("password here ...", &app.ti_password)
            .secure(true)
            .on_input(|value| Message::InputChanged(InputField::Password, value))
            .width(Fixed(200.0))
        ],
        row![
            button(text("enter")).on_press(Message::ButtonPressed(ButtonP::LoginEnter))
        ]
    ]
    .padding(50)
    .spacing(20)
    .into()
}

