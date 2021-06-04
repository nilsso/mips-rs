use pest::Parser;
use myps::{MypsParser, Rule};

macro_rules! parse_assign_l_value {
    ($source:literal) => {{
        let res = MypsParser::parse(Rule::assign_l_value_line, &$source);
        println!("{:#?}", res);
        res.unwrap().flatten()
    }};
}

macro_rules! assert_assign_l_value_pairs {
    ($source:literal$(,)? $(($a:path, $b:literal)),*$(,)*) => {{
        let mut pairs = parse_assign_l_value!($source);
        $( assert_eq!(next_pair!(pairs), ($a, $b)); )*
        assert_eq!(next_pair!(pairs), (Rule::EOI, ""));
        assert!(pairs.next().is_none());
    }};
}

macro_rules! next_pair {
    ($iter:ident) => {{
        let pair = $iter.next().unwrap();
        (pair.as_rule(), pair.as_str())
    }};
}

#[test]
fn assign_l_value1() {
    assert_assign_l_value_pairs!(
        "sensor = d0",
        (Rule::assign_l_value, "sensor = d0"),
        (Rule::alias,   "sensor"),
        (Rule::dev_lit, "d0"),
        (Rule::int_lit, "0"));
}

#[test]
fn assign_l_value2() {
    assert_assign_l_value_pairs!(
        "panels = -2045627372.all",
        (Rule::assign_l_value, "panels = -2045627372.all"),
        (Rule::alias,   "panels"),
        (Rule::dev_batch, "-2045627372.all"),
        (Rule::int_lit, "-2045627372"));
}

#[test]
#[should_panic]
fn assign_l_value_panic1() {
    parse_assign_l_value!("d0 = d0");
}

#[test]
#[should_panic]
fn assign_l_value_panic2() {
    parse_assign_l_value!("x = (1 + 2)");
}
