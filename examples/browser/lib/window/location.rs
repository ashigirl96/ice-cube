use iced::text_input;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub href: String,
    pub state: text_input::State,
}
