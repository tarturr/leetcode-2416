mod route;
mod root;
mod trie;

use crate::root::Root;

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
