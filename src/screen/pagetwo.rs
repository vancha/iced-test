use iced::{Element, Length};
use iced::widget::{Container, column, row, Text, Space, Button};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateText,
    ToPageOne,
}

#[derive(Debug, Clone)]
pub enum Event {
    SomeEvent,
}

pub struct PageTwo {
    page_description : String,
}

impl PageTwo {
    pub fn new() -> Self {
        PageTwo { page_description: "This is some content for page two.".into() }
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::UpdateText => {
                self.page_description = String::from("The text on this page is now updated!");
            },
            //this message will not be handled here, because it is never passed back from the main application.
            Message::ToPageOne => {
                println!("moving to the previous page");
            },
        }
        None
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        Container::new(
            column![
                Text::new(self.page_description.clone()),
                Space::new(Length::Shrink, Length::Fill),
                row![
                    Button::new("Change text").on_press(Message::UpdateText),
                    Button::new("To page 1").on_press(Message::ToPageOne),
                ]
                .spacing(20.)
            ]
        )
        .padding(20.)
        .center_x()
        .width(Length::Fill)
        .into()
    }
}
