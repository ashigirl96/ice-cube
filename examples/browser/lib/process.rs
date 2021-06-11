use iced::{
    button, scrollable, text_input, Background, Button, Color, Column, Container, Element,
    HorizontalAlignment, Length, Row, Sandbox, Scrollable, Space, Text, TextInput, Vector,
    VerticalAlignment,
};
use iced_native::Widget;

use window::Window;

mod window;

pub struct BrowserProcess {
    back_button: button::State,
    next_button: button::State,
    reload_button: button::State,
    path_input: PathInputState,
    window: Window,
    debug: bool,
}

struct Location {}

#[derive(Debug, Clone)]
pub enum ProcessMessage {
    BackPressed,
    NextPressed,
    ReloadPressed,
    InputPath(String),
    Enter,
}

#[derive(Debug, Default)]
struct PathInputState {
    value: String,
    state: text_input::State,
}

impl Sandbox for BrowserProcess {
    type Message = ProcessMessage;

    fn new() -> Self {
        Self {
            back_button: button::State::new(),
            next_button: button::State::new(),
            reload_button: button::State::new(),
            path_input: PathInputState::default(),
            window: Window::default(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        "wabi-sabi".to_string()
    }

    fn update(&mut self, event: Self::Message) {
        match event {
            ProcessMessage::BackPressed => {
                self.window.history.back();
                println!("history.ブラウザバック {:?}", &self.window.history.path());
            }
            ProcessMessage::NextPressed => {
                self.window.history.forward();
                println!("history.forward {:?}", &self.window.history.path());
            }
            ProcessMessage::InputPath(path) => {
                self.path_input.value = path;
            }
            ProcessMessage::Enter => {
                self.window.history.push(&self.path_input.value);
                println!("Enter!! path is {:?}", &self.window.history);
            }
            _ => {}
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let Self {
            back_button,
            next_button,
            path_input,
            window,
            ..
        } = self;

        let mut controls = Row::new();

        let mut back = button(back_button, "←").style(Buttons::Secondary);
        if !window.history.no_back() {
            back = back.on_press(ProcessMessage::BackPressed)
        }
        let mut next = button(next_button, "→").style(Buttons::Secondary);
        if !window.history.no_next() {
            next = next.on_press(ProcessMessage::NextPressed)
        }
        controls = controls.push(back);
        controls = controls.push(next);
        let PathInputState { value, state } = path_input;
        controls = controls.push(text_input(state, value));

        Container::new(controls).into()
    }
}

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    let btn = Button::new(
        state,
        Text::new(label)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center),
    )
    .padding(10)
    .width(Length::Units(40))
    .height(Length::Units(30));
    btn
}

fn text_input<'a>(state: &'a mut text_input::State, value: &str) -> Column<'a, ProcessMessage> {
    let text_input = TextInput::new(state, "URL...", value, ProcessMessage::InputPath)
        .padding(10)
        .width(Length::Fill)
        .on_submit(ProcessMessage::Enter);

    let txt_inp = Column::new().push(text_input);
    txt_inp
}

pub enum Buttons {
    Primary,
    Secondary,
}

impl button::StyleSheet for Buttons {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Buttons::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                Buttons::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
            })),
            border_radius: 12.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}
