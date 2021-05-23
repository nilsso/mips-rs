#![feature(custom_inner_attributes)]
#![rustfmt::skip::macros(mips_ast_test)]
use mips_parser::ast::nodes::Program;

#[test]
fn program1() {
    let input = "\
# aliases
alias x r0 # slot 1 (x)
alias y r1 # slot 2 (y)
alias z r2 # output slot (z)

# do stuff
move x 1   # slot x = 1
move y 2   # slot y = 2
add z x y  # slot z = x + y
";

    let output = "\
alias x r0
alias y r1
alias z r2
move x 1
move y 2
add z x y
";
    let res = Program::try_from_str(input);
    assert_eq!(res.unwrap().to_string(), output);
}
