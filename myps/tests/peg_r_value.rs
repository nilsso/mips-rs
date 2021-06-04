use pest::Parser;
use myps::{MypsParser, Rule};

macro_rules! parse_rvalue {
    ($source:literal) => {{
        let res = MypsParser::parse(Rule::r_value_line, &$source);
        println!("{:#?}", res);
        res.unwrap().flatten()
    }};
}

macro_rules! next_pair {
    ($iter:ident) => {{
        let pair = $iter.next().unwrap();
        (pair.as_rule(), pair.as_str())
    }};
}

macro_rules! assert_rvalue_pairs {
    ($source:literal, $(($rule:path, $str:literal)),*$(,)*) => {
        let mut pairs = parse_rvalue!($source);
        $( assert_eq!(next_pair!(pairs), ($rule, $str)); )*
        assert_eq!(next_pair!(pairs), (Rule::EOI, ""));
        assert!(pairs.next().is_none());
    };
}

#[test]
fn rvalue_batch_param() {
    assert_rvalue_pairs!(
        "-128473777.sum.Setting",
        (Rule::r_param,   "-128473777.sum.Setting"),
        (Rule::r_batch,   "-128473777.sum"),
        (Rule::int_lit,   "-128473777"),
        (Rule::batch_sum, "sum"),
        (Rule::param,     "Setting"));

    assert_rvalue_pairs!(
        "SolarPanel.max.Horizontal",
        (Rule::r_param,   "SolarPanel.max.Horizontal"),
        (Rule::r_batch,   "SolarPanel.max"),
        (Rule::alias,     "SolarPanel"),
        (Rule::batch_max, "max"),
        (Rule::param,     "Horizontal"));
}
