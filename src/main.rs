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
        for i in 0..string.len() {
            if let Some(child) = self.children.iter_mut().find(|child| child.value == string.as_bytes()[i] as char) {
                child.score += 1;
                child.route.push(&string[1..]);
            } else {
                let mut trie = Trie::new(string.as_bytes()[0] as char);
                trie.route.push(&string[1..]);
                self.children.push(trie);
            }
        }
    }

    fn score(&self) -> Vec<i32> {
        let mut scores: Vec<i32> = Vec::new();

        for child in &self.children {
            scores.push(child.score());
        }

        scores
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

    fn score(&self) -> i32 {
        if self.route.children.is_empty() {
            self.score
        } else {
            let mut score = self.score;

            for child in &self.route.children {
                score += child.score();
            }

            score
        }
    }
}

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut route = Route::new();

    for word in &words {
        route.push(word);
    }

    route.score()
}


fn main() {
    println!("{:?}", sum_prefix_scores(vec![
        String::from("abc"),
        String::from("ab"),
        String::from("bc"),
        String::from("b")
    ]));
}
