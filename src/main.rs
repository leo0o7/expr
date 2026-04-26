use crate::utils::{Bracket, Operator, Token};

#[cfg(test)]
mod tests;
mod utils;

#[derive(Debug, PartialEq, Clone)]
struct Lexer<'a> {
    source: &'a str,
    pos: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct ExpressionEvaluator<'a> {
    ops: Vec<Operator>,
    values: Vec<f64>,
    brackets: Vec<Bracket>,
    expect_operand: bool,
    lexer: Lexer<'a>,
}

impl<'a> ExpressionEvaluator<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            values: Vec::<f64>::new(),
            ops: Vec::<Operator>::new(),
            brackets: Vec::<Bracket>::new(),
            expect_operand: true,
            lexer: Lexer::new(str),
        }
    }

    fn eval(&mut self) -> f64 {
        while let Some(token) = self.lexer.next_token() {
            self.process_token(token);
        }

        self.finalize()
    }

    fn process_token(&mut self, token: Token) {
        match token {
            Token::Operator(op) => self.process_operator(op),
            Token::Operand(value) => self.process_operand(value),
        }
    }

    fn process_operator(&mut self, op: Operator) {
        if self.expect_operand {
            match op {
                Operator::Subtract => {
                    self.ops.push(Operator::Neg);
                    return;
                }
                Operator::Add => {
                    return;
                }
                Operator::Bracket(_) => {}
                _ => {
                    panic!("invalid operator");
                }
            }
        }

        if op.is_bracket() {
            self.process_bracket(op);
            return;
        }

        self.execute_until(|top| {
            top.precedence() >= op.precedence()
                        && !(top == &Operator::Exp && op == Operator::Exp) // exponentiation is right associative
                        && !top.is_bracket()
        });
        self.ops.push(op);
        self.expect_operand = true;
    }

    fn process_operand(&mut self, value: f64) {
        if !self.expect_operand {
            self.add_implicit_multiplication();
        }

        self.values.push(value);
        self.expect_operand = false;
    }

    fn process_bracket(&mut self, op: Operator) {
        let bracket = *op.inner_bracket();
        if bracket.is_opening() {
            self.process_left_bracket(bracket, op);
        } else {
            self.process_right_bracket(op, bracket);
        }
    }

    fn process_left_bracket(&mut self, bracket: Bracket, op: Operator) {
        if !self.expect_operand {
            self.add_implicit_multiplication();
        }

        self.brackets.push(bracket);
        self.ops.push(op);
        self.expect_operand = true;
    }

    fn process_right_bracket(&mut self, op: Operator, bracket: Bracket) {
        if self.brackets.last().is_none_or(|b| b.opposite() != bracket) {
            panic!("mismatched brackets");
        };

        self.execute_until(|top| top.precedence() < op.precedence());
        self.brackets.pop();
        self.ops.pop();
        self.expect_operand = false;
    }

    fn add_implicit_multiplication(&mut self) {
        self.execute_until(|top| {
            top.precedence() >= Operator::Multiply.precedence() && !top.is_bracket()
        });
        self.ops.push(Operator::Multiply);
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

    fn execute_until<F: Fn(&Operator) -> bool>(&mut self, fun: F) {
        while let Some(top) = self.ops.last()
            && fun(top)
        {
            if top == &Operator::EOF {
                self.ops.pop();
                continue;
            }
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

    fn next_token(&mut self) -> Option<Token> {
        let rest = &self.source[self.pos..];

        let mut chars = rest.chars();
        let mut c = chars.next();
        loop {
            if let Some(n) = c {
                self.pos += 1;
                if let Ok(op) = Operator::try_from(n) {
                    return Some(Token::Operator(op));
                }

                let mut end = self.pos + 1;
                while let Some(n) = chars.next()
                    && Operator::try_from(n).is_err()
                {
                    end += 1;
                }
                let str = self.source[self.pos - 1..end - 1].trim();
                if str.is_empty() {
                    c = self.source.chars().nth(end - 1);
                    self.pos = end - 1;
                    continue;
                }

                let v = str.parse::<f64>().expect("unparsable character sequence");
                self.pos = end - 1;
                return Some(Token::Operand(v));
            }
            return None;
        }
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
