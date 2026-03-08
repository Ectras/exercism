pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

type Definition = (String, Vec<String>);

pub struct Forth {
    /// A vector of definitions. The items are inserted in the order they are defined.
    /// Also, a word can be defined multiple times. Hence, this allows redefining
    /// words without changing the functionality of existing definitions that used
    /// this word.
    ///
    /// # Example
    /// ```notrust
    /// : foo 5;
    /// : bar foo ;
    /// : foo 6 ;
    /// bar foo
    /// ```
    /// Here, `bar` still uses the first definition of `foo` while the direct call to
    /// `foo` uses the redefined value of 6.
    /// In this vector, the scenario would look like this:
    /// ```notrust
    /// [("foo", ["5"]), ("bar", ["foo"]), ("foo", ["6"])]
    /// ```
    /// Hence, the definition of `foo` used by `bar` can be found by searching for
    /// `foo`, starting from `bar` and going backwards.
    definitions: Vec<Definition>,

    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            definitions: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn pop_args(&mut self, count: usize) -> std::result::Result<Vec<Value>, Error> {
        let Some(new_len) = self.stack.len().checked_sub(count) else {
            return Err(Error::StackUnderflow);
        };
        let last = self.stack.drain(new_len..);
        let args = last.collect::<Vec<_>>();
        Ok(args)
    }

    fn add(&mut self) -> Result {
        let args = self.pop_args(2)?;
        self.stack.push(args[0] + args[1]);
        Ok(())
    }

    fn sub(&mut self) -> Result {
        let args = self.pop_args(2)?;
        self.stack.push(args[0] - args[1]);
        Ok(())
    }

    fn mul(&mut self) -> Result {
        let args = self.pop_args(2)?;
        self.stack.push(args[0] * args[1]);
        Ok(())
    }

    fn div(&mut self) -> Result {
        let args = self.pop_args(2)?;
        if args[1] == 0 {
            return Err(Error::DivisionByZero);
        }
        self.stack.push(args[0] / args[1]);
        Ok(())
    }

    fn dup(&mut self) -> Result {
        let last = self.stack.last().ok_or(Error::StackUnderflow)?;
        self.stack.push(*last);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        let _ = self.pop_args(1)?;
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let args = self.pop_args(2)?;
        self.stack.push(args[1]);
        self.stack.push(args[0]);
        Ok(())
    }

    fn over(&mut self) -> Result {
        let index = self.stack.len().checked_sub(2).ok_or(Error::StackUnderflow);
        let second_last = self.stack[index?];
        self.stack.push(second_last);
        Ok(())
    }

    fn read_definition(input: &[&str]) -> std::result::Result<Definition, Error> {
        assert_eq!(input[0], ":");

        // Search for end of definition
        let end = input.iter().position(|&w| w == ";");
        let end = end.ok_or(Error::InvalidWord)?;

        // Get the substring
        let definition = &input[1..end];

        // Prevent nested definitions
        assert!(!definition.contains(&":"));

        // Extract the first word (the name of the definition)
        let [name, body @ ..] = definition else {
            return Err(Error::InvalidWord);
        };

        // Check if the key is valid
        if let Ok(_) = name.parse::<Value>() {
            return Err(Error::InvalidWord);
        }

        // Convert slices to owned strings
        let body = body.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        Ok((name.to_string(), body))
    }

    fn eval_code(&mut self, input: &[&str], mut scope_ptr: usize) -> Result {
        let mut pos = 0;
        while let Some(&word) = input.get(pos) {
            if word == ":" {
                // We have a definition
                assert_eq!(scope_ptr, self.definitions.len());
                let (name, body) = Forth::read_definition(&input[pos..])?;
                pos += body.len() + 2; // + 2 because of key and end token (;)
                self.definitions.push((name, body));
                scope_ptr += 1;
            } else if let Ok(number) = word.parse::<Value>() {
                // We have a number literal
                self.stack.push(number)
            } else if let Some(rev_definition_index) = self.definitions[..scope_ptr]
                .iter()
                .rev()
                .position(|x| x.0 == word)
            {
                // We have a user-defined word
                let definition_index = scope_ptr - 1 - rev_definition_index;
                let body = self.definitions[definition_index].1.clone();
                let body = body.iter().map(|s| &**s).collect::<Vec<_>>();
                self.eval_code(&body, definition_index)?;
            } else {
                // We have a built-in word or an invalid word
                match word {
                    "+" => self.add()?,
                    "-" => self.sub()?,
                    "*" => self.mul()?,
                    "/" => self.div()?,
                    "dup" => self.dup()?,
                    "drop" => self.drop()?,
                    "swap" => self.swap()?,
                    "over" => self.over()?,
                    _ => return Err(Error::UnknownWord),
                }
            }
            pos += 1;
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        let tokens = input.split_ascii_whitespace().collect::<Vec<_>>();
        self.eval_code(&tokens, self.definitions.len())
    }
}
