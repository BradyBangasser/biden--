pub mod lexer {
    use crate::compiler::{tokenizer::tokenizer::Token, constants::constants::Constants};

    #[derive(Debug)]
    enum Types {
        String(Variable<String>),
        Double(Variable<f64>),
        Unknown(Variable<i32>),
        Int(Variable<i128>),
    }

    #[derive(Clone, Debug)]
    struct Variable<T: Sized> {
        name: String,
        val: T,
        undefined: bool
    }

    impl<T: Sized> Variable<T> {
        fn new(name: String, val: Option<T>) -> Variable<T> {
            let is_undefined: bool = val.is_none();
            let unwrapped_val: T = val.unwrap_or(unsafe { std::mem::MaybeUninit::zeroed().assume_init() });
            return Variable { name, val: unwrapped_val, undefined: is_undefined };
        }

        fn set_val(&mut self, new_val: T) {
            if self.undefined {
                self.undefined = false;
            }
            self.val = new_val;
        }

        // fn get_val(&self) -> T {
        //     return *self.val;
        // }

        fn name(self) -> String {
            return self.name;
        }

        fn is_undefined(self) -> bool {
            return self.undefined;
        }
    }

    pub fn parse(tokens: Vec<Token>) {
        let mut i: usize = 0;
        let mut variables: Vec<Types> = vec![];

        while i < tokens.len() {
            let token: &Token = &tokens[i];
            println!("here");
            
            if token.token == Constants::VAR_T {
                i += 1;

                if tokens[i].token != Constants::WORD_T || tokens[i].val.is_none() {
                    todo!("syntax error")
                }

                let variable_name: String = String::from(tokens[i].val.as_deref().unwrap());
                i += 1;

                if tokens[i].token != Constants::ASSIGN_T {
                    variables.push(Types::Unknown(Variable::new(variable_name, None)))
                } else {
                    i += 1;
                    let variable_value: &Box<str>;

                    if tokens[i].val.is_none() {
                        todo!("error here")
                    }

                    variable_value = tokens[i].val.as_ref().unwrap();

                    // figure out how to automate this
                    if tokens[i].token == Constants::DOUBLE_T {
                        variables.push(Types::Double(Variable::new(variable_name, Some(variable_value.parse::<f64>().unwrap()))))
                    } else if tokens[i].token == Constants::INT_T {
                        variables.push(Types::Int(Variable::new(variable_name, Some(variable_value.parse::<i128>().unwrap()))))
                    } else if tokens[i].token == Constants::STRING_T {
                        variables.push(Types::String(Variable::new(variable_name, Some(variable_value.parse::<String>().unwrap()))))
                    } else if tokens[i].token == Constants::INPUT_T {
                        todo!("input");
                        // The comment below to just to avoid an unused code warning
                    } else if tokens[i].token == Constants::PRINT_T {
                        i += 1;

                        let print_value: String;
                        if tokens[i].token == Constants::WORD_T {
                            todo!("print variable")
                        } else {
                            print_value = tokens[i].val.as_deref().unwrap().to_string()
                        }

                        println!("{}", print_value)
                    } else {
                        todo!("syntax error")
                    }
                }
                
                i += 1;

            }

            i += 1;
        }
        println!("{:#?}", variables);
    }
}