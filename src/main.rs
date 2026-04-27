use crate::utils::{Bracket, Operator, OperatorOrBracket, Token};

#[cfg(test)]
mod tests;
mod utils;

#[derive(Debug, PartialEq, Clone)]
struct ExpressionEvaluator<'a> {
    ops: Vec<OperatorOrBracket>,
    values: Vec<f64>,
    brackets: Vec<Bracket>,
    expect_operand: bool,
    lexer: Lexer<'a>,
}

#[derive(Debug, PartialEq, Clone)]
struct Lexer<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> ExpressionEvaluator<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            ops: Vec::<OperatorOrBracket>::new(),
            values: Vec::<f64>::new(),
            brackets: Vec::<Bracket>::new(),
            expect_operand: true,
            lexer: Lexer::new(str),
        }
    }

    fn eval(mut self) -> f64 {
        while let token = self.lexer.next_token()
            && token != Token::EOF
        {
            match token {
                Token::Operator(op) => self.process_operator(op),
                Token::Operand(value) => self.process_operand(value),
                Token::OpenBracket(bracket) => self.process_open_bracket(bracket),
                Token::CloseBracket(bracket) => self.process_close_bracket(bracket),
                Token::EOF => unreachable!(),
            };
        }

        self.finalize()
    }

    fn process_operator(&mut self, op: Operator) {
        if self.expect_operand {
            match op {
                Operator::Subtract => {
                    self.ops.push(OperatorOrBracket::Operator(Operator::Neg));
                    return;
                }
                Operator::Add => {
                    return;
                }
                _ => panic!("invalid operator"),
            }
        }

        self.execute_until(|top| {
            top.precedence() >= op.precedence() && !(top == Operator::Exp && op == Operator::Exp) // exponentiation is right associative
        });
        self.ops.push(OperatorOrBracket::Operator(op));
        self.expect_operand = true;
    }

    fn process_operand(&mut self, value: f64) {
        if !self.expect_operand {
            self.add_implicit_multiplication();
        }

        self.values.push(value);
        self.expect_operand = false;
    }

    fn process_open_bracket(&mut self, bracket: Bracket) {
        if !self.expect_operand {
            self.add_implicit_multiplication();
        }

        self.brackets.push(bracket);
        self.ops.push(OperatorOrBracket::Bracket(bracket));
        self.expect_operand = true;
    }

    fn process_close_bracket(&mut self, bracket: Bracket) {
        if self.brackets.last().is_none_or(|b| b != &bracket) {
            panic!("mismatched brackets");
        };

        self.execute_until(|_| true);
        self.brackets.pop();
        self.ops.pop();
        self.expect_operand = false;
    }

    fn add_implicit_multiplication(&mut self) {
        self.execute_until(|top| top.precedence() >= Operator::Multiply.precedence());
        self.ops
            .push(OperatorOrBracket::Operator(Operator::Multiply));
    }

    fn finalize(&mut self) -> f64 {
        if !self.brackets.is_empty() {
            panic!("mismatched brackets");
        }

        self.execute_until(|_| true);

        if self.values.len() != 1 {
            panic!("invalid expression");
        }

        self.values[0]
    }

    fn execute_until<F: Fn(Operator) -> bool>(&mut self, fun: F) {
        while let Some(top) = self.ops.last().and_then(|top| top.get_operator())
            && fun(top)
        {
            top.apply(&mut self.values);
            self.ops.pop();
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            source: str,
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        let rest = &self.source[self.pos..];

        let mut chars = rest.chars().peekable();
        while let Some(n) = chars.next() {
            self.pos += 1;
            if n.is_whitespace() {
                continue;
            }

            match n {
                '(' => return Token::OpenBracket(Bracket::Paren),
                ')' => return Token::CloseBracket(Bracket::Paren),
                '[' => return Token::OpenBracket(Bracket::Square),
                ']' => return Token::CloseBracket(Bracket::Square),
                '{' => return Token::OpenBracket(Bracket::Brace),
                '}' => return Token::CloseBracket(Bracket::Brace),
                _ => {
                    if let Ok(op) = Operator::try_from(n) {
                        return Token::Operator(op);
                    }

                    let mut seen_dot = false;
                    let mut end = self.pos;
                    while let Some(n) = chars.peek()
                        && (n.is_numeric() || (*n == '.' && !seen_dot))
                    {
                        if *n == '.' {
                            seen_dot = true;
                        }
                        end += 1;
                        chars.next();
                    }
                    let str = self.source[self.pos - 1..end].trim();
                    let v = str.parse::<f64>().expect("unparsable character sequence");

                    self.pos = end;
                    return Token::Operand(v);
                }
            }
        }

        Token::EOF
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falied to read input");
    let res = ExpressionEvaluator::new(&input).eval();
    println!("={}", res);
}
