use super::*;

fn assert_eval(input: &str, expected: f64) {
    let result = ExpressionEvaluator::new(input).eval();
    assert_eq!(result, expected);
}

fn assert_same_eval(left: &str, right: &str) {
    assert_eq!(
        ExpressionEvaluator::new(left).eval(),
        ExpressionEvaluator::new(right).eval()
    );
}

#[test]
fn adds() {
    assert_eval("1+2", 3.0);
}

#[test]
fn subtracts() {
    assert_eval("5-2", 3.0);
}

#[test]
fn multiplies() {
    assert_eval("3*4", 12.0);
}

#[test]
fn divides() {
    assert_eval("8/2", 4.0);
}

#[test]
fn exponentiates() {
    assert_eval("2^3", 8.0);
}

#[test]
fn respects_precedence() {
    assert_eval("1+2*3", 7.0);
}

#[test]
fn exponentiation_is_right_associative() {
    assert_eval("2^3^2", 512.0);
}

#[test]
fn handles_brackets() {
    assert_eval("(1+2)*3", 9.0);
}

#[test]
fn handles_signs() {
    assert_eval("-2+ +3", 1.0);
}

#[test]
fn handles_implicit_multiplication() {
    assert_eval("2(3+4)", 14.0);
    assert_eval("(1+2)(3+4)", 21.0);
}

#[test]
fn handles_decimals() {
    assert_eval("1.5+.5", 2.0);
    assert_eval("2.25*4", 9.0);
}

#[test]
fn formatting_does_not_change_result() {
    assert_same_eval("1+2*(3+4)", " 1 + 2 * ( 3 + 4 ) ");
    assert_same_eval("-2+3", " - 2 + + 3 ");
}

#[test]
fn implicit_multiplication_matches_explicit_multiplication() {
    assert_same_eval("2(3)", "2*(3)");
    assert_same_eval("2(3+4)", "2*(3+4)");
    assert_same_eval("(1+2)(3+4)", "(1+2)*(3+4)");
}

#[test]
fn decimal_implicit_multiplication_matches_explicit_multiplication() {
    assert_same_eval("2.5(4)", "2.5*(4)");
    assert_same_eval(".5(2+3)", ".5*(2+3)");
}

#[test]
fn division_with_implicit_multiplication_is_left_associative() {
    assert_same_eval("2/2(3)/2", "((2/2)*(3))/2");
}

#[test]
fn decimal_division_with_implicit_multiplication_is_left_associative() {
    assert_same_eval("2/.5(3)/.25", "((2/.5)*(3))/.25");
}

#[test]
fn division_respects_exponent_precedence() {
    assert_same_eval("2/2^(-3)/4", "2/(2^(-3))/4");
}

#[test]
fn decimal_exponents_keep_their_decimal_operand() {
    assert_same_eval("4^.5", "4^(1/2)");
    assert_same_eval("8/4^.5", "8/(4^.5)");
}

#[test]
#[should_panic(expected = "mismatched brackets")]
fn rejects_mismatched_brackets() {
    ExpressionEvaluator::new("(1+2").eval();
}

#[test]
#[should_panic(expected = "invalid operator")]
fn rejects_invalid_operator_position() {
    ExpressionEvaluator::new("*2").eval();
}
