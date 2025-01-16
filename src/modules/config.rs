pub struct Config{
}

fn config_view(&self) -> Column<Message>{
    Self::container("Configs")
        .push(column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                .width(Fill),
        ]
        .spacing(10))
}
