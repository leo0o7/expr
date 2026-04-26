#[derive(Debug)]
pub(crate) enum Token {
    Operator(Operator),
    Operand(f64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub(crate) enum Operator {
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
pub(crate) struct Bracket(char);

impl TryFrom<char> for Operator {
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

impl Operator {
    pub(crate) fn precedence(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }

    fn execute(&self, a: f64, b: f64) -> f64 {
        match self {
            Operator::Add => a + b,
            Operator::Subtract => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b,
            Operator::Exp => a.powf(b),
            Operator::EOF | Operator::Bracket(_) | Operator::Neg => unreachable!(),
        }
    }

    /// values should have at least two elements
    pub(crate) fn apply(&self, values: &mut Vec<f64>) {
        if matches!(self, Operator::Neg) {
            let a = values.pop().expect("not enough elements");
            values.push(-a);
            return;
        }

        assert!(values.len() >= 2, "not enough elements");

        let b = values.pop().unwrap();
        let a = values.pop().unwrap();
        values.push(self.execute(a, b));
    }

    pub(crate) fn is_bracket(&self) -> bool {
        matches!(self, Operator::Bracket(_))
    }

    pub(crate) fn inner_bracket(&self) -> &Bracket {
        match self {
            Self::Bracket(a) => a,
            _ => panic!("impossible"),
        }
    }
}

impl Bracket {
    pub(crate) fn opposite(&self) -> Bracket {
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

    pub(crate) fn is_opening(&self) -> bool {
        match self.0 {
            '{' | '[' | '(' => true,
            '}' | ']' | ')' => false,
            _ => unreachable!(),
        }
    }
}
