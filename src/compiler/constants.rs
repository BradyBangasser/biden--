pub mod constants {
    pub struct Constants;
    impl Constants {
        pub const FALSE_T: i32 = 0;
        pub const TRUE_T: i32 = 1;

        pub const ADD_T: i32 = 20;
        pub const SUB_T: i32 = 21;
        pub const MULTI_T: i32 = 22;
        pub const DIVIDE_T: i32 = 23;
        pub const MOD_T: i32 = 24; // I don't know how to spell modulus

        pub const IF_T: i32 = 30;
        pub const ELIF_T: i32 = 31;
        pub const ELSE_T: i32 = 32;
        pub const AND_T: i32 = 33;
        pub const OR_T: i32 = 34;
        pub const NOT_T: i32 = 35;
        pub const EQUALS_T: i32 = 36;
        pub const LESS_THAN_T: i32 = 37;
        pub const GREAT_THAN_T: i32 = 38;

        pub const DOT_T: i32 = 40;
        pub const LEFT_BRACKET: i32 = 41;
        pub const RIGHT_BRACKET: i32 = 42;
        pub const LEFT_CBRACKET: i32 = 43;
        pub const RIGHT_CBRACKET: i32 = 44;
        pub const LEFT_PAREN: i32 = 45;
        pub const RIGHT_PAREN: i32 = 46;

        pub const WHILE_T: i32 = 50;
        pub const FOR_T: i32 = 51;
        pub const PRINT_T: i32 = 52;
        pub const INPUT_T: i32 = 53;
        pub const TERMINATE_T: i32 = 54;

        pub const STRING_T: i32 = 100; // Tokenizer only
        pub const INT_T: i32 = 101; // Tokenizer only
        pub const DOUBLE_T: i32 = 102; // Tokenizer only

        pub const WORD_T: i32 = 404; // Tokenizer only (This is for when a word fits none of the keywords)

        pub const END_T: i32 = 999; // Tokenizer only (end of script)

        // for future implementation
        // pub const FUNC_T: i32 = 60;
        // pub const CLASS_T: i32 = 61;
        // pub const ENUM_T: i32 = 62;
    }
}