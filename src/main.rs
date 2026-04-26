use core::panic;

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
    EOF = 255,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Bracket(char);

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

    let mut values = Vec::<f64>::new();
    let mut ops = Vec::<Op>::new();
    let mut open_brackets = Vec::<Bracket>::new();

    let mut curr = String::from("");
    let mut expect_operand = true;
    for (op_opt, c) in input.chars().map(|c| (Op::try_from(c), c)) {
        if op_opt.is_err() {
            curr.push(c);
            continue;
        }

        let op = op_opt.unwrap();
        let trimmed = curr.trim();

        // dbg!(&trimmed);
        // dbg!(&op);
        // dbg!(&ops);
        // dbg!(&values);
        // dbg!(&expect_operand);

        if !trimmed.is_empty() {
            let v = trimmed
                .parse::<f64>()
                .expect("unparsable character sequence");
            values.push(v);
        } else {
            if expect_operand && op == Op::Subtract {
                ops.push(Op::Neg);
                curr = String::from("");
                continue;
            }
            if expect_operand && op == Op::Add {
                curr = String::from("");
                continue;
            }
        }

        if op.is_bracket() {
            let bracket = *op.inner_bracket();
            if bracket.is_opening() {
                open_brackets.push(bracket);
                ops.push(op);
                expect_operand = true;
            } else {
                if open_brackets.last().is_none_or(|b| b.opposite() != bracket) {
                    panic!("mismatched brackets");
                };

                while let Some(top) = ops.last()
                    && top.precedence() < op.precedence()
                {
                    top.apply(&mut values);
                    ops.pop();
                }
                open_brackets.pop();
                ops.pop();
                expect_operand = false;
            }
        } else {
            while let Some(top) = ops.last()
                && top.precedence() >= op.precedence()
                && !(top == &Op::Exp && op == Op::Exp) // exponentiation is right associative
                && !top.is_bracket()
            {
                top.apply(&mut values);
                ops.pop();
            }

            ops.push(op);
            expect_operand = true;
        }

        curr = String::from("");
    }
    // dbg!(&ops);
    // dbg!(&values);

    if !open_brackets.is_empty() {
        panic!("mismatched brackets");
    }
    if !curr.is_empty() {
        let v = curr
            .trim()
            .parse::<f64>()
            .expect("unparsable character sequence");
        values.push(v);
    }
    while let Some(op) = ops.pop() {
        // dbg!(&op);
        // dbg!(&ops);
        // dbg!(&values);
        if op == Op::EOF {
            continue;
        }
        op.apply(&mut values);
    }
    // dbg!(&values);

    println!("= {}", values[0]);
}
