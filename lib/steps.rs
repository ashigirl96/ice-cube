use iced::{
    slider, text_input, Checkbox, Color, Column, Container, Element, HorizontalAlignment, Image,
    Length, Radio, Row, Scrollable, Settings, Slider, Space, Text, TextInput,
};

use crate::language::Language;
use crate::layout::Layout;

pub struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    pub fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::Slider {
                    state: slider::State::new(),
                    value: 50,
                },
                Step::RowsAndColumns {
                    layout: Layout::Row,
                    spacing_slider: slider::State::new(),
                    spacing: 20,
                },
                Step::Text {
                    size_slider: slider::State::new(),
                    size: 30,
                    color_sliders: [slider::State::new(); 3],
                    color: Color::BLACK,
                },
                Step::Radio { selection: None },
                Step::Image {
                    width: 300,
                    slider: slider::State::new(),
                },
                Step::Scrollable,
                Step::TextInput {
                    value: String::new(),
                    is_secure: false,
                    state: text_input::State::new(),
                },
                Step::Debugger,
                Step::End,
            ],
            current: 0,
        }
    }

    pub fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        self.steps[self.current].update(msg, debug);
    }

    pub fn view(&mut self, debug: bool) -> Element<StepMessage> {
        self.steps[self.current].view(debug)
    }

    pub fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    pub fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    pub fn has_previous(&self) -> bool {
        self.current > 0
    }

    pub fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
    }

    pub fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

enum Step {
    Welcome,
    Slider {
        state: slider::State,
        value: u8,
    },
    RowsAndColumns {
        layout: Layout,
        spacing_slider: slider::State,
        spacing: u16,
    },
    Text {
        size_slider: slider::State,
        size: u16,
        color_sliders: [slider::State; 3],
        color: Color,
    },
    Radio {
        selection: Option<Language>,
    },
    Image {
        width: u16,
        slider: slider::State,
    },
    Scrollable,
    TextInput {
        value: String,
        is_secure: bool,
        state: text_input::State,
    },
    Debugger,
    End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    SliderChanged(u8),
    LayoutChanged(Layout),
    SpacingChanged(u16),
    TextSizeChanged(u16),
    TextColorChanged(Color),
    LanguageSelected(Language),
    ImageWidthChanged(u16),
    InputChanged(String),
    ToggleSecureInput(bool),
    DebugToggled(bool),
}

impl<'a> Step {
    pub fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        match msg {
            StepMessage::DebugToggled(value) => {
                if let Step::Debugger = self {
                    *debug = value;
                }
            }
            StepMessage::LanguageSelected(language) => {
                if let Step::Radio { selection } = self {
                    *selection = Some(language);
                }
            }
            StepMessage::SliderChanged(new_value) => {
                if let Step::Slider { value, .. } = self {
                    *value = new_value;
                }
            }
            StepMessage::TextSizeChanged(new_size) => {
                if let Step::Text { size, .. } = self {
                    *size = new_size;
                }
            }
            StepMessage::TextColorChanged(new_color) => {
                if let Step::Text { color, .. } = self {
                    *color = new_color;
                }
            }
            StepMessage::LayoutChanged(new_layout) => {
                if let Step::RowsAndColumns { layout, .. } = self {
                    *layout = new_layout;
                }
            }
            StepMessage::SpacingChanged(new_spacing) => {
                if let Step::RowsAndColumns { spacing, .. } = self {
                    *spacing = new_spacing;
                }
            }
            StepMessage::ImageWidthChanged(new_width) => {
                if let Step::Image { width, .. } = self {
                    *width = new_width;
                }
            }
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
        };
    }

    pub fn title(&self) -> &str {
        match self {
            Step::Welcome => "Welcome",
            Step::Radio { .. } => "Radio button",
            Step::Slider { .. } => "Slider",
            Step::Text { .. } => "Text",
            Step::Image { .. } => "Image",
            Step::RowsAndColumns { .. } => "Rows and columns",
            Step::Scrollable => "Scrollable",
            Step::TextInput { .. } => "Text input",
            Step::Debugger => "Debugger",
            Step::End => "End",
        }
    }

    pub fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Radio { selection } => *selection == Some(Language::Rust),
            Step::Slider { .. } => true,
            Step::Text { .. } => true,
            Step::Image { .. } => true,
            Step::RowsAndColumns { .. } => true,
            Step::Scrollable => true,
            Step::TextInput { value, .. } => !value.is_empty(),
            Step::Debugger => true,
            Step::End => false,
        }
    }

    pub fn view(&mut self, debug: bool) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome(),
            Step::Radio { selection } => Self::radio(*selection),
            Step::Slider { state, value } => Self::slider(state, *value),
            Step::Text {
                size_slider,
                size,
                color_sliders,
                color,
            } => Self::text(size_slider, *size, color_sliders, *color),
            Step::Image { width, slider } => Self::image(*width, slider),
            Step::RowsAndColumns {
                layout,
                spacing_slider,
                spacing,
            } => Self::rows_and_columns(*layout, spacing_slider, *spacing),
            Step::Scrollable => Self::scrollable(),
            Step::TextInput {
                value,
                is_secure,
                state,
            } => Self::text_input(value, *is_secure, state),
            Step::Debugger => Self::debugger(debug),
            Step::End => Self::end(),
        }
        .into()
    }

    pub fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    pub fn welcome() -> Column<'a, StepMessage> {
        Self::container("Welcome!")
            .push(Text::new(
                "This is a simple tour meant to showcase a bunch of widgets \
                 that can be easily implemented on top of Iced.",
            ))
            .push(Text::new(
                "Iced is a cross-platform GUI library for Rust focused on \
                 simplicity and type-safety. It is heavily inspired by Elm.",
            ))
            .push(Text::new(
                "It was originally born as part of Coffee, an opinionated \
                 2D game engine for Rust.",
            ))
            .push(Text::new(
                "On native platforms, Iced provides by default a renderer \
                 built on top of wgpu, a graphics library supporting Vulkan, \
                 Metal, DX11, and DX12.",
            ))
            .push(Text::new(
                "Additionally, this tour can also run on WebAssembly thanks \
                 to dodrio, an experimental VDOM library for Rust.",
            ))
            .push(Text::new(
                "You will need to interact with the UI in order to reach the \
                 end!",
            ))
    }

    pub fn slider(state: &'a mut slider::State, value: u8) -> Column<'a, StepMessage> {
        Self::container("Slider")
            .push(Text::new(
                "A slider allows you to smoothly select a value from a range \
                 of values.",
            ))
            .push(Text::new(
                "The following slider lets you choose an integer from \
                 0 to 100:",
            ))
            .push(Slider::new(
                state,
                0..=100,
                value,
                StepMessage::SliderChanged,
            ))
            .push(
                Text::new(&value.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    pub fn rows_and_columns(
        layout: Layout,
        spacing_slider: &'a mut slider::State,
        spacing: u16,
    ) -> Column<'a, StepMessage> {
        let row_radio = Radio::new(Layout::Row, "Row", Some(layout), StepMessage::LayoutChanged);

        let column_radio = Radio::new(
            Layout::Column,
            "Column",
            Some(layout),
            StepMessage::LayoutChanged,
        );

        let layout_section: Element<_> = match layout {
            Layout::Row => Row::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
            Layout::Column => Column::new()
                .spacing(spacing)
                .push(row_radio)
                .push(column_radio)
                .into(),
        };

        let spacing_section = Column::new()
            .spacing(10)
            .push(Slider::new(
                spacing_slider,
                0..=80,
                spacing,
                StepMessage::SpacingChanged,
            ))
            .push(
                Text::new(&format!("{} px", spacing))
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            );

        Self::container("Rows and columns")
            .spacing(spacing)
            .push(Text::new(
                "Iced uses a layout model based on flexbox to position UI \
                 elements.",
            ))
            .push(Text::new(
                "Rows and columns can be used to distribute content \
                 horizontally or vertically, respectively.",
            ))
            .push(layout_section)
            .push(Text::new(
                "You can also easily change the spacing between elements:",
            ))
            .push(spacing_section)
    }

    pub fn text(
        size_slider: &'a mut slider::State,
        size: u16,
        color_sliders: &'a mut [slider::State; 3],
        color: Color,
    ) -> Column<'a, StepMessage> {
        let size_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("You can change its size:"))
            .push(Text::new(&format!("This text is {} pixels", size)).size(size))
            .push(Slider::new(
                size_slider,
                10..=70,
                size,
                StepMessage::TextSizeChanged,
            ));

        let [red, green, blue] = color_sliders;

        let color_sliders = Row::new()
            .spacing(10)
            .push(color_slider(red, color.r, move |r| Color { r, ..color }))
            .push(color_slider(green, color.g, move |g| Color { g, ..color }))
            .push(color_slider(blue, color.b, move |b| Color { b, ..color }));

        let color_section = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("And its color:"))
            .push(Text::new(&format!("{:?}", color)).color(color))
            .push(color_sliders);

        Self::container("Text")
            .push(Text::new(
                "Text is probably the most essential widget for your UI. \
                 It will try to adapt to the dimensions of its container.",
            ))
            .push(size_section)
            .push(color_section)
    }

    pub fn radio(selection: Option<Language>) -> Column<'a, StepMessage> {
        let question = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Iced is written in...").size(24))
            .push(Language::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, language| {
                    choices.push(Radio::new(
                        language,
                        language,
                        selection,
                        StepMessage::LanguageSelected,
                    ))
                },
            ));

        Self::container("Radio button")
            .push(Text::new(
                "A radio button is normally used to represent a choice... \
                 Surprise test!",
            ))
            .push(question)
            .push(Text::new(
                "Iced works very well with iterators! The list above is \
                 basically created by folding a column over the different \
                 choices, creating a radio button for each one of them!",
            ))
    }

    pub fn image(width: u16, slider: &'a mut slider::State) -> Column<'a, StepMessage> {
        Self::container("Image")
            .push(Text::new("An image that tries to keep its aspect ratio."))
            .push(Slider::new(
                slider,
                100..=500,
                width,
                StepMessage::ImageWidthChanged,
            ))
            .push(
                Text::new(&format!("Width: {} px", width.to_string()))
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    pub fn scrollable() -> Column<'a, StepMessage> {
        Self::container("Scrollable")
            .push(Text::new(
                "Iced supports scrollable content. Try it out! Find the \
                 button further below.",
            ))
            .push(Text::new("Tip: You can use the scrollbar to scroll down faster!").size(16))
            .push(Column::new().height(Length::Units(4096)))
            .push(
                Text::new("You are halfway there!")
                    .width(Length::Fill)
                    .size(30)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(Column::new().height(Length::Units(4096)))
            .push(
                Text::new("You made it!")
                    .width(Length::Fill)
                    .size(50)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    pub fn text_input(
        value: &str,
        is_secure: bool,
        state: &'a mut text_input::State,
    ) -> Column<'a, StepMessage> {
        let text_input = TextInput::new(
            state,
            "Type something to continue...",
            value,
            StepMessage::InputChanged,
        )
        .padding(10)
        .size(30);
        Self::container("Text input")
            .push(Text::new(
                "Use a text input to ask for different kinds of information.",
            ))
            .push(if is_secure {
                text_input.password()
            } else {
                text_input
            })
            .push(Checkbox::new(
                is_secure,
                "Enable password mode",
                StepMessage::ToggleSecureInput,
            ))
            .push(Text::new(
                "A text input produces a message every time it changes. It is \
                 very easy to keep track of its contents:",
            ))
            .push(
                Text::new(if value.is_empty() {
                    "You have not typed anything yet..."
                } else {
                    value
                })
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
            )
    }

    pub fn debugger(debug: bool) -> Column<'a, StepMessage> {
        Self::container("Debugger")
            .push(Text::new(
                "You can ask Iced to visually explain the layouting of the \
                 different elements comprising your UI!",
            ))
            .push(Text::new(
                "Give it a shot! Check the following checkbox to be able to \
                 see element boundaries.",
            ))
            .push(if cfg!(target_arch = "wasm32") {
                Element::new(
                    Text::new("Not available on web yet!")
                        .color([0.7, 0.7, 0.7])
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
            } else {
                Element::new(Checkbox::new(
                    debug,
                    "Explain layout",
                    StepMessage::DebugToggled,
                ))
            })
            .push(Text::new("Feel free to go back and take a look."))
    }

    pub fn end() -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push(Text::new(
                "This tour will be updated as more features are added.",
            ))
            .push(Text::new("Make sure to keep an eye on it!"))
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
