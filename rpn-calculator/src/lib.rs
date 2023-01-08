use std::vec;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {

    let mut stage: Vec<i32> = vec![];
    for value in inputs {
        if let CalculatorInput::Value(x) = value {
            stage.push(*x)
        } else {
            let second = stage.pop()?;
            let first = stage.pop()?;

            match value {
                CalculatorInput::Add => {
                    stage.push(first + second)
                },
                CalculatorInput::Subtract => {
                    stage.push(first - second)
                },
                CalculatorInput::Multiply => {
                    stage.push(first * second)
                },
                CalculatorInput::Divide => {
                    stage.push(first / second)
                },
                _ => return None
            }
        }
    }

    if stage.len() != 1 {
        return None
    }

    return stage.pop();
}