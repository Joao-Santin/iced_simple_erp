use iced::widget::{column, text};
use iced::widget::Column;
use iced::Element;

fn main()-> iced::Result {
    println!("Hello, world!");
    iced::application(SERP::title, SERP::update, SERP::view)
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
                .
        let screen = match self.screen {
            Screen::Login => self.login(),
        };
    }
    fn login(&self) ->Column<Message>{
        Self::container("Welcome!")
            .push("apenas um teste, gatão")
    }

    fn container(title: &str) -> Column<'_, Message>{
        column![text(title).size(50)].spacing(20)
    }
}
