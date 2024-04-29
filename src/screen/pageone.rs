use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    ToNextPage,
    ToPrevPage,
}

#[derive(Debug, Clone)]
pub enum Event {
    SomeEvent,
}

pub struct PageOne;

impl PageOne {
    pub fn new() -> Self {
        PageOne
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match message {
            Message::ToNextPage => {
                println!("moving to next page");
            },
            Message::ToPrevPage => {
                println!("moving to the previous page");
            },
        }
        None
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        iced::widget::Container::new(iced::widget::Button::new("Next Page")).into()
    }
}
