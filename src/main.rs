use iced::widget::{column, text, button, row, container, pick_list};
use iced::widget::{Column, Button};
use iced::{Element, Fill, Theme};

mod modules;

fn main()-> iced::Result {
    println!("Hello, world!");
    iced::application(SERP::title, SERP::update, SERP::view)
        .default_font(iced::Font::MONOSPACE)
        .theme(SERP::theme)
        .centered()
        .run()
}

pub struct SERP{
    screen: Screen,
    theme: Theme
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Login,
    Config
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    ThemeChanged(Theme),
    SwitchTo(Screen)
}

impl Default for SERP {
    fn default() -> Self {
        Self {
            screen: Screen::Login,
            theme: Theme::GruvboxDark
        }
    }
}

impl SERP{
    fn title(&self) -> String{
        let screen = match self.screen{
            Screen::Login => "Login",
            Screen::Config => "Config"
        };
        format!("{} - SERP", screen)
    }

    fn update(&mut self, event:Message){
        match event {
            Message::ButtonPressed => println!("Apenas pressionando alguma coisa"),
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            },
            Message::SwitchTo(screen) => {
                self.screen = screen
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let top_acess = 
            row![]
                .push({padded_button("Login")
                .on_press(Message::SwitchTo(Screen::Login))})
                .push({padded_button("Config")
                .on_press(Message::SwitchTo(Screen::Config))});
                
        let controls = 
            row![]
                .push({padded_button("Next")
                    .on_press(Message::ButtonPressed)
                    });

        let screen = match self.screen {
            Screen::Login => self.login(),
            Screen::Config => self.config(),
        };

        let content: Element<_> = column![
            top_acess, screen, controls
            ]
            .into();

        container(content).center_y(Fill).center_x(Fill).into()
    }

    fn theme(&self) -> Theme{
        self.theme.clone()
    }

    fn login(&self) ->Column<Message>{
        Self::container("Bem vindo!")
            .push("apenas um teste, gatao")
    }

    fn config(&self) -> Column<Message>{
        Self::container("Configs")
            .push(column![
                text("Theme:"),
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                    .width(Fill),
            ]
            .spacing(10))
    }

    fn container(title: &str) -> Column<'_, Message>{
        column![
            text(title).size(50)
        ].spacing(20)
    }
}

fn padded_button<Message: Clone>(label:&str)-> Button<'_, Message> {
    button(text(label)).padding([12,24])
}

