use crate::route::Route;

pub struct Trie {
    pub score: i32,
    pub value: char,
    pub route: Route
}

impl Trie {
    pub fn new(value: char) -> Self {
        Trie {
            score: 1,
            value,
            route: Route::new()
        }
    }

    pub fn find_child(&self, pattern: &str) -> Option<&Trie> {
        let length = pattern.len();

        if length >= 1 {
            self.route.find_child(pattern)
        } else {
            Some(self)
        }
    }
}