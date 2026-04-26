#[derive(Debug, PartialEq, Clone)]
struct Parser<'a> {
    whole: &'a str,
    byte: usize,
    ops: Vec<Op>,
    values: Vec<f64>,
    brackets: Vec<Bracket>,
    expect_operand: bool,
}

#[derive(Debug)]
enum Token {
    Operator(Op),
    Operand(f64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum Op {
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Neg = 5,
    Exp = 6,
    Bracket(Bracket) = 7,
    #[allow(clippy::upper_case_acronyms)]
    EOF = 255,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Bracket(char);

impl<'a> Parser<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            whole: str,
            byte: 0,
            values: Vec::<f64>::new(),
            ops: Vec::<Op>::new(),
            brackets: Vec::<Bracket>::new(),
            expect_operand: true,
        }
    }

    fn run(&mut self) {
        while let Some(next) = self.next_token() {
            dbg!("--------");
            dbg!(&next);
            dbg!(&self.ops);
            dbg!(&self.values);
            dbg!(&self.expect_operand);

            match next {
                Token::Operator(op) => {
                    // handle sign changers & implicit multiplication
                    if self.expect_operand {
                        match op {
                            Op::Subtract => {
                                self.ops.push(Op::Neg);
                                continue;
                            }
                            Op::Add => {
                                continue;
                            }
                            Op::Bracket(_) => {}
                            _ => {
                                panic!("invalid operator");
                            }
                        }
                    }

                    if op.is_bracket() {
                        self.handle_brackets(op);
                        continue;
                    }

                    self.execute_until(|top| {
                        top.precedence() >= op.precedence()
                        && !(top == &Op::Exp && op == Op::Exp) // exponentiation is right associative
                        && !top.is_bracket()
                    });
                    self.ops.push(op);
                    self.expect_operand = true;
                }
                Token::Operand(v) => {
                    if !self.expect_operand {
                        self.execute_until(|top| {
                            top.precedence() >= Op::Multiply.precedence() && !top.is_bracket()
                        });
                        self.ops.push(Op::Multiply);
                    }

                    self.values.push(v);
                    self.expect_operand = false;
                }
            }
        }

        dbg!("--------");
        dbg!("outside of main loop, rest to execute is: ");
        dbg!(&self.whole[self.byte..]);
        dbg!(&self.ops);
        dbg!(&self.values);
        dbg!(&self.expect_operand);

        if !self.brackets.is_empty() {
            panic!("mismatched brackets");
        }

        // parse final number
        if self.byte < self.whole.len() {
            dbg!("parsing final number");
            let v = self.whole[self.byte..]
                .trim()
                .parse::<f64>()
                .expect("unparsable character sequence");
            self.values.push(v);
        }

        dbg!("--------");
        dbg!("executing remaining ops");
        dbg!(&self.ops);
        dbg!(&self.values);
        self.execute_until(|_| true);

        dbg!("--------");
        dbg!(&self.values);
        println!("= {}", self.values[0]);
    }

    fn handle_brackets(&mut self, op: Op) {
        let bracket = *op.inner_bracket();
        if bracket.is_opening() {
            if !self.expect_operand {
                self.execute_until(|top| {
                    top.precedence() >= Op::Multiply.precedence() && !top.is_bracket()
                });
                self.ops.push(Op::Multiply);
            }
            self.brackets.push(bracket);
            self.ops.push(op);

            self.expect_operand = true;
            return;
        }

        if self.brackets.last().is_none_or(|b| b.opposite() != bracket) {
            panic!("mismatched brackets");
        };

        self.execute_until(|top| top.precedence() < op.precedence());
        self.brackets.pop();
        self.ops.pop();
        self.expect_operand = false;
    }

    fn next_token(&mut self) -> Option<Token> {
        let rest = &self.whole[self.byte..];

        let mut chars = rest.chars();
        let mut c = chars.next();
        loop {
            if let Some(n) = c {
                self.byte += 1;
                if let Ok(op) = Op::try_from(n) {
                    return Some(Token::Operator(op));
                }

                let mut end = self.byte + 1;
                while let Some(n) = chars.next()
                    && Op::try_from(n).is_err()
                {
                    end += 1;
                }
                let str = self.whole[self.byte - 1..end - 1].trim();
                if str.is_empty() {
                    c = self.whole.chars().nth(end - 1);
                    self.byte = end - 1;
                    continue;
                }

                let v = str.parse::<f64>().expect("unparsable character sequence");
                self.byte = end - 1;
                return Some(Token::Operand(v));
            }
            return None;
        }
    }

    fn execute_until<F: Fn(&Op) -> bool>(&mut self, fun: F) {
        while let Some(top) = self.ops.last()
            && fun(top)
        {
            if top == &Op::EOF {
                self.ops.pop();
                continue;
            }

            top.apply(&mut self.values);
            self.ops.pop();
        }
    }
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Subtract),
            '*' => Ok(Self::Multiply),
            '/' => Ok(Self::Divide),
            '^' => Ok(Self::Exp),
            '{' | '}' | '[' | ']' | '(' | ')' => Ok(Self::Bracket(Bracket(value))),
            '\n' => Ok(Self::EOF),
            _ => Err(()),
        }
    }
}

impl Op {
    fn precedence(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }

    fn execute(&self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Subtract => a - b,
            Op::Multiply => a * b,
            Op::Divide => a / b,
            Op::Exp => a.powf(b),
            Op::EOF | Op::Bracket(_) | Op::Neg => unreachable!(),
        }
    }

    /// values should have at least two elements
    fn apply(&self, values: &mut Vec<f64>) {
        if matches!(self, Op::Neg) {
            let a = values.pop().expect("not enough elements");
            values.push(-a);
            return;
        }

        assert!(values.len() >= 2, "not enough elements");

        let b = values.pop().unwrap();
        let a = values.pop().unwrap();
        values.push(self.execute(a, b));
    }

    fn is_bracket(&self) -> bool {
        matches!(self, Op::Bracket(_))
    }

    fn inner_bracket(&self) -> &Bracket {
        match self {
            Self::Bracket(a) => a,
            _ => panic!("impossible"),
        }
    }
}

impl Bracket {
    fn opposite(&self) -> Bracket {
        Bracket(match self.0 {
            '{' => '}',
            '[' => ']',
            '(' => ')',
            '}' => '{',
            ']' => '[',
            ')' => '(',
            _ => unreachable!(),
        })
    }

    fn is_opening(&self) -> bool {
        match self.0 {
            '{' | '[' | '(' => true,
            '}' | ']' | ')' => false,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falied to read input");
    let mut p = Parser::new(&input);
    p.run();
}
