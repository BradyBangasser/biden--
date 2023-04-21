pub mod keywords {
    use crate::compiler::constants::constants::Constants;

    pub struct Keyword {
        pub token: i32,
        pub keywords: &'static [&'static str]
    }

    pub struct Operator {
        pub token: i32,
        pub operators: &'static [&'static str]
    }

    pub const KEYWORDS: &[Keyword] = &[

        // Keywords for false
        Keyword {
            token: Constants::FALSE_T,
            keywords: &["false"]
        },
        // Keywords for true
        Keyword {
            token: Constants::TRUE_T,
            keywords: &["true"]
        },

        // Keywords for if
        Keyword {
            token: Constants::IF_T,
            keywords: &["if"]
        },

        // Keywords for elif
        Keyword {
            token: Constants::ELIF_T,
            keywords: &["elif"]
        },

        // Keywords for else
        Keyword {
            token: Constants::ELSE_T,
            keywords: &["else"]
        },
    ];

    pub const OPERATORS: &[Operator] = &[
        Operator {
            token: Constants::ADD_T,
            operators: &["-"]
        }
    ];

    pub const LINE_TERMINATORS_REGEX: &str = r"[\s\.!]";
    pub const NUMBER_REGEX: &str = r"[0-9]";
    pub const ALLOWED_CHARACTERS: &str = r"[a-z0-9A-Z_]";
}