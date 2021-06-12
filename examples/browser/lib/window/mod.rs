pub mod history;
pub mod location;

use history::History;
use location::Location;

#[derive(Debug, Default, Clone)]
pub struct Window {
    pub history: History,
    pub location: Location,
}
