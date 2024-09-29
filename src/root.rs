use crate::route::Route;

pub struct Root {
    route: Route,
    words: Vec<String>
}

impl Root {
    pub fn new(words: Vec<String>) -> Self {
        let mut route = Route::new();

        for word in &words {
            route.push(&word)
        }

        Root {
            route,
            words
        }
    }

    pub fn compute_scores(&self) -> Vec<i32> {
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