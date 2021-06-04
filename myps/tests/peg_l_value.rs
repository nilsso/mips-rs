use pest::Parser;
use myps::{MypsParser, Rule};

macro_rules! parse_lvalue {
    ($source:literal) => {{
        let res = MypsParser::parse(Rule::l_value_line, &$source);
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

macro_rules! assert_lvalue_pairs {
    ($source:literal, $(($rule:path, $str:literal)),*$(,)*) => {
        let mut pairs = parse_lvalue!($source);
        $( assert_eq!(next_pair!(pairs), ($rule, $str)); )*
        assert_eq!(next_pair!(pairs), (Rule::EOI, ""));
        assert!(pairs.next().is_none());
    };
}

#[test]
fn lvalue_mem() {
    assert_lvalue_pairs!(
        "r0",
        (Rule::mem_lit, "r0"),
        (Rule::int_lit, "0"));

    assert_lvalue_pairs!(
        "rrr1",
        (Rule::mem_lit, "rrr1"),
        (Rule::int_lit, "1"));
}

#[test]
fn lvalue_alias() {
    assert_lvalue_pairs!(
        "x",
        (Rule::alias, "x"));
}

#[test]
fn lvalue_dev_param() {
    assert_lvalue_pairs!(
        "d0.On",
        (Rule::l_param, "d0.On"),
        (Rule::dev_lit, "d0"),
        (Rule::int_lit, "0"),
        (Rule::param,   "On"));
}

#[test]
fn lvalue_alias_param() {
    // Suppose "Light = d0" and "Lights = <hash>.all".

    // Important to note that the alias could be directly to a device, e.g.
    assert_lvalue_pairs!(
        "Light.On",
        (Rule::l_param, "Light.On"),
        (Rule::alias,   "Light"),
        (Rule::param,   "On"));

    // Or it could be to a batch selector, e.g.
    assert_lvalue_pairs!(
        "Lights.On",
        (Rule::l_param, "Lights.On"),
        (Rule::alias,   "Lights"),
        (Rule::param,   "On"));
}

#[test]
fn lvalue_batch_param() {
    assert_lvalue_pairs!(
        "-128473777.all.Setting",
        (Rule::l_param, "-128473777.all.Setting"),
        (Rule::dev_batch, "-128473777.all"),
        (Rule::int_lit, "-128473777"),
        (Rule::param,   "Setting"));

    assert_lvalue_pairs!(
        "SolarPanel.all.Horizontal",
        (Rule::l_param, "SolarPanel.all.Horizontal"),
        (Rule::dev_batch, "SolarPanel.all"),
        (Rule::alias,   "SolarPanel"),
        (Rule::param,   "Horizontal"));
}

#[test]
#[should_panic]
fn lvalue_dev_invalid1() {
    parse_lvalue!("d0");
}

#[test]
#[should_panic]
fn lvalue_dev_invalid2() {
    parse_lvalue!("drrr2");
}

#[test]
#[should_panic]
fn lvalue_batch_invalid1() {
    parse_lvalue!("r0.all");
}

#[test]
#[should_panic]
fn lvalue_batch_invalid2() {
    parse_lvalue!("d0.all");
}

#[test]
#[should_panic]
fn lvalue_batch_invalid3() {
    parse_lvalue!("Type.sum.Param");
}
