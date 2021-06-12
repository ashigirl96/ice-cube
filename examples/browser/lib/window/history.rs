use std::cmp::{max, min};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct History {
    history: VecDeque<String>,
    current: usize,
}

impl Default for History {
    fn default() -> Self {
        let mut history = VecDeque::new();
        history.push_back("wabi://new-tab-page".to_string());
        Self {
            history,
            current: 1,
        }
    }
}

impl History {
    pub fn forward(&mut self) {
        self.current = min(self.current + 1, self.length())
    }

    pub fn back(&mut self) {
        self.current = max(self.current - 1, 1)
    }

    pub fn push(&mut self, path: &str) {
        if self.current < self.length() {
            self.history.drain(self.current..);
        }
        self.history.push_back(path.to_string());
        self.current = self.length();
    }

    pub fn path(&self) -> String {
        match self.history.get(self.index()) {
            Some(path) => path.clone(),
            None => "".to_string(),
        }
    }

    pub fn no_back(&self) -> bool {
        self.index() == 0
    }

    pub fn no_next(&self) -> bool {
        self.current == self.length()
    }

    pub fn length(&self) -> usize {
        self.history.len()
    }

    fn index(&self) -> usize {
        self.current - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> History {
        let mut history = History::default();
        history.push("1");
        history.push("2");
        history.push("3");
        history
    }

    #[test]
    fn test() {
        let mut history = init();
        history.back();
        history.back();
        history.back();
        assert_eq!(history.path(), "wabi://new-tab-page".to_string());
        history.forward();
        assert_eq!(history.path(), "1".to_string());
        history.push("4");
        assert_eq!(history.path(), "4".to_string());
        history.back();
        assert_eq!(history.path(), "1".to_string());
        history.forward();
        history.forward();
        assert_eq!(history.path(), "4".to_string());
    }

    #[test]
    fn test_path() {
        let mut history = History::default();
        assert_eq!(history.path(), "wabi://new-tab-page".to_string());
        history.push("1");
        assert_eq!(history.path(), "1".to_string());
    }

    #[test]
    fn test_push() {
        let history = init();
        assert_eq!(history.path(), "3".to_string());
        assert_eq!(history.length(), 4);
    }

    #[test]
    fn test_back() {
        let mut history = init();
        history.back();
        assert_eq!(history.path(), "2".to_string());
        history.back();
        history.back();
        history.back();
        assert_eq!(history.path(), "wabi://new-tab-page".to_string());
        assert_eq!(history.length(), 4);
    }

    #[test]
    fn test_forward() {
        let mut history = init();
        history.back();
        history.forward();
        assert_eq!(history.path(), "3".to_string());
        history.forward();
        assert_eq!(history.path(), "3".to_string());
        assert_eq!(history.length(), 4);
    }
}
