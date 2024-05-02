use iced::{Element, Length};
use iced::widget::{Container, column, Space, Button};

#[derive(Debug, Clone)]
pub enum Message {
    ToPageTwo,
}

//single static page, is not supposed to do anything other than display a button that lets you move to the next page
pub struct PageOne;

impl PageOne {
    pub fn new() -> Self {
        PageOne
    }
    //I don't know when this is called..'
    pub fn update(&mut self, message: Message){
        match message {
            Message::ToPageTwo => {
            },
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        Container::new(
            column![
            Space::new(Length::Shrink, Length::Fill),
            Button::new("To page 2").on_press(Message::ToPageTwo),
            ]
        )
        .padding(20.)
        .center_x()
        .width(Length::Fill)
        .into()
    }
}
