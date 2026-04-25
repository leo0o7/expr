#[derive(Debug, PartialEq)]
enum Op {
    EOF = 9999,
    Add = 1,      // 1
    Subtract = 2, // 1
    Multiply = 3, // 2
    Divide = 4,   // 2
    Exp = 5,      // 3
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
            '\n' => Ok(Self::EOF),
            _ => Err(()),
        }
    }
}
impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Self::EOF => 0,
            Self::Add => 1,
            Self::Subtract => 1,
            Self::Multiply => 2,
            Self::Divide => 2,
            Self::Exp => 3,
        }
    }
    fn execute(&self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Subtract => a - b,
            Op::Multiply => a * b,
            Op::Divide => a / b,
            Op::Exp => a.powf(b),
            Op::EOF => unreachable!(),
        }
    }
}

fn apply_op(values: &mut Vec<f64>, op: &Op) {
    // values should have at least two elements
    assert!(values.len() >= 2, "not enough elements");
    let b = values.pop().unwrap();
    let a = values.pop().unwrap();
    values.push(op.execute(a, b));
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falied to read input");

    let mut values = Vec::<f64>::new();
    let mut ops = Vec::<Op>::new();

    let mut curr = String::from("");
    for c in input.chars() {
        // dbg!(&c);
        // dbg!(&ops);
        // dbg!(&values);

        if let Ok(op) = Op::try_from(c) {
            let v = curr
                .trim()
                .parse::<f64>()
                .expect("unparsable character sequence");
            values.push(v);

            while let Some(top) = ops.last()
                && top.precedence() >= op.precedence()
                && !(top == &Op::Exp && op == Op::Exp)
            // exponentiation is right associative
            {
                apply_op(&mut values, top);
                ops.pop();
            }

            ops.push(op);
            curr = String::from("");
        } else {
            curr.push(c);
        }
    }
    // dbg!(&ops);
    // dbg!(&values);

    // run cleanup
    if !curr.is_empty() {
        let v = curr
            .trim()
            .parse::<f64>()
            .expect("unparsable character sequence");
        values.push(v);

        while let Some(op) = ops.pop() {
            apply_op(&mut values, &op);
        }
    }
    println!("= {}", values[0]);
}
