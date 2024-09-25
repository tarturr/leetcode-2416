struct Route {
    children: Vec<Trie>
}

struct Trie {
    score: i32,
    value: char,
    children: Route
}

impl Route {
    fn new() -> Self {
        Route { children: Vec::new() }
    }

    fn push(&mut self, string: String) {
        for i in 0..string.len() {
            if let Some(child) = self.children.iter_mut().find(|child| child.value == string[i]) {
                child.value += 1;
                child.push(&string[1..]);
            } else {
                let mut trie = Trie::new(string[0]);
                trie.children.push(&string[1..]);
                self.children.push(trie);
            }
        }
    }
}

impl Trie {
    fn new(value: char) -> Trie {
        Trie {
            score: 0,
            value,
            children: Route::new()
        }
    }
}

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    /*
    let mut scores = vec![0; words.len()];

    for (i, string) in words.iter().enumerate() {
        for j in 0..(string.len()) {
            scores[i] += words.iter()
                    .filter(|&word| j < word.len() && string[..=j] == word[..=j])
                    .count() as i32;
        }
    }

    scores
    */
}


fn main() {
    println!("Hello, world!");
}
