use iced::widget::{column, text, button, row, container};
use iced::widget::{Column, Button};
use iced::{Element, Fill};

fn main()-> iced::Result {
    println!("Hello, world!");
    iced::application(SERP::title, SERP::update, SERP::view)
        .default_font(iced::Font::MONOSPACE)
        .centered()
        .run()
}
pub struct SERP{
    screen: Screen 
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Login
}
impl Default for SERP {
    fn default() -> Self {
        Self {
            screen: Screen::Login,
        }
    }
}

impl SERP{
    fn title(&self) -> String{
        let screen = match self.screen{
            Screen::Login => "Login"
        };
        format!("{} - SERP", screen)
    }
    fn update(&mut self, event:Message){
        match event {
            Message::ButtonPressed => println!("Apenas pressionando alguma coisa")
        }
    }
    fn view(&self) -> Element<Message> {
        let controls = 
            row![]
                .push({padded_button("Next")
                    .on_press(Message::ButtonPressed)
                    });

        let screen = match self.screen {
            Screen::Login => self.login(),
        };
        let content: Element<_> = column![screen, controls,]
            .into();
        container(content).center_y(Fill).into()
    }
    fn login(&self) ->Column<Message>{
        Self::container("Welcome!")
            .push("apenas um teste, gatao")
    }

    fn container(title: &str) -> Column<'_, Message>{
        column![text(title).size(50)].spacing(20)
    }
}
fn padded_button<Message: Clone>(label:&str)-> Button<'_, Message> {
    button(text(label)).padding([12,24])
}









