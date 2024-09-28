struct Route {
    children: Vec<Trie>
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
        for byte in string.as_bytes() {
            if let Some(child) = self.children.iter_mut().find(|child| child.value == *byte as char) {
                child.score += 1;
                child.route.push(&string[1..]);
            } else {
                let mut trie = Trie::new(*byte as char);
                trie.route.push(&string[1..]);
                self.children.push(trie);
            }
        }
    }

    fn compute_scores(&self) -> Vec<i32> {
        let mut scores: Vec<i32> = Vec::new();

        for child in &self.children {
            scores.push(child.compute_score());
        }

        scores
    }

    fn find_mut_child(&mut self, pattern: &str) -> Option<&mut Trie> {
        let objective = pattern.as_bytes()[0] as char;

        if pattern.len() > 0 {
            let child = self.children.iter_mut().find(|child| child.value == objective);

            if let Some(child) = child {
                child.find_mut_child(&pattern[1..])
            } else {
                None
            }
        } else {
            panic!("Pattern cannot be empty!")
        }
    }
}

impl Trie {
    fn new(value: char) -> Trie {
        Trie {
            score: 0,
            value,
            route: Route::new()
        }
    }

    fn compute_score(&self) -> i32 {
        if self.route.children.is_empty() {
            self.score
        } else {
            let mut score = self.score;

            for child in &self.route.children {
                score += child.compute_score();
            }

            score
        }
    }

    fn find_mut_child(&mut self, pattern: &str) -> Option<&mut Trie> {
        let length = pattern.len();

        if length > 1 {
            self.route.find_mut_child(pattern)
        } else if length == 1 {
            let target = pattern.as_bytes()[0] as char;

            if target == self.value {
                Some(self)
            } else {
                None
            }
        } else {
            Some(self)
        }
    }
}

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut route = Route::new();

    for word in &words {
        route.push(word);
    }

    route.compute_scores()
}


fn main() {
    println!("{:?}", sum_prefix_scores(vec![
        String::from("abc"),
        String::from("ab"),
        String::from("bc"),
        String::from("b")
    ]));
}
