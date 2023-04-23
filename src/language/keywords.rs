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

        // Keywords for while
        Keyword {
            token: Constants::WHILE_T,
            keywords: &["while"]
        },

        // Keywords for for
        Keyword {
            token: Constants::FOR_T,
            keywords: &["for"]
        },

        // Keywords for print
        Keyword {
            token: Constants::PRINT_T,
            keywords: &["print"]
        },

        // Keywords for input
        Keyword {
            token: Constants::INPUT_T,
            keywords: &["input"]
        },

        // Keywords for force ending the program
        Keyword {
            token: Constants::TERMINATE_T,
            keywords: &["end"]
        },

        // Keywords for variable declaration
        Keyword {
            token: Constants::VAR_T,
            keywords: &["var"]
        },
    ];

    pub const OPERATORS: &[Operator] = &[
        // Operator for addition
        Operator {
            token: Constants::ADD_T,
            operators: &["+"]
        },

        // Operator for subtraction
        Operator {
            token: Constants::SUB_T,
            operators: &["-"]
        },

        // Operator for multiplication
        Operator {
            token: Constants::MULTI_T,
            operators: &["*"]
        },

        // Operator for division
        Operator {
            token: Constants::DIVIDE_T,
            operators: &["/"]
        },

        // Operator for modulus
        Operator {
            token: Constants::MOD_T,
            operators: &["%"]
        },

        // Operator for and
        Operator {
            token: Constants::AND_T,
            operators: &["&&"]
        },

        // Operator for or
        Operator {
            token: Constants::OR_T,
            operators: &["||"]
        },

        // Operator for not
        Operator {
            token: Constants::NOT_T,
            operators: &["!"]
        },

        // Operator for equals
        Operator {
            token: Constants::EQUALS_T,
            operators: &["=="]
        },

        // Operator for less than
        Operator {
            token: Constants::LESS_THAN_T,
            operators: &["<"]
        },

        // Operator for great than
        Operator {
            token: Constants::GREAT_THAN_T,
            operators: &[">"]
        },

        // Operator for dot
        Operator {
            token: Constants::DOT_T,
            operators: &["."]
        },

        // Operator for left bracket
        Operator {
            token: Constants::LEFT_BRACKET,
            operators: &["["]
        },

        // Operator for right bracket
        Operator {
            token: Constants::RIGHT_BRACKET,
            operators: &["]"]
        },

        // Operator for left curly bracket
        Operator {
            token: Constants::LEFT_CBRACKET,
            operators: &["{"]
        },

        // Operator for right curly bracket
        Operator {
            token: Constants::RIGHT_CBRACKET,
            operators: &["}"]
        },

        // Operator for left parenthesis
        Operator {
            token: Constants::LEFT_PAREN,
            operators: &["("]
        },

        // Operator for right parenthesis
        Operator {
            token: Constants::RIGHT_PAREN,
            operators: &[")"]
        },

        // Operator for variable assignment
        Operator {
            token: Constants::ASSIGN_T,
            operators: &["="]
        },
    ];

    pub const TERMINATOR_REGEX: &str = r"[\s\.!]";
    pub const NUMBER_REGEX: &str = r"[0-9]";
    pub const ALLOWED_CHARACTERS: &str = r"[a-z0-9A-Z_]";
}