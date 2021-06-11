use iced::{
    slider, text_input, Checkbox, Color, Column, Element, HorizontalAlignment, Length, Radio, Row,
    Slider, Text, TextInput,
};

use crate::language::Language;
use crate::layout::Layout;
use crate::step_message::StepMessage;

pub enum Step {
    TextInput {
        value: String,
        is_secure: bool,
        state: text_input::State,
    },
    Debugger,
    End,
}

impl<'a> Step {
    pub fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        println!("Step#update {:?}", msg);
        match msg {
            StepMessage::InputChanged(new_value) => {
                if let Step::TextInput { value, .. } = self {
                    *value = new_value;
                }
            }
            StepMessage::ToggleSecureInput(toggle) => {
                if let Step::TextInput { is_secure, .. } = self {
                    *is_secure = toggle;
                }
            }
            _ => {}
        };
    }

    pub fn view(&mut self, debug: bool) -> Element<StepMessage> {
        match self {
            Step::TextInput {
                value,
                is_secure,
                state,
            } => {
                println!("step#view");
                Self::text_input(value, state)
            }
            _ => panic!("FACK"),
        }
        .into()
    }

    pub fn title(&self) -> &str {
        match self {
            _ => "FACK",
            Step::TextInput { .. } => "Text input",
            Step::Debugger => "Debugger",
            Step::End => "End",
        }
    }

    pub fn can_continue(&self) -> bool {
        match self {
            Step::TextInput { value, .. } => !value.is_empty(),
            Step::Debugger => true,
            Step::End => false,
            _ => false,
        }
    }

    pub fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    pub fn text_input(value: &str, state: &'a mut text_input::State) -> Column<'a, StepMessage> {
        println!("view > text_input");
        let text_input = TextInput::new(
            state,
            "Type something to continue...",
            value,
            StepMessage::InputChanged,
        )
        .padding(10)
        .size(30);

        Column::new()
            .spacing(20)
            .push(Text::new("FACK").size(50))
            .push(text_input)
            .push(Text::new(value))
    }
}

fn color_slider(
    state: &mut slider::State,
    component: f32,
    update: impl Fn(f32) -> Color + 'static,
) -> Slider<f64, StepMessage> {
    Slider::new(state, 0.0..=1.0, f64::from(component), move |c| {
        StepMessage::TextColorChanged(update(c as f32))
    })
    .step(0.01)
}
