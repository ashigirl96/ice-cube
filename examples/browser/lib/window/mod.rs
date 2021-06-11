mod history;

use history::History;

#[derive(Debug, Default, Clone)]
pub struct Window {
    pub history: History,
}

impl Window {
    fn new() -> Self {
        Self {
            history: History::default(),
        }
    }
}
