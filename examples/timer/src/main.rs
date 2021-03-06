use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Executor,
    Font, HorizontalAlignment, Length, Row, Settings, Text,
};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    TimerFront::run(settings);
}

struct TimerFront {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
    tick_state: TickState,
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Stop,
    Reset,
}

impl Application for TimerFront {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
                tick_state: TickState::Stopped,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Timer")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
            }
            Message::Reset => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let duration_text = "00:00:00.00";

        let start_stop_text = match self.tick_state {
            TickState::Stopped => {
                Text::new("Start").horizontal_alignment(HorizontalAlignment::Center)
            }
            TickState::Ticking => {
                Text::new("Stop").horizontal_alignment(HorizontalAlignment::Center)
            }
        };

        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        let tick_text = Text::new(duration_text).size(60);

        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
            .min_width(80)
            .on_press(start_stop_message);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset").horizontal_alignment(HorizontalAlignment::Center),
        )
        .min_width(80)
        .on_press(Message::Reset);

        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

enum TickState {
    Stopped,
    Ticking,
}
