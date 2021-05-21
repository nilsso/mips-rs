#[test]
fn parse_solar_script() {
    use mips_parser::prelude::*;
    use mips_parser::ast::all_node_variants::*;

    let path = "./example-scripts/solar.mips";
    let program = build_ast_from_path(path).unwrap();

    let res: Program = Program(vec![
        ( 0,  ExprFunc(Alias, vec![ ArgToken("sensor".into()), ArgDev(DevBase(0)) ]) ),
        ( 2,  ExprFunc(Alias, vec![ ArgToken("x".into()), ArgMem(MemBase(0)) ]) ),
        ( 4,  ExprFunc(Define, vec![ ArgToken("SolarPanelHash".into()), ArgVal(ValLit(-2045627400.0)) ]) ),
        ( 5,  ExprFunc(Define, vec![ ArgToken("H0".into()), ArgVal(ValLit(0.0)) ]) ),
        ( 7,  ExprLabel("start".into())),
        ( 8,  ExprFunc(Yield, vec![]) ),
        ( 9,  ExprLabel("horizontal".into()) ),
        ( 10, ExprFunc(Unknown, vec![
                       ArgMem(MemAlias("x".into())),
                       ArgDev(DevAlias("sensor".into())),
                       ArgToken("Horizontal".into())
        ]) ),
    ]);

    assert_eq!(program, res);
}
