#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Token {
    Operand(f64),
    Operator(Operator),
    OpenBracket(Bracket),
    CloseBracket(Bracket),
    #[allow(clippy::upper_case_acronyms)]
    EOF,
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Bracket {
    Paren,  // ()
    Square, // []
    Brace,  // {}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum OperatorOrBracket {
    Operator(Operator),
    Bracket(Bracket),
}

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Subtract),
            '*' => Ok(Self::Multiply),
            '/' => Ok(Self::Divide),
            '^' => Ok(Self::Exp),
            _ => Err(()),
        }
    }
}

impl Operator {
    pub(crate) fn precedence(&self) -> u8 {
        *self as u8
    }

    fn execute(&self, a: f64, b: f64) -> f64 {
        match self {
            Operator::Add => a + b,
            Operator::Subtract => a - b,
            Operator::Multiply => a * b,
            Operator::Divide => a / b,
            Operator::Exp => a.powf(b),
            Operator::Neg => unreachable!("can't call execute on a Operator::Neg"),
        }
    }

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
}

impl OperatorOrBracket {
    pub fn get_operator(&self) -> Option<Operator> {
        match &self {
            OperatorOrBracket::Operator(operator) => Some(*operator),
            OperatorOrBracket::Bracket(_) => None,
        }
    }
}
