pub mod lexer {
    use std::any::{Any};

    use crate::compiler::{tokenizer::tokenizer::Token, constants::constants::Constants};

    #[derive(Debug)]
    struct Variable<T: Sized> {
        pub name: String,
        pub val: Box<T>
    }

    impl<T: 'static> Variable<T> {
        fn new(name: String, val: Option<Box<T>>) -> Variable<T> {
            return Variable { name, val: val.unwrap_or(Box::new(unsafe { std::mem::MaybeUninit::zeroed().assume_init() })) };
        }
    }

    pub fn parse(tokens: Vec<Token>) {
        let mut i: usize = 0;
        let mut variables: Vec<Variable<_>> = vec![];

        while i < tokens.len() {
            let token: &Token = &tokens[i];
            
            if token.token == Constants::VAR_T {
                i += 1;

                if tokens[i].token != Constants::WORD_T {
                    todo!("syntax error")
                }

                let variable_name: &str = &*tokens[i].val;
                i += 1;

                if tokens[i].token != Constants::EQUALS_T {
                    variables.push(Variable::new(variable_name.to_string(), None))
                }

                variables[0].val = Box::<i32>::new(5);

                println!("{:#?}", variables[0]);

                let new_box: *mut i32 = Box::into_raw(variables[0].val.to_owned());

                unsafe { new_box.write(72) };

                println!("{:?}", variables[0].val = unsafe { Box::from_raw(new_box) });

                println!("{:#?}", variables[0]);
                
                i += 1;

            }

            i += 1;
        }
    }
}