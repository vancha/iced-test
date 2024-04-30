use iced::{Command, Element, executor,  Renderer, Theme};
use iced::widget::{container};
use iced::advanced::{Application};

pub mod screen;
use screen::{pageone, pagetwo };


pub enum Screen {
    PageOne(screen::PageOne),
    PageTwo(screen::PageTwo),
}

#[derive(Debug)]
pub enum Message {
    PageOne(pageone::Message),
    PageTwo(pagetwo::Message),
}

struct PageSwitchingApp {
    screen: Screen,
}

impl Application for PageSwitchingApp {
    type Renderer = Renderer;
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (PageSwitchingApp, Command<Self::Message>) {
        (PageSwitchingApp { screen: Screen::PageOne(screen::PageOne::new())} , Command::none())
    }

    fn title(&self) -> String {
        String::from("switching the pages")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message>{
        match message {
            Message::PageOne(msg) => {
                self.screen = Screen::PageTwo(screen::PageTwo::new());
                println!("Message from page 1");
            },
            Message::PageTwo(msg) => {
                self.screen = Screen::PageOne(screen::PageOne::new());
                println!("Message from page 2");
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
       let screen = match &self.screen {
            Screen::PageOne(pageone) => pageone.view().map(Message::PageOne),
            Screen::PageTwo(pagetwo) => pagetwo.view().map(Message::PageTwo),
        };
        container(screen).into()
    }
}
fn main() -> iced::Result {
    PageSwitchingApp::run(iced::Settings::default())
}
