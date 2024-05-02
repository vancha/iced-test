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

//the main applicaiton struct, holding it's current screen
struct PageSwitchingApp {
    screen: Screen,
}

impl Application for PageSwitchingApp {
    type Renderer = Renderer;
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();


    fn new(_flags: Self::Flags) -> (PageSwitchingApp, Command<Self::Message>) {
        (PageSwitchingApp { screen: Screen::PageOne(screen::PageOne::new())} , Command::none())
    }


    fn title(&self) -> String {
        String::from("switching the pages")
    }

    //This is where things seem to get complicated
    fn update(&mut self, message: Self::Message) -> iced::Command<Message>{
        //here we get a message originating from our view
        match message {
            //if said message was sent form page one, the view made sure to wrap that message with a message::pageone
            Message::PageOne(_) => {
                //pageone has only a single message, which is the movetopagetwo message
                self.screen = Screen::PageTwo(screen::PageTwo::new());
            },
            //pagetwo has two messages
            Message::PageTwo(msg) => {
                //here we are figuring out which message we are currently receiving
                match msg {
                    //this is the message that lets you navigate back to page one.
                    pagetwo::Message::ToPageOne => {
                        self.screen = Screen::PageOne(screen::PageOne::new());
                    },
                    //and here we decide that if it has more messages, we will let it handle them itself.
                    _ => {
                        //we get the screen
                        let Screen::PageTwo(page) = &mut self.screen else { return Command::none(); };
                        //and give it back it's own message
                        page.update(msg);
                    },
                }
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
