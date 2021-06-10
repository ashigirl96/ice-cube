mod window;

use iced::{
    button, scrollable, text_input, Button, Color, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Scrollable, Space, Text,
};

use window::Window;

pub struct BrowserProcess {
    back_button: button::State,
    next_button: button::State,
    reload_button: button::State,
    window: Window,
    debug: bool,
}

struct Location {}

#[derive(Debug, Clone)]
pub enum ProcessMessage {
    BackPressed,
    NextPressed,
    ReloadPressed,
}

impl Sandbox for BrowserProcess {
    type Message = ProcessMessage;

    fn new() -> Self {
        Self {
            back_button: button::State::new(),
            next_button: button::State::new(),
            reload_button: button::State::new(),
            window: Window::default(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        "wabi-sabi".to_string()
    }

    fn update(&mut self, event: Self::Message) {
        // match event {
        //     ProcessMessage::BackPressed => {
        //         if let Some(url) = self.history.pop_back() {
        //             self.location.url = url
        //         };
        //     }
        //     ProcessMessage::NextPressed => {
        //         if let Some(url) = self.history.pop_back() {
        //             self.location.url = url
        //         };
        //     }
        //     ProcessMessage::ReloadPressed => {}
        // }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        todo!()
    }
}
