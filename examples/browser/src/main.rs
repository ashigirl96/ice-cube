use iced::{Sandbox, Settings};

use process::BrowserProcess;

pub fn main() -> iced::Result {
    BrowserProcess::run(Settings {
        default_font: Some(include_bytes!("../../../fonts/Mamelon-5-Hi-Regular.otf")),
        ..Settings::default()
    })
}
