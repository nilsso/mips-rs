use pest::Parser;
use myps::{MypsParser, Rule, FirstInner, Ast, Op, RValueExpr};

#[test]
fn binary_expr() {
    let source = "(1 + (2 - 3 * 4) / 5) * 6";
    let res = MypsParser::parse(Rule::r_value_expr, &source);
    let pair = res.unwrap().first_inner().unwrap();
    let ast = RValueExpr::try_from_pair(pair).unwrap();

    use Op::*;
    use RValueExpr::*;

    let ans = RVEBinary {
        op: OpMul,
        lhs: Box::new(RVEBinary {
            op: OpAdd,
            lhs: Box::new(RVEValue(1.0)),
            rhs: Box::new(RVEBinary {
                op: OpDiv,
                lhs: Box::new(RVEBinary {
                    op: OpSub,
                    lhs: Box::new(RVEValue(2.0)),
                    rhs: Box::new(RVEBinary {
                        op: OpMul,
                        lhs: Box::new(RVEValue(3.0)),
                        rhs: Box::new(RVEValue(4.0)),
                    })
                }),
                rhs: Box::new(RVEValue(5.0)),
            }),
        }),
        rhs: Box::new(RVEValue(6.0)),
    };

    assert_eq!(ast, ans);
}
