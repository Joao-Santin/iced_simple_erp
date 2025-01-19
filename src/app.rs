use iced::{widget::column, widget::button, widget::row, widget::container, Theme, Element, Fill};

use std::collections::HashMap;

use crate::screens::{login, config};

//all screens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screen {
    Login,
    Config,
}
//all inputs
#[derive(Debug, Clone)]
pub enum InputField {
    Username,
    Password,
}

#[derive(Debug, Clone)]
pub enum ButtonP {
    Test,
    LoginEnter
}

//all info for screens, used to a hashmap in app structure(SERP)
#[derive(Debug, Clone)]
pub struct ScreenData {
    pub title: &'static str,
    pub condition: Option<fn() -> bool>,
    pub view: fn(&SERP) -> Element<Message>,
}

// all messages
#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(ButtonP),
    ThemeChanged(Theme),
    SwitchTo(Screen),
    InputChanged(InputField, String)
}

//the app(basically)
pub struct SERP {
    pub screen: Screen,
    pub ti_login: String,
    pub ti_password: String,
    pub theme: Theme,
    pub screens: HashMap<Screen, ScreenData>,
}

// the default for app(how it starts)
impl Default for SERP {
    fn default() -> Self {
        let mut screens = HashMap::new();

        // Registro de telas
        screens.insert(
            Screen::Login,
            ScreenData {
                title: "Login",
                condition: None,
                view: login::view,
            },
        );
        screens.insert(
            Screen::Config,
            ScreenData {
                title: "Config",
                condition: Some(|| true),
                view: config::view,
            },
        );

        Self {
            screen: Screen::Login,
            ti_login: String::new(),
            ti_password: String::new(),
            theme: Theme::GruvboxDark,
            screens,
        }
    }
}
fn getting_config() -> String{
    "joginho".to_string()
}

// all functions needed for the app
impl SERP {
    pub fn title(&self) -> String {
        let screen_data = self.screens.get(&self.screen).unwrap();
        format!("{} - SERP", screen_data.title)
    }

    pub fn update(&mut self, event: Message) {
        match event {
            Message::ButtonPressed(button) => {
                match button{
                    ButtonP::Test =>{println!("testeeee")},
                    ButtonP::LoginEnter => {
                        if self.ti_login == "1" && self.ti_password == "1"{
                            println!("cuidado pessoal la vem vindo a veraneio")
                        }
                    }
                }
            },
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::SwitchTo(screen) => {
                if let Some(data) = self.screens.get(&screen) {
                    if let Some(condition) = data.condition {
                        if condition() {
                            self.screen = screen;
                        } else {
                            println!("Acesso negado à tela {:?}", screen);
                        }
                    } else {
                        self.screen = screen;
                    }
                }
            }
            Message::InputChanged(field, value) => {
                match field{
                    InputField::Username => {
                        self.ti_login = value;
                        println!("{}", self.ti_login)
                    },
                    InputField::Password => {
                        self.ti_password = value;
                        println!("{}", self.ti_password)
                    }
                }
                
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let top_acess = row![].spacing(10)
            .push(padded_button("Login").on_press(Message::SwitchTo(Screen::Login)))
            .push(padded_button("Config").on_press(Message::SwitchTo(Screen::Config)));

        let controls = row![]
            .push(padded_button("Next").on_press(Message::ButtonPressed(ButtonP::Test)));

        let screen_data = self.screens.get(&self.screen).unwrap();
        let screen_view = (screen_data.view)(self);

        let content: Element<_> = column![top_acess, screen_view, controls].into();

        container(content)
            .center_y(Fill)
            .center_x(Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn padded_button<Message: Clone>(label: &str) -> iced::widget::Button<'_, Message> {
    button(iced::widget::text(label)).padding([12, 24])
}

