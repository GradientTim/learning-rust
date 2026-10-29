fn main() {
    println!("0 = {}", calculate(""));
    println!("1 + 2 = {}", calculate("1 + 2"));
    println!("1 + 2 + 3 * 5 = {}", calculate("1 + 2 + 3 * 5"));
    println!("2 + 2 * 2 = {}", calculate("1 + 2 + 3 * 5"));
    println!("50 - 10 * 2 = {}", calculate("1 + 2 + 3 * 5"));
}

#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    Operator(Operator),
}

#[derive(Debug, Clone)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

fn calculate(input: &str) -> i64 {
    parse(tokenize(input))
}

const RADIX: u32 = 10;

fn tokenize(input: &str) -> Vec<Token> {
    let normalized_input = input.replace(" ", "");

    let mut tokens = vec![];
    let mut chars = normalized_input.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            '0'..='9' => {
                let mut value = char.to_digit(RADIX).unwrap() as i64;

                while let Some(peeked_char) = chars.peek() {
                    if peeked_char.is_digit(RADIX) {
                        let next_char = chars.next().unwrap();
                        value = value * RADIX as i64 + next_char.to_digit(RADIX).unwrap() as i64;
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(value));
            }
            '+' => tokens.push(Token::Operator(Operator::Addition)),
            '-' => tokens.push(Token::Operator(Operator::Subtraction)),
            '*' => tokens.push(Token::Operator(Operator::Multiplication)),
            '/' => tokens.push(Token::Operator(Operator::Division)),
            _ => {}
        }
    }

    tokens
}

fn parse(tokens: Vec<Token>) -> i64 {
    if tokens.is_empty() {
        return 0;
    }

    let mut result = 0;
    let mut last_operator = Operator::Addition;

    let mut term = 0;
    let mut last_term_operator: Operator = Operator::Addition;

    // TODO: fix
    tokens.iter().for_each(|token| match token {
        Token::Number(digit) => match last_operator {
            Operator::Addition => {
                result += digit;
            }
            Operator::Subtraction => {
                result -= digit;
            }
            Operator::Multiplication => {
                result *= digit;
            }
            Operator::Division => {
                result /= digit;
            }
        },
        Token::Operator(operator) => {
            last_operator = operator.clone();
        },
    });

    result
}

// tokens goes just from left to right without respecting multiplication and division,
// so the results are wrong when using one of these operators.
// fn parse(tokens: Vec<Token>) -> i64 {
//     let mut result = 0;
//     let mut last_operator = Token::Addition;
//
//     tokens.iter().for_each(|token| match token {
//         Token::Digit(digit) => match last_operator {
//             Token::Addition => {
//                 result += digit;
//             }
//             Token::Subtraction => {
//                 result -= digit;
//             }
//             Token::Multiplication => {
//                 result *= digit;
//             }
//             Token::Division => {
//                 result /= digit;
//             }
//             _ => {}
//         },
//         Token::Addition | Token::Subtraction | Token::Multiplication | Token::Division => {
//             last_operator = token.clone()
//         }
//     });
//
//     result
// }
