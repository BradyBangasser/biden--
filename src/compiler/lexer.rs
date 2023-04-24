pub mod lexer {

    use crate::compiler::{tokenizer::tokenizer::Token, constants::constants::Constants};

    #[derive(Debug, Clone)]
    struct Variable<T: Sized> {
        name: String,
        val: Box<T>,
        undefined: bool
    }

    impl<T: 'static> Variable<T> {
        fn new(name: String, val: Option<Box<T>>) -> Variable<T> {
            let is_undefined = val.is_none();
            let unwrapped_val = val.unwrap_or(Box::new(unsafe { std::mem::MaybeUninit::zeroed().assume_init() }));
            return Variable { name, val: unwrapped_val, undefined: is_undefined };
        }

        fn set_val(&mut self, new_val: T) {
            if !self.undefined {
                self.undefined = false;
            }


            let raw_variable: *mut T = Box::into_raw(self.val);
            unsafe {
                raw_variable.write(new_val);
                self.val = Box::from_raw(raw_variable);
            }
        }

        fn get_val(self) -> T {
            return *self.val;
        }

        fn name(self) -> String {
            return self.name;
        }

        fn is_undefined(self) -> bool {
            return self.undefined;
        }

        fn get_boxed_val(self) -> Box<T> {
            return self.val;
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
                    variables.push(Variable::<i32>::new(variable_name.to_string(), None))
                }

                println!("{:#?}", variables[0]);

                variables[0].set_val(5);

                println!("{:#?}", variables[0]);
                
                i += 1;

            }

            i += 1;
        }
    }
}