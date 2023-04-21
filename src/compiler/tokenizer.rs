pub mod tokenizer {
    use regex::Regex;

    use crate::{language::keywords::keywords::{self, KEYWORDS, OPERATORS}, compiler::constants::constants::Constants};

    #[derive(Debug)]
    pub struct Token {
        pub token: i32,
        pub val: Box<str>,
        pub line: i128,
    }

    // wilford among potion 3am gone wrong prank gone cops called walmarrt all night challenge caps lock is on haha

    pub fn tokenize(src: &str) {
        let end_regex: Regex = Regex::new(keywords::LINE_TERMINATORS_REGEX).unwrap();
        let number_regex: Regex = Regex::new(r"[e\.]").unwrap();
        let user_number_regex: Regex = Regex::new(keywords::NUMBER_REGEX).unwrap();
        let valid_characters: Regex = Regex::new(keywords::ALLOWED_CHARACTERS).unwrap();
        let split = str::split(src, "");
        let vec: Vec<&str> = split.collect();
        let mut tokens: Vec<Token> = vec![];

        let mut line = 1;

        let mut i = 0;
        while i < vec.len() {
            let token = vec[i];
            if token == "\n" {
                line += 1;
                i += 1;
                continue;
            }

            if user_number_regex.is_match(token) {
                let mut number_constructor: Vec<&str> = vec![];

                while user_number_regex.is_match(vec[i]) || (number_regex.is_match(vec[i]) && user_number_regex.is_match(vec[i + 1])) {
                    number_constructor.push(vec[i]);
                    i += 1;
                }

                let complete_number = number_constructor.join("");

                if number_regex.is_match(&complete_number) {
                    tokens.push(Token { token: Constants::DOUBLE_T, val: complete_number.into(), line })
                } else {
                    tokens.push(Token { token: Constants::INT_T, val: complete_number.into(), line })
                }

                continue;
            }

            if end_regex.is_match(token) {
                i += 1;
                continue;
            }

            if token == "\"" || token == "'" || token == "`" {
                let mut string_builder: Vec<&str> = vec![];

                while vec[i + 1] != token && vec[i] != "\\" {
                    string_builder.push(vec[i + 1]);
                    i += 1;

                    if i + 2 > vec.len() {
                        todo!("Unterminated quote error")
                    }
                }

                tokens.push(Token { token: Constants::STRING_T, val: string_builder.join("").into(), line });

                i += 2;
                continue;
            }
            
            if valid_characters.is_match(token) {
                let mut word_builder: Vec<&str> = vec![];

                while valid_characters.is_match(vec[i]) {
                    word_builder.push(vec[i]);
                    i += 1;
                }

                let word = word_builder.join("");

                let mut token = Constants::WORD_T;

                for keyword in KEYWORDS {
                    if keyword.keywords.contains(&word.as_str()) {
                        token = keyword.token
                    }
                }

                tokens.push(Token { token: token, val: word.into(), line });
                continue;
            }

            let mut operator_constructor: Vec<&str> = vec![];

            while i < vec.len() && !valid_characters.is_match(vec[i]) {
                operator_constructor.push(vec[i]);
                i += 1;
            }

            let string_operator = operator_constructor.join("");
            let built_operator: &str = string_operator.as_str();

            for operator in OPERATORS {
                if operator.operators.contains(&built_operator) {
                    tokens.push(Token { token: operator.token, val: built_operator.into(), line })
                }
            }
        }
        println!("{:#?}", tokens)
    }
}