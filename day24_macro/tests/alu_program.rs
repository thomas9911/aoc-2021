use day24_macro::AluProgram;

#[derive(AluProgram)]
#[alu_program("day24_macro/tests/test.txt")]
struct Hello {}

#[derive(AluProgram)]
#[alu_program("day24_macro/tests/test2.txt")]
struct Bye {}

#[test]
fn calculate() {
    assert_eq!(13, Hello::calculate(&[1, 2]));
    assert_eq!(20, Hello::calculate(&[8, 2]));
}

#[test]
fn calculate_parts() {
    assert_eq!(74, Bye::calculate_0(9, 2));
    assert_eq!(71, Bye::calculate_1(9, 2));
}
