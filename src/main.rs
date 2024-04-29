use iced::{Command, executor,  Renderer, Theme};
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
        (PageSwitchingApp { screen: Screen::PageOne()} , Command::none())
    }

}
fn main() {
    println!("Hello, world!");
}
