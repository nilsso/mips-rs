#[test]
fn parse_solar_script() {
    use mips_parser::prelude::*;
    use mips_parser::ast::all_node_variants::*;

    let path = "./example-scripts/solar.mips";
    let program = build_ast_from_path(path).unwrap();

    let res: Program = Program(vec![
        (0, ExprFunc(Alias, vec![
            ArgToken("sensor".into()),
            ArgDev(DevBase(0))
        ])),
        (2, ExprFunc(Alias, vec![
            ArgToken("x".into()),
            ArgMem(MemBase(0))
        ])),
        (4, ExprFunc(Define, vec![
            ArgToken("SolarPanelHash".into()),
            ArgVal(ValLit(-2045627400.0))
        ])),
        (5, ExprFunc(Define, vec![
            ArgToken("H0".into()),
            ArgVal(ValLit(0.0))
        ])),
        (7, ExprLabel("start".into())),
        (8, ExprFunc(Yield, vec![])),
        (9, ExprLabel("horizontal".into())),
        (10, ExprFunc(L, vec![
            ArgMem(MemAlias("x".into())),
            ArgDev(DevAlias("sensor".into())),
            ArgToken("Horizontal".into())
        ])),
        (11, ExprFunc(Sub, vec![
            ArgMem(MemAlias("x".into())),
            ArgVal(ValMem(MemAlias("H0".into()))),
            ArgVal(ValMem(MemAlias("x".into())))
        ])),
        (12, ExprFunc(Sb, vec![
            ArgVal(ValMem(MemAlias("SolarPanelHash".into()))),
            ArgToken("Horizontal".into()),
            ArgVal(ValMem(MemAlias("x".into())))
        ])),
        (13, ExprLabel("vertical".into())),
        (14, ExprFunc(L, vec![
            ArgMem(MemAlias("x".into())),
            ArgDev(DevAlias("sensor".into())),
            ArgToken("Vertical".into())
        ])),
        (15, ExprFunc(Sub, vec![
            ArgMem(MemAlias("x".into())),
            ArgVal(ValLit(75.0)),
            ArgVal(ValMem(MemAlias("x".into())))
        ])),
        (16, ExprFunc(Div, vec![
            ArgMem(MemAlias("x".into())),
            ArgVal(ValMem(MemAlias("x".into()))),
            ArgVal(ValLit(1.5))
        ])),
        (17, ExprFunc(Sb, vec![
            ArgVal(ValMem(MemAlias("SolarPanelHash".into()))),
            ArgToken("Vertical".into()),
            ArgVal(ValMem(MemAlias("x".into())))
        ])),
        (18, ExprFunc(J, vec![
            ArgVal(ValMem(MemAlias("start".into())))
        ])),
    ]);

    assert_eq!(program, res);
}
