// <stmnt> ::= "What is" <number> (<operation> <number> | <exp>)* "?"
// <operation> ::= "plus" | "minus" | "multiplied by" | "divided by"
// <exp> ::= "raised to the" <count> "power?"
// <count> ::= <number> ("st", "nd", "rd", "th")

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operation {
    Unknown,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operation {
    fn perform(&self, a: i32, b: i32) -> i32 {
        match self {
            Operation::Add => a + b,
            Operation::Sub => a - b,
            Operation::Mul => a * b,
            Operation::Div => a / b,
            Operation::Pow => a.pow(b.try_into().unwrap()),
            Operation::Unknown => unreachable!(),
        }
    }
}

fn assert_next<'a>(tokens: &mut impl Iterator<Item = &'a str>, expected: &str) -> Option<()> {
    tokens.next().filter(|&t| t == expected).and(Some(()))
}

fn parse_operation<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Option<Operation> {
    match tokens.next()? {
        "plus" => Some(Operation::Add),
        "minus" => Some(Operation::Sub),
        "multiplied" => {
            assert_next(tokens, "by")?;
            Some(Operation::Mul)
        }
        "divided" => {
            assert_next(tokens, "by")?;
            Some(Operation::Div)
        }
        "raised" => {
            assert_next(tokens, "to")?;
            assert_next(tokens, "the")?;
            Some(Operation::Pow)
        }
        _ => Some(Operation::Unknown),
    }
}

fn parse_number<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Option<i32> {
    tokens.next().map(|t| t.parse().ok()).flatten()
}

fn get_suffix(digit: char) -> &'static str {
    match digit {
        '1' => "st",
        '2' => "nd",
        '3' => "rd",
        _ => "th",
    }
}

fn parse_power_number<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Option<i32> {
    let token = tokens.next()?;
    let (number, suffix) = token.split_at(token.len() - 2);
    let correct_suffix = get_suffix(number.chars().last().unwrap());
    if suffix != correct_suffix {
        return None;
    }

    let number = number.parse().ok();
    number
}

fn parse_statement<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Option<i32> {
    assert_next(tokens, "What")?;
    assert_next(tokens, "is")?;
    let first = parse_number(tokens)?;
    let mut acc = first;
    while let Some(op) = parse_operation(tokens) {
        if op == Operation::Unknown {
            return None;
        }

        if op == Operation::Pow {
            let second = parse_power_number(tokens)?;
            assert_next(tokens, "power")?;
            acc = op.perform(acc, second);
        } else {
            let second = parse_number(tokens)?;
            acc = op.perform(acc, second);
        }
    }
    Some(acc)
}

pub fn answer(command: &str) -> Option<i32> {
    // Remove the question mark (since there is no whitespace, it's nasty to parse)
    if !command.ends_with("?") {
        return None;
    }
    let command = &command[..command.len() - 1];

    // Parse the statement
    let mut tokens = command.split_ascii_whitespace();
    parse_statement(&mut tokens)
}
