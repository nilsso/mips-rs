use pest::Parser;
use myps::{MypsParser, Rule};

macro_rules! parse_assign_r_value {
    ($source:literal) => {{
        let res = MypsParser::parse(Rule::assign_r_value_line, &$source);
        println!("{:#?}", res);
        res.unwrap().flatten()
    }};
}

macro_rules! assert_assign_r_value_pairs {
    ($source:literal$(,)? $(($a:path, $b:literal)),*$(,)*) => {{
        let mut pairs = parse_assign_r_value!($source);
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
fn assign_r_value_num() {
    assert_assign_r_value_pairs!(
        "r0 = 5"
        (Rule::assign_r_value, "r0 = 5"),
        (Rule::mem_lit, "r0"),
        (Rule::int_lit, "0"),
        (Rule::num_lit, "5"));

    assert_assign_r_value_pairs!(
        "x = 6"
        (Rule::assign_r_value, "x = 6"),
        (Rule::alias, "x"),
        (Rule::num_lit, "6"));
}

#[test]
fn assign_r_value_param() {
    assert_assign_r_value_pairs!(
        "a = d0.Param"
        (Rule::assign_r_value, "a = d0.Param"),
        (Rule::alias, "a"),
        (Rule::r_param, "d0.Param"),
        (Rule::dev_lit, "d0"),
        (Rule::int_lit, "0"),
        (Rule::param, "Param"));
}

#[test]
fn assign_r_value_expr1() {
    assert_assign_r_value_pairs!(
        "r1 = 3 / 5"
        (Rule::assign_r_value, "r1 = 3 / 5"),
        (Rule::mem_lit, "r1"),
        (Rule::int_lit, "1"),
        (Rule::b_expr,  "3 / 5"),
        (Rule::num_lit, "3"),
        (Rule::div,     "/"),
        (Rule::num_lit, "5"));
}

#[test]
fn assign_r_value_expr2() {
    assert_assign_r_value_pairs!(
        "x = -(2 * y)"
        (Rule::assign_r_value, "x = -(2 * y)"),
        (Rule::alias, "x"),
        (Rule::u_expr, "-(2 * y)"),
        (Rule::inv,    "-"),
        (Rule::b_expr, "2 * y"),
        (Rule::num_lit, "2"),
        (Rule::mul,     "*"),
        (Rule::alias,   "y"));
}

#[test]
fn assign_r_value_expr3() {
    assert_assign_r_value_pairs!(
        "panels.Vertical = (sensor.Vertical - 75) / 1.5"
        (Rule::assign_r_value, "panels.Vertical = (sensor.Vertical - 75) / 1.5"),
        (Rule::l_param, "panels.Vertical"),
        (Rule::alias,   "panels"),
        (Rule::param,   "Vertical"),
        (Rule::b_expr,  "(sensor.Vertical - 75) / 1.5"),
        (Rule::b_expr,  "sensor.Vertical - 75"),
        (Rule::r_param, "sensor.Vertical"),
        (Rule::alias,   "sensor"),
        (Rule::param,   "Vertical"),
        (Rule::sub,     "-"),
        (Rule::num_lit, "75"),
        (Rule::div,     "/"),
        (Rule::num_lit, "1.5"));
}

#[test]
#[should_panic]
fn assign_r_value_panic1() {
    parse_assign_r_value!("y = SolarPanel.all");
}
