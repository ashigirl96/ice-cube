use iced::{
    button, Background, Button, Color, Column, Element, HorizontalAlignment, Row, Sandbox,
    Settings, Text, Vector,
};

fn main() -> iced::Result {
    Pusher::run(Settings::default())
}

struct Pusher {
    button: button::State,
    block_size: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Push,
}

impl Sandbox for Pusher {
    type Message = Message;

    fn new() -> Self {
        Self {
            button: button::State::new(),
            block_size: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Pusher")
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::Push => {
                self.block_size += 1;
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let Self {
            button: btn,
            block_size,
        } = self;
        let mut blocks = Column::new();

        blocks = blocks.push(
            button(btn, "hello")
                .on_press(Message::Push)
                .style(Button2::Primary),
        );

        for _ in 0..*block_size {
            blocks = blocks.push(Text::new("Hello"))
        }
        blocks.into()
    }
}

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

pub enum Button2 {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button2 {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button2::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                Button2::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
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
