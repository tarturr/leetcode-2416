struct Route {
    children: Vec<Trie>
}

struct Root {
    route: Route,
    words: Vec<String>
}

struct Trie {
    score: i32,
    value: char,
    route: Route
}

impl Route {
    fn new() -> Self {
        Route { children: Vec::new() }
    }

    fn push(&mut self, string: &str) {
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

    fn find_child(&self, pattern: &str) -> Option<&Trie> {
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

impl Root {
    fn new(words: Vec<String>) -> Self {
        let mut route = Route::new();

        for word in &words {
            route.push(&word)
        }

        Root {
            route,
            words
        }
    }

    fn compute_scores(&self) -> Vec<i32> {
        let mut scores: Vec<i32> = Vec::new();

        for word in &self.words {
            let mut score = 0;

            for i in 1..=word.len() {
                if let Some(trie) = self.route.find_child(&word[..i]) {
                    score += trie.score;
                }
            }

            scores.push(score);
        }

        scores
    }
}

impl Trie {
    fn new(value: char) -> Self {
        Trie {
            score: 1,
            value,
            route: Route::new()
        }
    }

    fn find_child(&self, pattern: &str) -> Option<&Trie> {
        let length = pattern.len();

        if length >= 1 {
            self.route.find_child(pattern)
        } else {
            Some(self)
        }
    }
}

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    Root::new(words).compute_scores()
}


fn main() {
    println!("{:?}", sum_prefix_scores(vec![
        String::from("abc"),
        String::from("ab"),
        String::from("bc"),
        String::from("b")
    ]));
}
