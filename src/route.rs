use crate::trie::Trie;

pub struct Route {
    children: Vec<Trie>
}

impl Route {
    pub fn new() -> Self {
        Route { children: Vec::new() }
    }

    pub fn push(&mut self, string: &str) {
        if string.len() == 0 {
            return;
        }

        let current = string.as_bytes()[0] as char;

        if let Some(child) = self.children.iter_mut().find(|child| child.value == current) {
            child.score += 1;
            child.route.push(&string[1..]);
        } else {
            let mut trie = Trie::new(current);
            trie.route.push(&string[1..]);
            self.children.push(trie);
        }
    }

    pub fn find_child(&self, pattern: &str) -> Option<&Trie> {
        if pattern.len() > 0 {
            let objective = pattern.as_bytes()[0] as char;
            let child = self.children.iter().find(|child| child.value == objective);

            if let Some(child) = child {
                child.find_child(&pattern[1..])
            } else {
                None
            }
        } else {
            panic!("Pattern cannot be empty!")
        }
    }
}