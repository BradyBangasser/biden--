pub mod keywords;

pub mod language {
    use super::keywords;

    pub fn load_lang() {
        let mut used_words: Vec<&str> = vec![];
        for keyword in keywords::keywords::KEYWORDS {
            for word in keyword.keywords {
                if used_words.contains(word) {
                    panic!("The word '{}' is used more than once", word)
                }
                used_words.push(word)
            }
        }
    }
}