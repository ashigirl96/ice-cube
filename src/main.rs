use iced::{Sandbox, Settings};

use tour_lib::Tour;

pub fn main() -> iced::Result {
    Tour::run(Settings {
        default_font: Some(include_bytes!("../fonts/Mamelon-5-Hi-Regular.otf")),
        ..Settings::default()
    })
}
