mod app;        // Importa a lógica principal
mod screens;    // Importa as telas
mod db;

use app::SERP;
fn main() -> iced::Result {
    iced::application(SERP::title, SERP::update, SERP::view)
        .default_font(iced::Font::MONOSPACE)
        .theme(SERP::theme)
        .centered()
        .run()
}

