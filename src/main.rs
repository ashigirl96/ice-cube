use iced::{Sandbox, Settings};

use browser_lib::Browser;

pub fn main() -> iced::Result {
    Browser::run(Settings {
        default_font: Some(include_bytes!("../fonts/Mamelon-5-Hi-Regular.otf")),
        ..Settings::default()
    })
}
