#[derive(Clone, Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn perform_operation(op: &CalculatorInput, arg1: i32, arg2: i32) -> i32 {
    match op {
        CalculatorInput::Add => arg1 + arg2,
        CalculatorInput::Subtract => arg1 - arg2,
        CalculatorInput::Multiply => arg1 * arg2,
        CalculatorInput::Divide => arg1 / arg2,
        _ => panic!("Unhandled operator"),
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => stack.push(*val),
            _ => {
                let arg2 = stack.pop();
                let arg1 = stack.pop();
                if arg1.is_none() || arg2.is_none() {
                    return None;
                } else {
                    let res = perform_operation(input, arg1?, arg2?);
                    stack.push(res);
                }
            }
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
