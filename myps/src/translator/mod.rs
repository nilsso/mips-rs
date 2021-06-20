// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::{
    fmt,
    fmt::{Debug, Display},
};

use serde::{Serialize, Deserialize};
use itertools::join;

use crate::superprelude::*;

// ================================================================================================
// Unit variable (memory register) type
// ================================================================================================

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnitVar(usize);

impl UnitVar {
    pub fn map_var(&mut self, map: &HashMap<usize, usize>) {
        self.0 = map[&self.0];
    }
}

impl Display for UnitVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

impl TryFrom<&UnitVar> for UnitVar {
    type Error = ();

    fn try_from(unit_var: &UnitVar) -> Result<Self, ()> {
        Ok(*unit_var)
    }
}

macro_rules! impl_unit_var_try_from {
    ($(($T:ty, [$($variant:path),*$(,)*])),*$(,)*) => {
        $(
            impl TryFrom<&$T> for UnitVar {
                type Error = ();

                fn try_from(other: &$T) -> Result<Self, ()> {
                    match other {
                        $(
                            $variant(inner) => Ok(inner.try_into()?),
                        )*
                        _ => Err(()),
                    }
                }
            }

            impl TryFrom<$T> for UnitVar {
                type Error = ();

                fn try_from(other: $T) -> Result<Self, ()> {
                    Self::try_from(&other)
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_unit_var_try_from!(
    (UnitNum,    [UnitNum::Var]),
    (UnitDev,    [UnitDev::Var]),
    (UnitDevNet, [UnitDevNet::Var]),
    (UnitLine,   [UnitLine::Var]),
    (UnitReturn, [UnitReturn::Num, UnitReturn::Dev, UnitReturn::Net]),
);

// ================================================================================================
// Unit number type
// ================================================================================================

#[derive(Copy, Clone, Debug)]
pub enum UnitNum {
    Lit(f64),
    Var(UnitVar),
}

impl Display for UnitNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(n) => write!(f, "{}", n),
            Self::Var(var) => write!(f, "{}", var),
        }
    }
}

impl From<UnitVar> for UnitNum {
    fn from(var: UnitVar) -> Self {
        Self::Var(var)
    }
}

impl From<UnitDevNet> for UnitNum {
    fn from(unit_dev_net: UnitDevNet) -> Self {
        match unit_dev_net {
            UnitDevNet::Lit(h) => Self::Lit(h as f64),
            UnitDevNet::Num(n) => n,
            UnitDevNet::Var(v) => Self::Var(v),
        }
    }
}

impl TryFrom<UnitReturn> for UnitNum {
    type Error = MypsLexerError;

    fn try_from(rtn: UnitReturn) -> MypsLexerResult<Self> {
        match rtn {
            UnitReturn::Num(unit_num) => Ok(unit_num),
            UnitReturn::Var(unit_var) => Ok(UnitNum::Var(unit_var)),
            _ => Err(MypsLexerError::failed_conversion("a unit number variant unit return", rtn)),
        }
    }
}

// ================================================================================================
// Unit device type
// ================================================================================================

#[derive(Copy, Clone, Debug)]
pub enum UnitDev {
    Lit(u64),
    Var(UnitVar),
    DB,
}

impl UnitDev {
    pub fn map_var(&mut self, map: &HashMap<usize, usize>) {
        match self {
            Self::Var(unit_var) => unit_var.map_var(map),
            _ => {}
        }
    }
}

// impl From<Var> for UnitDev {
//     fn from(var: Var) -> Self {
//         match var {
//             Var::Dev(unit_dev) => unit_dev,
//             // Var::Dev() => Self::Lit(b, i),
//             // Var::Dev(UnitDev::DB) => Self::DB,
//             _ => unreachable!("{:?}", var),
//         }
//     }
// }

impl Display for UnitDev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(i) => write!(f, "d{}", i),
            Self::Var(v) => write!(f, "d{}", v),
            Self::DB => write!(f, "db"),
        }
    }
}

impl TryFrom<UnitReturn> for UnitDev {
    type Error = MypsLexerError;

    fn try_from(rtn: UnitReturn) -> MypsLexerResult<Self> {
        match rtn {
            UnitReturn::Dev(unit_dev) => Ok(unit_dev),
            _ => Err(MypsLexerError::failed_conversion("a unit device variant unit return", rtn)),
        }
    }
}

// ================================================================================================
// Unit network device type
// ================================================================================================

#[derive(Copy, Clone, Debug)]
pub enum UnitDevNet {
    Lit(i64),
    Num(UnitNum),
    Var(UnitVar),
}

impl UnitDevNet {
    pub fn map_var(&mut self, map: &HashMap<usize, usize>) {
        if let Self::Var(unit_var) = self {
            unit_var.map_var(map);
        }
    }
}

// impl From<Var> for UnitDevNet {
//     fn from(var: Var) -> Self {
//         match var {
//             Var::Num(n) => Self::Lit(n as i64),
//             Var::Var(v) => Self::Var(v),
//             _ => unreachable!("{:?}", var),
//         }
//     }
// }

impl Display for UnitDevNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(h) => write!(f, "{}", h),
            Self::Num(n) => write!(f, "{}", n),
            Self::Var(v) => write!(f, "{}", v),
        }
    }
}

impl TryFrom<UnitReturn> for UnitDevNet {
    type Error = MypsLexerError;

    fn try_from(rtn: UnitReturn) -> MypsLexerResult<Self> {
        match rtn {
            UnitReturn::Net(unit_dev_net) => Ok(unit_dev_net),
            _ => Err(MypsLexerError::failed_conversion("a unit network device variant unit return", rtn)),
        }
    }
}

// ================================================================================================
// Unit return type
// ================================================================================================

#[derive(Copy, Clone, Debug)]
pub enum UnitReturn {
    Num(UnitNum),
    Dev(UnitDev),
    Net(UnitDevNet),
    Var(UnitVar),
}

impl Display for UnitReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Num(num) => write!(f, "{}", num),
            Self::Dev(dev) => write!(f, "{}", dev),
            Self::Net(net) => write!(f, "{}", net),
            Self::Var(var) => write!(f, "{}", var),
        }
    }
}

// ================================================================================================
// UnitLine type
// ================================================================================================

#[derive(Copy, Clone, Debug)]
pub enum UnitLine {
    Lit(i64),
    Var(UnitVar),
    Indexed(usize),
    RA,
}

impl UnitLine {
    pub fn map_var(&mut self, map: &HashMap<usize, usize>) {
        if let Self::Var(var) = self {
            var.map_var(map);
        }
    }
}

impl Display for UnitLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(n) => write!(f, "{}", n),
            Self::Var(var) => write!(f, "{}", var),
            Self::Indexed(i) => write!(f, "ID:{}", i),
            Self::RA => write!(f, "ra"),
        }
    }
}

// ================================================================================================
// Arg enumeration
// ================================================================================================

// Arg enumeration (of all the above) definition and implementation helper macro.
macro_rules! def_impl_arg {
    ($($variant:ident),*) => {
        #[derive(Clone, Debug)]
        pub enum UnitArg {
            $(
                $variant($variant),
            )*
        }

        // impl Arg {
        //     pub fn map_var(&mut self, map: &HashMap<usize, usize>) {
        //         match self {
        //             $( Arg::$variant(thing) => thing.map_var(map), )*
        //         }
        //     }
        // }

        $(
            impl From<$variant> for UnitArg {
                fn from(thing: $variant) -> UnitArg {
                    UnitArg::$variant(thing)
                }
            }
        )*

        $(
            impl Into<$variant> for UnitArg {
                fn into(self) -> $variant {
                    match self {
                        Self::$variant(thing) => thing,
                        _ => unreachable!("{:?}", self),
                    }
                }
            }
        )*

        impl Display for UnitArg {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        UnitArg::$variant(thing) => write!(f, "{}", thing),
                    )*
                }
            }
        }
    }
}

// Define Arg over the following types:
def_impl_arg!(UnitVar, UnitNum, Dev, UnitDev, UnitDevNet, UnitLine, Mode, String);

impl UnitArg {
    pub fn as_unit_line_mut(&mut self) -> Option<&mut UnitLine> {
        match self {
            UnitArg::UnitLine(line) => Some(line),
            _ => None,
        }
    }
}

impl From<UnitReturn> for UnitArg {
    fn from(unit_return: UnitReturn) -> Self {
        match unit_return {
            UnitReturn::Num(unit_num) => Self::UnitNum(unit_num),
            UnitReturn::Dev(unit_dev) => Self::UnitDev(unit_dev),
            UnitReturn::Net(unit_net) => Self::UnitDevNet(unit_net),
            UnitReturn::Var(unit_var) => Self::UnitVar(unit_var),
        }
    }
}

// ================================================================================================
// Unit expression enumeration
// ================================================================================================

// Unit expression enumeration and implementation helper macro.
macro_rules! def_impl_unit_expr {
    (@try_from_pair $variant:ident, $nargs:literal, $disp:literal, $args:ident) => {{
        let n_args = $args.len();
        if n_args != $nargs {
            return Err(MypsLexerError::wrong_num_args($disp, 0, $nargs));
        }
        Ok(Self::$variant($args.try_into().unwrap()))
    }};

    ($(
        ($variant:ident, $nargs:literal, $disp:literal, $new:ident, [
            $(($argty:ty, $arg:ident)),*
        ])
    ),*$(,)*) => {
        #[derive(Clone, Debug)]
        pub enum UnitExpr {
            $(
                $variant([UnitArg; $nargs]),
            )*
            Empty,
            Dummy,
        }

        impl UnitExpr {
            $(
                pub fn $new($($arg: $argty,)*) -> Self {
                    UnitExpr::$variant([$($arg.into(),)*])
                }
            )*

            pub fn try_from_pair(name: String, args: Vec<UnitArg>) -> MypsLexerResult<Self> {
                match name.as_str() {
                    $(
                        $disp => def_impl_unit_expr!(@try_from_pair $variant, $nargs, $disp, args),
                    )*
                    "empty" => Ok(Self::Empty),
                    "dummy" => Ok(Self::Dummy),
                    _ => Err(MypsLexerError::undefined_function(&name)),
                }
            }

            /// Iterator over references to the arguments of this unit expression.
            pub fn iter_args<'a>(&'a self) -> Box<dyn Iterator<Item = &UnitArg> + 'a> {
                match self {
                    $( UnitExpr::$variant(args) => Box::new(args.iter()), )*
                    UnitExpr::Empty => Box::new(std::iter::empty()),
                    UnitExpr::Dummy => Box::new(std::iter::empty()),
                }
            }

            /// Iterator over mutable references to the arguments of this unit expression.
            pub fn iter_args_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut UnitArg> + 'a> {
                match self {
                    $( UnitExpr::$variant(args) => Box::new(args.iter_mut()), )*
                    UnitExpr::Empty => Box::new(std::iter::empty()),
                    UnitExpr::Dummy => Box::new(std::iter::empty()),
                }
            }

            /// Returns a `Some` reference to the last argument of the unit expression,
            /// or `None` if it has no arguments.
            pub fn last(&self) -> Option<&UnitArg> {
                match self {
                    $( UnitExpr::$variant(args) => args.last(), )*
                    UnitExpr::Empty => None,
                    UnitExpr::Dummy => None,
                }
            }

            /// Returns a `Some` mutable reference to the last argument of the unit expression,
            /// or `None` if it has no arguments.
            pub fn last_mut(&mut self) -> Option<&mut UnitArg> {
                match self {
                    $( UnitExpr::$variant(args) => args.last_mut(), )*
                    UnitExpr::Empty => None,
                    UnitExpr::Dummy => None,
                }
            }
        }

        impl Display for UnitExpr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $( UnitExpr::$variant(args) => {
                        write!(f, $disp)?;
                        write!(f, " {}", join(args.iter().map(UnitArg::to_string), " "))
                    },)*
                    UnitExpr::Empty => write!(f, ""),
                    UnitExpr::Dummy => write!(f, "(dummy)"),
                }
            }
        }
    }
}

#[rustfmt::skip]
def_impl_unit_expr!(
    // Device IO
    (Bdns,   2, "bdns",   new_bdns,   [(UnitDev, d), (UnitLine, l)]),
    (Bdnsal, 2, "bdnsal", new_bdnsal, [(UnitDev, d), (UnitLine, l)]),
    (Bdse,   2, "bdse",   new_bdse,   [(UnitDev, d), (UnitLine, l)]),
    (Bdseal, 2, "bdseal", new_bdseal, [(UnitDev, d), (UnitLine, l)]),
    (Brdns,  2, "brdns",  new_brdns,  [(UnitDev, d), (UnitLine, l)]),
    (Brdse,  2, "brdse",  new_brdse,  [(UnitDev, d), (UnitLine, l)]),
    (L,      3, "l",      new_l,      [(UnitVar, r), (UnitDev, d), (String, p)]),
    (Lb,     4, "lb",     new_lb,     [(UnitVar, r), (UnitDevNet, d), (String, p), (Mode, m)]),
    // (Lr
    (Ls,     4, "ls",     new_ls,     [(UnitVar, r), (UnitDev, d), (UnitNum, i), (String, p)]),
    (S,      3, "s",      new_s,      [(UnitDev, d), (String, p), (UnitNum, r)]),
    (Sb,     3, "sb",     new_sb,     [(UnitDevNet, d), (String, p), (UnitNum, r)]),

    // Flow Control, Branches and Jumps
    // (Bap
    // (Bapal
    // (Bapz
    // (Bapzal
    // (Beq
    // (Beqal
    // (Beqz
    // (Beqzal
    // (Bge
    // (Bgeal
    // (Bgez
    // (Bgezal
    // (Bgt
    // (Bgtal
    // (Bgtz
    // (Bgtzal
    // (Ble
    // (Bleal
    // (Blez
    // (Blezal
    // (Blt
    // (Bltal
    // (Bltz
    // (Bltzal
    // (Bna
    // (Bnaal
    // (Bnaz
    // (Bnazal
    // (Bne
    // (Bneal
    // (Bnez
    // (Bnezal

    // (Brap
    // (Brapz
    (Breq,   3, "breq",   new_breq,   [(UnitNum, a), (UnitNum, b), (UnitLine, l)]),
    (Breqz,  2, "breqz",  new_breqz,  [(UnitNum, a), (UnitLine, l)]),
    (Brge,   3, "brge",   new_brge,   [(UnitNum, a), (UnitNum, b), (UnitLine, l)]),
    (Brgez,  2, "brgez",  new_brgez,  [(UnitNum, a), (UnitLine, l)]),
    (Brgt,   3, "brgt",   new_brgt,   [(UnitNum, a), (UnitNum, b), (UnitLine, l)]),
    (Brgtz,  2, "brgtz",  new_brgtz,  [(UnitNum, a), (UnitLine, l)]),
    (Brle,   3, "brle",   new_brle,   [(UnitNum, a), (UnitNum, b), (UnitLine, l)]),
    (Brlez,  2, "brlez",  new_brlez,  [(UnitNum, a), (UnitLine, l)]),
    (Brlt,   3, "brlt",   new_brlt,   [(UnitNum, a), (UnitNum, b), (UnitLine, l)]),
    (Brltz,  2, "brltz",  new_brltz,  [(UnitNum, a), (UnitLine, l)]),
    // (Brna
    // (Brnaz
    (Brne,   3, "brne",   new_brne,   [(UnitNum, a), (UnitNum, b), (UnitLine, l)]),
    (Brnez,  2, "brnez",  new_brnez,  [(UnitNum, a), (UnitLine, l)]),
    (J,      1, "j",      new_j,      [(UnitLine, l)]),
    (Jal,    1, "jal",    new_jal,    [(UnitLine, l)]),
    (Jr,     1, "jr",     new_jr,     [(UnitLine, l)]),

    // Variable Selection
    // (Sap
    // (Sapz
    // (Sdns
    // (Sdse
    // (Select
    (Seq,    3, "seq",    new_seq,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Seqz
    // (Sge
    // (Sgez
    (Sgt,    3, "sgt",    new_sgt,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Sgtz
    // (Sle
    // (Slez
    (Slt,    3, "slt",    new_slt,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Sltz
    // (Sna
    // (Snaz
    (Sne,    3, "sne",    new_sne,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Snez

    // Mathematical Operations
    // (Abs
    // (Acos
    (Add,    3, "add",    new_add,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Asin
    // (Atan
    // (Ceil
    // (Cos
    (Div,    3, "div",    new_div,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Exp
    // (Floor
    // (Log
    // (Max
    // (Min
    (Mod,    3, "mod",    new_mod,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    (Mul,    3, "mul",    new_mul,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Rand
    // (Round
    // (Sin
    // (Sqrt
    (Sub,    3, "sub",    new_sub,    [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Tan
    (Trunc,  2, "trunc",  new_trunc,  [(UnitVar, r), (UnitNum, a)]),

    // Logic
    // (And
    // (Nor
    (Or,     3, "or",     new_or,     [(UnitVar, r), (UnitNum, a), (UnitNum, b)]),
    // (Xor

    // Stack
    (Peek,   1, "peek",   new_peek,   [(UnitVar, r)]),
    (Pop,    1, "pop",    new_pop,    [(UnitVar, r)]),
    (Push,   1, "push",   new_push,   [(UnitNum, a)]),

    // Misc
    (Alias,  2, "alias",  new_alias,  [(String, a), (UnitDev, d)]),
    // (Define, 2, "define", new_define, [(String, d), (UnitNum, a)]),
    (Hcf,    0, "hcf",    new_hcf,    []),
    (Move,   2, "move",   new_move,   [(UnitVar, r), (UnitNum, a)]),
    (Sleep,  1, "sleep",  new_sleep,  [(UnitNum, a)]),
    (Yield,  0, "yield",  new_yield,  []),
);

impl UnitExpr {
    /// Is this unit expresison a logical expression.
    pub fn is_logical(&self) -> bool {
        match self {
            _ => false,
        }
    }

    /// Is this unit expresison a select expression.
    pub fn is_select(&self) -> bool {
        match self {
            Self::Seq(..) | Self::Slt(..) | Self::Sgt(..) | Self::Sne(..) => true,
            _ => false,
        }
    }

    /// Is this unit expresison a logical or select expression.
    pub fn is_logical_or_select(&self) -> bool {
        self.is_logical() || self.is_select()
    }

    /// Apply map to inner Vars (i.e. as a result of register optimization)
    pub fn map_vars(&mut self, map: &HashMap<usize, usize>) {
        for arg in self.iter_args_mut() {
            #[rustfmt::skip]
            match arg {
                UnitArg::UnitVar(var)               => var.map_var(map),
                UnitArg::UnitNum(UnitNum::Var(var)) => var.map_var(map),
                UnitArg::UnitDev(unit_dev)          => unit_dev.map_var(map),
                UnitArg::UnitDevNet(unit_dev_net)   => unit_dev_net.map_var(map),
                UnitArg::UnitLine(line)             => line.map_var(map),
                _ => {},
                // _ => unreachable!("{:?}", arg),
            }
        }
    }
}

// ================================================================================================
// Unit type
// ================================================================================================

#[derive(Clone)]
pub struct Unit {
    unit_expr: UnitExpr,
    comment: Option<String>,
}

impl Unit {
    pub fn new(unit_expr: UnitExpr, comment: Option<String>) -> Self {
        Self { unit_expr, comment }
    }

    pub fn is_logical(&self) -> bool {
        self.unit_expr.is_logical()
    }

    pub fn is_select(&self) -> bool {
        self.unit_expr.is_select()
    }

    pub fn is_logical_or_select(&self) -> bool {
        self.unit_expr.is_logical_or_select()
    }

    pub fn map_vars(&mut self, map: &HashMap<usize, usize>) {
        self.unit_expr.map_vars(map);
    }

    pub fn iter_args(&self) -> impl Iterator<Item = &UnitArg> {
        self.unit_expr.iter_args()
    }

    pub fn iter_args_mut(&mut self) -> impl Iterator<Item = &mut UnitArg> {
        self.unit_expr.iter_args_mut()
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.unit_expr, &self.comment) {
            (UnitExpr::Empty, Some(comment)) => write!(f, "{}", comment),
            (UnitExpr::Empty, None         ) => write!(f, ""),
            (expr,            Some(comment)) => write!(f, "{} {}", expr, comment),
            (expr,            None         ) => write!(f, "{}", expr),
        }
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(comment) = &self.comment {
            write!(f, "{:?} ({})", self.unit_expr, comment)
        } else {
            write!(f, "{:?}", self.unit_expr)
        }
    }
}

// ================================================================================================
// UnitAlias and UnitAliasKey
// ================================================================================================

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum UnitAliasKey {
    Var(UnitVar),
    String(String),
}

// TODO: Figure out how to use Deref here
impl From<String> for UnitAliasKey {
    fn from(k: String) -> Self {
        Self::String(k)
    }
}

impl From<&String> for UnitAliasKey {
    fn from(k: &String) -> Self {
        k.clone().into()
    }
}

impl From<UnitVar> for UnitAliasKey {
    fn from(unit_var: UnitVar) -> Self {
        Self::Var(unit_var)
    }
}

impl From<&UnitVar> for UnitAliasKey {
    fn from(unit_var: &UnitVar) -> Self {
        unit_var.clone().into()
    }
}

impl TryFrom<LValue> for UnitAliasKey {
    type Error = MypsLexerError;

    fn try_from(l_value: LValue) -> MypsLexerResult<Self> {
        match l_value {
            LValue::Var(k, _) => Ok(UnitAliasKey::String(k)),
            _ => Err(MypsLexerError::failed_conversion("a unit var variant l-value", l_value)),
        }
    }
}

impl TryFrom<UnitNum> for UnitAliasKey {
    type Error = MypsLexerError;

    fn try_from(unit_num: UnitNum) -> MypsLexerResult<Self> {
        match unit_num {
            UnitNum::Var(unit_var) => Ok(unit_var.into()),
            _ => Err(MypsLexerError::failed_conversion("a unit var variant unit number", unit_num)),
        }
    }
}

impl TryFrom<UnitDev> for UnitAliasKey {
    type Error = MypsLexerError;

    fn try_from(unit_dev: UnitDev) -> MypsLexerResult<Self> {
        match unit_dev {
            UnitDev::Var(unit_var) => Ok(unit_var.into()),
            _ => Err(MypsLexerError::failed_conversion("a unit var variant unit device", unit_dev)),
        }
    }
}

impl Debug for UnitAliasKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "String({})", s),
            Self::Var(v) => write!(f, "Var({:?})", v),
        }
    }
}

#[derive(Clone)]
pub enum UnitAlias {
    Var(UnitVar),
    Num(UnitNum),
    Dev(UnitDev),
    Net(UnitDevNet),
    // Function(f64),
}

impl UnitAlias {
    pub fn try_as_var(&self) -> Option<&UnitVar> {
        match self {
            Self::Var(uv) => Some(uv),
            _ => None,
        }
    }
}

impl Debug for UnitAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Var(v) => write!(f, "Var({:?})", v),
            Self::Num(n) => write!(f, "Num({:?})", n),
            Self::Dev(d) => write!(f, "Dev({:?})", d),
            Self::Net(n) => write!(f, "Net({:?})", n),
        }
    }
}

// ================================================================================================
// Translator
// ================================================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranslatorConf {
    // Display lines that are entirely empty
    pub show_empty: bool,

    // Display comments on lines
    pub show_comments: bool,

    // Display lines that have no statement, but have comments
    pub show_empty_comments: bool,
}

impl TranslatorConf {
    pub fn show_empty(mut self) -> Self {
        self.show_empty = true;
        self
    }

    pub fn show_comments(mut self) -> Self {
        self.show_comments = true;
        self
    }

    pub fn show_empty_comments(mut self) -> Self {
        self.show_empty_comments = true;
        self
    }
}

impl Default for TranslatorConf {
    fn default() -> Self {
        Self {
            show_empty: false,
            show_comments: false,
            show_empty_comments: false,
        }
    }
}

#[derive(Debug)]
pub struct Translator {
    pub units: Vec<Unit>,
    pub aliases: HashMap<UnitAliasKey, UnitAlias>,
    pub var_next_id: usize,
    pub var_lifetimes: Vec<(usize, usize)>,
    pub vars_fixed: Vec<usize>,
    pub branch_tails: Vec<usize>,
    pub line_lookup: HashMap<String, usize>,

    pub conf: TranslatorConf,
}

mod optimize;

impl Translator {
    pub fn new(conf: TranslatorConf) -> Self {
        Self {
            units: Vec::new(),
            aliases: HashMap::new(),
            var_next_id: 0,
            var_lifetimes: Vec::new(),
            vars_fixed: Vec::new(),
            branch_tails: Vec::new(),
            line_lookup: HashMap::new(),

            conf,
        }
    }

    /// Translate a top-level item, treating it as the program block.
    pub fn translate(conf: TranslatorConf, program_item: Item, functions: HashMap<String, (Block, Option<String>)>) -> MypsLexerResult<Self> {
        let mut translator = Self::new(conf);

        // Reserve a line number for each user function
        for (i, k) in functions.keys().enumerate() {
            translator.branch_tails.push(i);
            translator.line_lookup.insert(k.to_owned(), i);
        }

        // Translate the program item
        translator.translate_item(program_item, None)?;

        // Insert the functions
        for (_, (function, comment)) in functions {
            let item = Item::block(function, comment);
            translator.translate_item(item, None)?;
            // let i = translator.line_lookup[&name];
            // translator.branch_tails[i] = translator.units.len();
        }

        // Replace the temporary UnitLine::Indexed members of If/Elif/Else units with their
        // corresponding tail lines
        let tails = translator
            .branch_tails
            .iter()
            .map(|i| UnitLine::Lit(*i as i64))
            .collect::<Vec<_>>();

        // #[rustfmt::skip]
        for arg in translator.units.iter_mut().flat_map(Unit::iter_args_mut) {
            if let UnitArg::UnitLine(line) = arg {
                if let UnitLine::Indexed(i) = line {
                    *line = tails[*i];
                }
            }
            // unit.iter
            // if let Some(line) = unit.unit_expr.last_mut().and_then(UnitArg::as_unit_line_mut) {
            //     if let UnitLine::Indexed(i) = line {
            //         *line = tails[*i];
            //     }
            // }
        }

        Ok(translator)
    }

    fn insert_alias<K: Into<UnitAliasKey>>(&mut self, k: K, alias: UnitAlias) {
        self.aliases.insert(k.into(), alias);
    }

    /// Parse, lex and translate a MYPS source string.
    pub fn parse_lex_and_translate(conf: TranslatorConf, source: &str) -> MypsLexerResult<Self> {
        let peg = MypsParser::parse(Rule::program, source)?;
        let program_pair = peg.only_inner()?;
        let (program_item, functions) = lex_program_pair(program_pair)?;
        Translator::translate(conf, program_item, functions)
    }

    fn push_unit(&mut self, unit_expr: UnitExpr, comment: Option<String>) {
        if self.conf.show_comments {
            self.units.push(Unit::new(unit_expr, comment));
        } else {
            self.units.push(Unit::new(unit_expr, None));
        }
    }

    pub fn optimize(&mut self) {}

    pub fn optimize_registers(&mut self) {
        let var_to_reg_map = optimize::registers::var_to_reg_optimizer_map(
            self.var_next_id,
            &self.var_lifetimes,
            &self.vars_fixed,
        );
        for unit in self.units.iter_mut() {
            unit.map_vars(&var_to_reg_map);
        }
    }

    // fn update_lifetime_s_unitvar(&mut self, var: &UnitVar, line: usize) {
    //     self.var_lifetimes[var.0].0 = line;
    // }

    // fn update_lifetime_unitvar(&mut self, var: &UnitVar, line: usize) {
    //     self.var_lifetimes[var.0].1 = line;
    // }

    // fn update_lifetime_s_num(&mut self, num: &UnitNum, line: usize) {
    //     if let UnitNum::Var(var) = &num {
    //         self.update_lifetime_s_unitvar(var, line);
    //     }
    // }

    // fn update_lifetime_num(&mut self, num: &UnitNum, line: usize) {
    //     if let UnitNum::Var(var) = &num {
    //         self.update_lifetime_unitvar(var, line);
    //     }
    // }

    // ============================================================================================
    // Unit variable, lookup, and lifetime helpers
    // ============================================================================================

    // Get the next unit var (and set its starting lifetime)
    fn next_var(&mut self) -> UnitVar {
        let var = UnitVar(self.var_next_id);
        let line = self.units.len();
        self.var_lifetimes.push((line, line));
        self.var_next_id += 1;
        var
    }

    fn delete_last_var(&mut self) {
        self.var_next_id -= 1;
        self.var_lifetimes.pop();
    }

    fn unwrap_var(&mut self, unit_var: Option<UnitVar>) -> UnitVar {
        if let Some(unit_var) = unit_var {
            self.update_lifetime(unit_var);
            unit_var
        } else {
            self.next_var()
        }
    }

    // Update a unit var lifetime
    fn update_lifetime<V: TryInto<UnitVar>>(&mut self, v: V) {
        if let Ok(unit_var) = v.try_into() {
            self.var_lifetimes[unit_var.0].1 = self.units.len();
        }
    }

    // Lookup a unit var (updating a var's lifetime)
    fn get_var<K: Into<UnitAliasKey> + Debug>(&mut self, k: K, fix: bool) -> UnitVar {
        let unit_var = if fix {
            self.next_var()
        } else {
            let unit_var = self.aliases
                .get(&k.into())
                .and_then(UnitAlias::try_as_var)
                .cloned();
            self.unwrap_var(unit_var)
        };

        self.update_lifetime(unit_var);
        if fix {
            self.vars_fixed.push(unit_var.0);
        }

        unit_var
    }

    fn get_dev(&mut self, r_value: RValue) -> MypsLexerResult<(UnitDev, usize)> {
        let (rv_rtn, rv_depth) = self.translate_r_value(r_value, None, &mut None)?;
        let unit_dev = match rv_rtn {
            UnitReturn::Num(unit_num) => {
                match unit_num {
                    UnitNum::Lit(n) => UnitDev::Lit(n as u64),
                    UnitNum::Var(unit_var) => UnitDev::Var(unit_var),
                }
            },
            UnitReturn::Dev(unit_dev) => unit_dev,
            // TODO: specialize this error, and/or simplify this API
            UnitReturn::Net(_unit_net) => return Err(MypsLexerError::Dummy),
            UnitReturn::Var(unit_var) => UnitDev::Var(unit_var),
        };
        Ok((unit_dev, rv_depth))
    }

    // Lookup a unit alias (updating a var's lifetime)
    fn lookup_alias<K: Into<UnitAliasKey>>(&mut self, k: K) -> MypsLexerResult<UnitAlias> {
        let k = k.into();
        let alias = self
            .aliases
            .get(&k)
            .cloned()
            .ok_or(MypsLexerError::undefined_alias(k))?;
        if let UnitAlias::Var(unit_var) = alias {
            self.update_lifetime(unit_var);
        }
        Ok(alias)
    }

    // Lookup a unit number (updating a var's lifetime)
    fn lookup_num<K: Into<UnitAliasKey>>(&mut self, k: K) -> MypsLexerResult<UnitNum> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Num(unit_num) => Ok(unit_num),
            UnitAlias::Var(var) => Ok(UnitNum::Var(var)),
            // _ => unreachable!("{:?}", alias),
            _ => Err(MypsLexerError::failed_conversion("an aliased unit number", alias)),
        }
    }

    // Lookup a unit device (updating a var's lifetime)
    fn lookup_dev<K: Into<UnitAliasKey>>(&mut self, k: K) -> MypsLexerResult<UnitDev> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Dev(unit_dev) => Ok(unit_dev),
            UnitAlias::Var(var) => Ok(UnitDev::Var(var)),
            // UnitAlias::Num(i) => UnitDev::Lit(i as usize),
            // _ => unreachable!("{:?}", alias),
            _ => Err(MypsLexerError::failed_conversion("an aliased unit device", alias)),
        }
    }

    fn lookup_dev_net<K: Into<UnitAliasKey>>(&mut self, k: K) -> MypsLexerResult<UnitDevNet> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Net(unit_dev_net) => Ok(unit_dev_net),
            UnitAlias::Var(var) => Ok(UnitDevNet::Var(var)),
            // UnitAlias::Num(i) => UnitDevNet::Lit(i as i64),
            // _ => unreachable!("{:?}", alias),
            _ => Err(MypsLexerError::failed_conversion("an aliased unit network device", alias)),
        }
    }

    fn lookup_var<K: Into<UnitAliasKey>>(&mut self, k: K) -> MypsLexerResult<UnitVar> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Var(unit_var) => Ok(unit_var),
            _ => Err(MypsLexerError::failed_conversion("an aliased unit var", alias)),
        }
    }

    // ============================================================================================
    // Translate a unit intenger (UnitInt)
    // ============================================================================================
    fn translate_int(&mut self, int: Int) -> MypsLexerResult<UnitNum> {
        match int {
            Int::Lit(n) => Ok(UnitNum::Lit(n as f64)),
            Int::Var(k) => Ok(self.lookup_num(k)?.clone()),
        }
    }

    // ============================================================================================
    // Translate a unit device (UnitDev)
    // ============================================================================================
    fn translate_dev(&mut self, dev: Dev) -> MypsLexerResult<(UnitDev, usize)> {
        match dev {
            Dev::Lit(box r_value) => {
                let (rtn, num_depth) = self.translate_r_value(r_value, None, &mut None)?;
                let num = UnitNum::try_from(rtn).unwrap();
                // let unit_dev = UnitDev
                let unit_dev = match num {
                    UnitNum::Lit(n) => UnitDev::Lit(n as u64),
                    UnitNum::Var(v) => UnitDev::Var(v),
                };
                Ok((unit_dev, num_depth))
            }
            Dev::Var(k) => Ok((self.lookup_dev(k).unwrap().clone(), 0)),
            Dev::DB => Ok((UnitDev::DB, 0)),
            _ => unreachable!("{:?}", dev),
        }
    }

    // ============================================================================================
    // Translate a unit network device (UnitDevNet)
    // ============================================================================================
    fn translate_dev_net(&mut self, dev: Dev) -> MypsLexerResult<(UnitDevNet, usize)> {
        match dev {
            Dev::Net(box r_value) => {
                let (rtn, num_depth) = self.translate_r_value(r_value, None, &mut None)?;
                let num = UnitNum::try_from(rtn).unwrap();
                let unit_dev_net = match num {
                    UnitNum::Lit(n) => UnitDevNet::Lit(n as i64),
                    UnitNum::Var(v) => UnitDevNet::Var(v),
                };
                Ok((unit_dev_net, num_depth))
            }
            Dev::Var(k) => Ok((self.lookup_dev_net(k).unwrap().clone(), 0)),
            _ => unreachable!("{:?}", dev),
        }
    }

    // fn return_to_u64(&self, rtn: UnitReturn) -> u64 {
    //     match rtn {
    //         UnitReturn::Dev(..) => unreachable!(),
    //         UnitReturn::Net(..) => unreachable!(),
    //         UnitReturn::Num(num) => {
    //             match num {
    //                 Num::Lit(n) => n as u64,
    //                 Num::Var(v) => unreachable!(),
    //             }
    //         },
    //     }
    // }

    // ============================================================================================
    // Reduce an alias
    // ============================================================================================
    // fn reduce_alias<K: Into<UnitAliasKey>>(&mut self, k: K, line: usize) -> MypsLexerResult<UnitReturn> {
    //     let alias = self.lookup_alias(k.into(), line)?.clone();
    //     match alias {
    //         UnitAlias::Num(unit_num) => Ok(UnitReturn::Num(unit_num)),
    //         UnitAlias::Dev(unit_dev) => Ok(UnitReturn::Dev(unit_dev)),
    //         UnitAlias::Net(unit_dev_net) => Ok(UnitReturn::Net(unit_dev_net)),
    //         UnitAlias::Var(unit_var) => {
    //             self.reduce_alias(unit_var, line)
    //         },
    //     }
    // }

    // ============================================================================================
    // Translate an r-value
    // ============================================================================================
    fn translate_r_value(
        &mut self,
        r_value: RValue,
        unit_var: Option<UnitVar>,
        comment: &mut Option<String>,
    ) -> MypsLexerResult<(UnitReturn, usize)> {
        match r_value {
            RValue::Num(num) => {
                let num = match num {
                    Num::Lit(n) => UnitNum::Lit(n),
                    Num::Var(k) => {
                        let num = self.lookup_num(k)?;
                        // self.update_lifetime_mem(&mem);
                        num.clone()
                    }
                };
                Ok((UnitReturn::Num(num), 0))
            }
            RValue::Dev(dev) => match dev {
                Dev::Lit(box id_r_value) => {
                    let (id_rtn, depth) = self.translate_r_value(id_r_value, None, comment)?;
                    let unit_dev = match id_rtn {
                        UnitReturn::Num(UnitNum::Lit(n)) => UnitDev::Lit(n as u64),
                        UnitReturn::Num(UnitNum::Var(v)) => UnitDev::Var(v),
                        _ => unreachable!("{:?}", id_rtn),
                    };
                    Ok((UnitReturn::Dev(unit_dev), depth))
                }
                Dev::Net(box hash_r_value) => {
                    let (hash_rtn, depth) = self.translate_r_value(hash_r_value, None, comment)?;
                    let unit_dev_net = match hash_rtn {
                        UnitReturn::Num(UnitNum::Lit(n)) => UnitDevNet::Lit(n as i64),
                        UnitReturn::Num(UnitNum::Var(v)) => UnitDevNet::Var(v),
                        _ => unreachable!("{:?}", hash_rtn),
                    };
                    Ok((UnitReturn::Net(unit_dev_net), depth))
                }
                Dev::DB => {
                    Ok((UnitReturn::Dev(UnitDev::DB), 0))
                }
                Dev::Var(k) => {
                    let alias = self.lookup_alias(k)?;
                    let unit_return = match alias {
                        UnitAlias::Num(..) => unreachable!("{:?}", alias),
                        UnitAlias::Dev(unit_dev) => UnitReturn::Dev(unit_dev),
                        UnitAlias::Net(unit_dev_net) => UnitReturn::Net(unit_dev_net),
                        UnitAlias::Var(..) => unreachable!("{:?}", alias),
                    };
                    Ok((unit_return, 0))
                }
            },
            RValue::NetParam(dev, mode, param) => {
                let (dev, dev_depth) = self.translate_dev_net(dev)?;
                let var = self.unwrap_var(unit_var);
                let unit_expr = UnitExpr::new_lb(var, dev, param, mode);
                self.push_unit(unit_expr, comment.take());
                Ok((UnitReturn::Var(var), 1 + dev_depth))
            }
            RValue::DevParam(dev, param) => {
                let (dev, dev_depth) = self.translate_dev(dev)?;
                let var = self.unwrap_var(unit_var);
                let unit_expr = UnitExpr::new_l(var, dev, param);
                self.push_unit(unit_expr, comment.take());
                Ok((UnitReturn::Var(var), 1 + dev_depth))
            }
            RValue::DevSlot(dev, slot, param) => {
                let (dev, dev_depth) = self.translate_dev(dev)?;
                let slot = self.translate_int(slot)?;
                let var = self.unwrap_var(unit_var);
                let unit_expr = UnitExpr::new_ls(var, dev, slot, param);
                self.push_unit(unit_expr, comment.take());
                Ok((UnitReturn::Var(var), 1 + dev_depth))
            }
            RValue::Expr(box expr) => {
                let (unit_num, depth) = self.translate_expr(expr, unit_var, comment)?;
                Ok((UnitReturn::Num(unit_num), depth))
            }
            RValue::Func(rv_func, r_values) => {
                let (mut arg_returns, arg_depths): (Vec<UnitReturn>, Vec<usize>) = r_values
                    .into_iter()
                    .map(|r_value| self.translate_r_value(r_value, None, comment))
                    .collect::<MypsLexerResult<Vec<(UnitReturn, usize)>>>()?
                    .into_iter()
                    .unzip();

                macro_rules! new_rv_func {
                    (0, $name:path, $r:ident, $arg_returns:ident) => {{
                        $name($r)
                    }};
                    (1, $name:path, $r:ident, $arg_returns:ident) => {{
                        let a = $arg_returns.pop().ok_or(MypsLexerError::Dummy)?.try_into()?;
                        $name($r, a)
                    }};
                    (2, $name:path, $r:ident, $arg_returns:ident) => {{
                        let a = $arg_returns.pop().ok_or(MypsLexerError::Dummy)?.try_into()?;
                        let b = $arg_returns.pop().ok_or(MypsLexerError::Dummy)?.try_into()?;
                        $name($r, a, b)
                    }};
                }

                let r = self.unwrap_var(unit_var);
                let unit_expr = match rv_func {
                    RVFunc::Trunc => new_rv_func!(1, UnitExpr::new_trunc, r, arg_returns),
                    // RVFunc::Trunc => {
                    //     let a = arg_returns.pop().unwrap().try_into().unwrap();
                    //     UnitExpr::new_trunc(r, a)
                    // },
                    _ => unreachable!("{:?}", rv_func),
                };
                self.push_unit(unit_expr, comment.take());

                Ok((UnitReturn::Var(r), arg_depths.into_iter().sum::<usize>() + 1))
            }
            RValue::Var(k) => {
                let unit_alias = self.lookup_alias(k)?;
                let unit_return = match unit_alias {
                    UnitAlias::Num(unit_num) => UnitReturn::Num(unit_num),
                    UnitAlias::Dev(unit_dev) => UnitReturn::Dev(unit_dev),
                    UnitAlias::Net(unit_dev_net) => UnitReturn::Net(unit_dev_net),
                    UnitAlias::Var(unit_var) => UnitReturn::Var(unit_var),
                };
                // self.update_lifetime_unitvar(&unit_var, line);
                Ok((unit_return, 0))
                // (self.reduce_alias(k).unwrap(), 0)
            }
        }
    }

    // ============================================================================================
    // Translate an expression
    // ============================================================================================
    fn translate_expr(
        &mut self,
        expr: Expr,
        unit_var: Option<UnitVar>,
        comment: &mut Option<String>,
    ) -> MypsLexerResult<(UnitNum, usize)> {

        match expr {
            Expr::Unary { op, box rhs } => {
                unreachable!("{:?} {:?}", op, rhs);
            }
            Expr::Binary {
                op,
                box lhs,
                box rhs,
            } => {
                // In any case, an expression adds one line to the depth
                let mut depth = 1;
                // depth += 1;
                // Translate the lhs and rhs expressions
                // If they are num r-values the the depth is zero and the
                let (a, d_a) = self.translate_expr(lhs, None, &mut None)?;
                let (b, d_b) = self.translate_expr(rhs, None, &mut None)?;
                depth += d_a + d_b;

                fn bool_to_num(cond: bool) -> f64 {
                    if cond {
                        1.0
                    } else {
                        0.0
                    }
                }

                match (a, b) {
                    (UnitNum::Lit(l), UnitNum::Lit(r)) => {
                        let unit_num = match op {
                            BinaryOp::Add => UnitNum::Lit(l + r),
                            BinaryOp::Sub => UnitNum::Lit(l - r),
                            BinaryOp::Mul => UnitNum::Lit(l * r),
                            BinaryOp::Div => UnitNum::Lit(l / r),
                            BinaryOp::Rem => UnitNum::Lit(l.rem_euclid(r)),

                            BinaryOp::EQ => UnitNum::Lit(bool_to_num(l == r)),
                            BinaryOp::GE => UnitNum::Lit(bool_to_num(l >= r)),
                            BinaryOp::GT => UnitNum::Lit(bool_to_num(l > r)),
                            BinaryOp::LE => UnitNum::Lit(bool_to_num(l <= r)),
                            BinaryOp::LT => UnitNum::Lit(bool_to_num(l < r)),
                            BinaryOp::NE => UnitNum::Lit(bool_to_num(l != r)),
                            _ => unreachable!("{:?}", op),
                        };
                        Ok((unit_num, depth))
                    }
                    _ => {
                        let r = self.unwrap_var(unit_var);
                        self.update_lifetime(a);
                        self.update_lifetime(b);
                        // Append this expression
                        let unit_expr = match op {
                            BinaryOp::Add => UnitExpr::new_add(r, a, b),
                            BinaryOp::Sub => UnitExpr::new_sub(r, a, b),
                            BinaryOp::Mul => UnitExpr::new_mul(r, a, b),
                            BinaryOp::Div => UnitExpr::new_div(r, a, b),
                            BinaryOp::Rem => UnitExpr::new_mod(r, a, b),

                            BinaryOp::EQ  => UnitExpr::new_seq(r, a, b),
                            BinaryOp::GT  => UnitExpr::new_sgt(r, a, b),
                            BinaryOp::LT  => UnitExpr::new_slt(r, a, b),
                            BinaryOp::NE  => UnitExpr::new_sne(r, a, b),

                            BinaryOp::Or  => UnitExpr::new_or (r, a, b),
                            _ => unreachable!("{:?}", op),
                        };
                        self.push_unit(unit_expr, comment.take());
                        Ok((r.into(), depth))
                    }
                }
            }
            Expr::Ternary { cond, if_t, if_f } => {
                unreachable!("{:?} {:?} {:?}", cond, if_t, if_f);
            }
            Expr::RValue(rv) => {
                let (rv_rtn, depth) = self.translate_r_value(rv, unit_var, comment)?;
                match rv_rtn {
                    UnitReturn::Num(num) => Ok((num, depth)),
                    UnitReturn::Var(unit_var) => {
                        self.update_lifetime(unit_var);
                        Ok((UnitNum::Var(unit_var), depth))
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    // ============================================================================================
    // Translate items
    // ============================================================================================

    fn translate_items(&mut self, items: Vec<Item>, mut first_comment: Option<String>) -> MypsLexerResult<usize> {
        let mut depth = 0;
        for item in items.into_iter() {
            depth += self.translate_item(item, first_comment.take())?;
        };
        Ok(depth)
    }

    // ============================================================================================
    // Translate an item
    // ============================================================================================

    fn translate_item(&mut self, item: Item, first_comment: Option<String>) -> MypsLexerResult<usize> {
        let Item {
            item_inner,
            comment,
            ..
        } = item;

        let mut comment = first_comment.or(comment);

        match item_inner {
            ItemInner::Block(Block { branch, items }) => {

                match branch {
                    Branch::Program => self.translate_items(items, comment),
                    // ============================================================================
                    // Loop (infinitely)
                    // ============================================================================
                    Branch::Loop => {
                        let depth = self.translate_items(items, None)?;
                        let line = UnitLine::Lit(-(depth as i64));
                        let unit_expr = UnitExpr::new_jr(line);
                        self.push_unit(unit_expr, comment);
                        Ok(depth + 1)
                    }
                    // ============================================================================
                    // If/elif/else branching
                    // ============================================================================
                    Branch::If(id, _) | Branch::Elif(id, _) | Branch::Else(id) => {
                        let mut depth = 0;
                        // (Elif/Else)
                        // Add a branch to tail id in case the prevous if/elif succeeded
                        match branch {
                            Branch::Elif(..) | Branch::Else(..) => {
                                let tail_id = UnitLine::Indexed(id);
                                let unit_expr = UnitExpr::new_jr(tail_id);
                                if matches!(branch, Branch::Else(..)) {
                                    self.push_unit(unit_expr, comment.take());
                                } else {
                                    self.push_unit(unit_expr, None);
                                };
                                depth += 1;
                            }
                            _ => {}
                        }
                        // (If/Elif)
                        // Translate the condition expression and save the index of the final
                        // condition expression unit, the var memory it stored its value to,
                        // and whether the condition was an expression or an option (i.e. if its
                        // depth is greater than zero)
                        let cond_opt = match branch {
                            Branch::If(_, cond) | Branch::Elif(_, cond) => {
                                let (i, num, cond_depth) =
                                    self.translate_condition(cond, comment)?;
                                depth += cond_depth;
                                Some((i, num))
                            }
                            _ => None,
                        };
                        // Translate branch body
                        let body_depth = self.translate_items(items, None)?;
                        depth += body_depth;
                        // (If/Elif)
                        if let Some((i, num)) = cond_opt {
                            self.transform_condition(i, num, body_depth);
                        }
                        // Update the branch tail for this index
                        let tail = self.units.len() + depth;
                        if let Some(line) = self.branch_tails.get_mut(id) {
                            *line = tail;
                        } else {
                            self.branch_tails.push(tail);
                        }
                        Ok(depth)
                    }
                    // ============================================================================
                    // While loop
                    // ============================================================================
                    // TODO: Same as if, with tail branch
                    Branch::While(cond) => {
                        let mut depth = 0;

                        let (i, num, cond_depth) = self.translate_condition(cond, comment)?;
                        depth += cond_depth;

                        // Translate branch body
                        let body_depth = self.translate_items(items, None)?;
                        depth += body_depth;

                        self.transform_condition(i, num, 1 + body_depth);

                        // (Branch statements)
                        let line = UnitLine::Lit(-(1 + body_depth as i64));
                        let unit_expr = UnitExpr::new_jr(line);
                        self.push_unit(unit_expr, None);
                        depth += 1;

                        Ok(depth)
                    }

                    // ============================================================================
                    // For loop
                    // ============================================================================
                    Branch::For(i, s, e, step_opt) => {
                        let mut depth = 0;
                        // (Start value expression)
                        let (s_num, s_depth) = self.translate_expr(s, None, &mut None)?;
                        depth += s_depth;

                        let i_var = match s_num {
                            UnitNum::Lit(..) => {
                                let i_var = self.get_var(&i, true);
                                self.insert_alias(i, UnitAlias::Var(i_var));

                                let unit_expr = UnitExpr::new_move(i_var, s_num);
                                self.push_unit(unit_expr, comment.take());
                                depth += 1;
                                i_var
                            },
                            UnitNum::Var(unit_var) => {
                                unit_var
                            },
                        };

                        // if depth == 0 {
                        //     let comment = Some(format!("# {} = ({} = {})", i, i_var, s_num));
                        //     let unit_expr = UnitExpr::new_move(i_var, s_num);
                        //     self.push_unit(unit_expr, comment);
                        //     depth += 1;
                        // }
                        // let i_unit = UnitVar::Var(i_var);
                        // Add new "i" var to lookup
                        // self.insert_alias(i, UnitAlias::Var(i_var));
                        let i_num = UnitNum::Var(i_var);

                        // (Body)
                        let mut inner_depth = self.translate_items(items, None)?;

                        // (End value expression)
                        let (e_num, e_depth) = self.translate_expr(e, None, &mut None)?;
                        inner_depth += e_depth;

                        // (Step value expression)
                        let step = if let Some(step_expr) = step_opt {
                            let (step_rtn, step_depth) =
                                self.translate_expr(step_expr, None, &mut None)?;
                            inner_depth += step_depth;
                            UnitNum::try_from(step_rtn).unwrap()
                        } else {
                            UnitNum::Lit(1.0)
                        };

                        // (Increment and branch statements)
                        let unit_expr = UnitExpr::new_add(i_var, i_num, step);
                        self.push_unit(unit_expr, None);
                        inner_depth += 1;

                        let line = UnitLine::Lit(-(inner_depth as i64));
                        let unit_expr = UnitExpr::new_brlt(i_num, e_num, line);
                        self.push_unit(unit_expr, comment);
                        depth += 1;

                        Ok(depth + inner_depth)
                    }
                    // ============================================================================
                    // Function definitions
                    // ============================================================================
                    // Branch::Def(name) => {
                    //     // Dummy unit for later JR
                    //     self.units.push(Unit::new(UnitExpr::Dummy, None));
                    //     // Body
                    //     let depth = self.translate_items(items, line);
                    //     self.var_lookup
                    //         .insert(name, Var::Function((1 + line) as f64));
                    //     // Replace dummy unit with jump relative to skip the body
                    //     let i = self.units.len() - depth - 1;
                    //     let line = UnitLine::UnitVar(UnitVar::Num((2 + depth) as f64));
                    //     self.units[i].unit_expr = UnitExpr::new_jr(line);
                    //     // Jump to previous location
                    //     let line = UnitLine::UnitVar(UnitVar::RA);
                    //     self.units.push(Unit::new(UnitExpr::new_j(line), None));
                    //     depth + 2
                    // }
                    Branch::Function(name) => {
                        let i = self.line_lookup[&name];
                        let line = self.units.len();
                        self.branch_tails[i] = line;
                        let depth = 1 + self.translate_items(items, comment)?;
                        let unit_expr = UnitExpr::new_j(UnitLine::RA);
                        self.push_unit(unit_expr, None);
                        Ok(depth)
                    }
                    _ => unreachable!("{:?}", branch),
                }
                // block.items()
            }
            ItemInner::Stmt(stmt) => {
                self.translate_statement(stmt, &mut comment)
            },
            // _ => unreachable!("{:?}", stmt),
        }
    }

    fn translate_condition(
        &mut self,
        cond: Expr,
        comment: Option<String>,
    ) -> MypsLexerResult<(usize, UnitNum, usize)> {
        let (rtn, mut depth) = self.translate_expr(cond, None, &mut None)?;
        let num = UnitNum::try_from(rtn).unwrap();
        if depth == 0
            || !self
                .units
                .last()
                .map(Unit::is_logical_or_select)
                .unwrap_or(false)
        {
            self.push_unit(UnitExpr::Dummy, comment);
            depth += 1;
        }
        // else {
        //     // Else, by replacing the select expresison by a branch
        //     // expression, the var that the select expression assigns to
        //     // is not needed
        //     self.var_next_id -= 1;
        //     self.var_lifetimes.pop();
        // }
        let i = self.units.len() - 1;
        Ok((i, num, depth))
    }

    // Helper to translate a condition of an if/elif/while block
    //
    // * `i` - Index of the final unit of the condition expression
    // * `num` - UnitNum result of translating the condition expression
    // * `body_depth` - Depth of the block body
    fn transform_condition(&mut self, i: usize, num: UnitNum, body_depth: usize) {
        let c = UnitLine::Lit((body_depth + 1) as i64);
        // Convert the final condition expression unit (a variable selection
        // expression unit or a dummyunit ) into the appropriate branching
        // expression unit. Note that conditionals need to be negated
        let cond_expr = {
            let unit_expr = self.units[i].unit_expr.clone();
            match unit_expr {
                // set if not equal    -> branch if equal
                UnitExpr::Sne([_, a, b]) => UnitExpr::new_breq(a.into(), b.into(), c),
                // set if greater than -> branch if less-than or equal
                UnitExpr::Sgt([_, a, b]) => UnitExpr::new_brle(a.into(), b.into(), c),
                // set if less than -> branch if greater-than or equal
                UnitExpr::Slt([_, a, b]) => UnitExpr::new_brge(a.into(), b.into(), c),
                // non-relational expr -> branch if equal to zero
                // UnitExpr::Seq(
                UnitExpr::Dummy => UnitExpr::new_breqz(num, c),
                _ => UnitExpr::new_breqz(num, c),
                // _ => unreachable!("{:?}", unit_expr),
            }
        };
        self.units[i].unit_expr = cond_expr;
    }

    // ============================================================================================
    // Translate a statement
    // ============================================================================================

    fn translate_statement(
        &mut self,
        stmt: Statement,
        comment: &mut Option<String>,
    ) -> MypsLexerResult<usize> {
        let stmt_string = format!("{:#?}", stmt);
        match self.translate_statement_helper(stmt, comment) {
            Err(err) => {
                Err(MypsLexerError::stmt_error(stmt_string, err))
            },
            depth @ _ => depth,
        }
    }

    fn translate_statement_helper(
        &mut self,
        stmt: Statement,
        comment: &mut Option<String>,
    ) -> MypsLexerResult<usize> {
        match stmt {
            // ============================================================================
            // Assign a new alias
            // ============================================================================
            // Statement::AssignAlias(a, d) => {
            //     match d {
            //         Dev::Lit(_) => {
            //             let (unit_dev, dev_depth) = self.translate_dev(d, line);
            //             let alias = UnitAlias::Dev(unit_dev);
            //             self.insert_alias(a, alias);
            //             dev_depth
            //         },
            //         Dev::Net(_) => {
            //             let (unit_dev_net, dev_depth) = self.translate_dev_net(d, line);
            //             let alias = UnitAlias::Net(unit_dev_net);
            //             self.insert_alias(a, alias);
            //             dev_depth
            //         }
            //         Dev::Var(v) => {
            //             unreachable!();
            //         }
            //         Dev::DB => {
            //             let alias = UnitAlias::Dev(UnitDev::DB);
            //             self.insert_alias(a, alias);
            //             1
            //         }
            //     }
            // }
            // ============================================================================
            // Assign a value to an existing variable or device parameter
            // ============================================================================
            Statement::AssignValue(l_values, r_values) => {
                let mut depth = 0;

                for (l_value, r_value) in l_values.into_iter().zip(r_values.into_iter()) {
                    depth += self.translate_assignment(l_value, r_value, comment)?;
                }

                // let unit_vars = l_values.iter().map(|l_value| {
                //     if let LValue::Var(k, _) = l_value {
                //         let key = UnitAliasKey::String(k.to_owned());
                //         if let Some(UnitAlias::Var(unit_var)) = self.aliases.get(&key) {
                //             Some(*unit_var)
                //         } else {
                //             None
                //         }
                //     } else {
                //         None
                //     }
                // }).collect::<Vec<_>>();

                // let rv_returns = r_values
                //     .into_iter()
                //     .zip(unit_vars.into_iter())
                //     .map(|(r_value, unit_var)| {
                //         let (rv_rtn, rv_depth) =
                //             self.translate_r_value(r_value, unit_var, comment);
                //         total_rv_depth += rv_depth;
                //         (rv_rtn, total_rv_depth)
                //     })
                //     .collect::<Vec<(UnitReturn, usize)>>();

                // let mut depth = 0;
                // for (lv, (rv_rtn, rv_depth)) in l_values.into_iter().zip(rv_returns) {
                //     depth += rv_depth + self.translate_assignment(lv, rv_rtn, comment);
                // }

                Ok(depth)
            }
            // ============================================================================
            // Assign to a var itself plus/minus/times/divide/mod a value
            // ============================================================================
            Statement::AssignSelf(op, l_value, r_value) => {
                let unit_alias_key = UnitAliasKey::try_from(l_value)?;
                let r = self.lookup_var(unit_alias_key)?;
                let a = UnitNum::Var(r);

                let (rv_rtn, rv_depth) = self.translate_r_value(r_value, Some(r), comment)?;
                let b = match rv_rtn {
                    UnitReturn::Num(unit_num) => unit_num,
                    UnitReturn::Var(unit_var) => UnitNum::Var(unit_var),
                    _ => unreachable!("{:?}", rv_rtn),
                };
                let unit_expr = match op {
                    BinaryOp::Add => UnitExpr::new_add(r, a, b),
                    BinaryOp::Sub => UnitExpr::new_sub(r, a, b),
                    BinaryOp::Mul => UnitExpr::new_mul(r, a, b),
                    BinaryOp::Div => UnitExpr::new_div(r, a, b),
                    BinaryOp::Rem => UnitExpr::new_mod(r, a, b),
                    _ => unreachable!("{:?}", op),
                };

                self.push_unit(unit_expr, comment.take());

                Ok(rv_depth + 1)
                // let rv = match self.translate
                // match op {
                // }
            },
            // ============================================================================
            // A lone function call
            // ============================================================================
            Statement::FunctionCall(FunctionCall::Nullary(name)) => {
                let unit_expr = UnitExpr::try_from_pair(name, vec![])?;
                self.push_unit(unit_expr, comment.take());
                Ok(1)
            }
            Statement::FunctionCall(FunctionCall::Unary(name, r_value)) => {
                let (rv_return, rv_depth) = self.translate_r_value(r_value, None, &mut None)?;
                let unit_expr = UnitExpr::try_from_pair(name, vec![rv_return.into()])?;
                self.push_unit(unit_expr, comment.take());
                Ok(1 + rv_depth)
            }
            Statement::FunctionCall(FunctionCall::User(name)) => {
                let i = self.line_lookup.get(&name).unwrap();
                let line = UnitLine::Indexed(*i);
                let unit_expr = UnitExpr::new_jal(line);
                self.push_unit(unit_expr, comment.take());
                Ok(1)
            }
            //     let alias = self.var_lookup.get(&name).unwrap();
            //     match alias {
            //         Var::Function(line) => {
            //             let line = UnitLine::UnitVar(UnitVar::Num(*line));
            //             let unit_expr = UnitExpr::new_jal(line);
            //             let unit = Unit::new(unit_expr, comment);
            //             self.units.push(unit);
            //         }
            //         _ => unreachable!("{:?}", alias),
            //     }
            //     1
            Statement::Empty => {
                let depth = if self.conf.show_empty {
                    self.push_unit(UnitExpr::Empty, comment.take());
                    1
                } else if self.conf.show_empty_comments && comment.is_some() {
                    self.push_unit(UnitExpr::Empty, comment.take());
                    1
                } else {
                    0
                };
                Ok(depth)
            }
        }
    }

    // ============================================================================================
    // Translate an assignment statement
    // ============================================================================================

    fn translate_assignment(
        &mut self,
        l_value: LValue,
        r_value: RValue,
        comment: &mut Option<String>,
        ) -> MypsLexerResult<usize> {
        let mut depth = 0;

        match l_value {
            LValue::Var(k, fix) => {
                let unit_var = self.get_var(&k, fix);

                let (rv_return, rv_depth) = self.translate_r_value(r_value, Some(unit_var), comment)?;
                depth += rv_depth;

                match rv_return {
                    UnitReturn::Dev(unit_dev) => {
                        // Write a value to a device
                        if fix {
                            let unit_expr = UnitExpr::new_alias(k.clone(), unit_dev);
                            self.push_unit(unit_expr, comment.take());
                            depth += 1;
                        }
                        let alias = UnitAlias::Dev(unit_dev);
                        self.insert_alias(k, alias);
                        self.delete_last_var();// TODO DO FOR NON-FIXED NUMBERS RESULTS
                    },
                    UnitReturn::Net(unit_dev_net) => {
                        // Write a value to network devices
                        let alias = UnitAlias::Net(unit_dev_net);
                        self.insert_alias(k, alias);
                    },
                    UnitReturn::Num(..) | UnitReturn::Var(..) => {
                        // Write a value to memory
                        let unit_num = match rv_return {
                            UnitReturn::Num(unit_num) => unit_num,
                            UnitReturn::Var(unit_var) => UnitNum::Var(unit_var),
                            _ => unreachable!(),
                        };
                        if fix {
                            // If marked fix
                            // - insert an alias from the var to the number, and
                            // - insert an alias from the name to the var
                            let unit_expr = UnitExpr::new_move(unit_var, unit_num);
                            self.push_unit(unit_expr, comment.take());
                            self.insert_alias(unit_var, UnitAlias::Num(unit_num));
                            self.insert_alias(k, UnitAlias::Var(unit_var));
                            depth += 1;
                        } else {
                            // Else if not marked fix
                            // - insert an alias from the name to the number
                            self.insert_alias(k, UnitAlias::Num(unit_num));
                            // self.delete_last_var();
                        }
                    },
                }
            },
            LValue::Param(dev, param) => {
                let (rv_return, rv_depth) = self.translate_r_value(r_value, None, comment)?;
                if rv_depth == 0 {
                    // self.var_next_id -= 1;
                    // self.var_lifetimes.pop();
                }
                depth += rv_depth;

                match rv_return {
                    UnitReturn::Num(..) | UnitReturn::Var(..) => {
                        let unit_num = match &rv_return {
                            UnitReturn::Num(unit_num) => *unit_num,
                            UnitReturn::Var(unit_var) => UnitNum::Var(*unit_var),
                            _ => unreachable!(),
                        };

                        let unit_expr = match dev {
                            Dev::Lit(box id_rv) => {
                                let (unit_dev, rv_depth) = self.get_dev(id_rv)?;
                                depth += rv_depth;
                                UnitExpr::new_s(unit_dev, param, unit_num)
                            }
                            Dev::Net(box hash_rv) => {
                                let (rtn, rv_depth) =
                                    self.translate_r_value(hash_rv, None, &mut None)?;
                                let unit_dev_net = UnitDevNet::try_from(rtn)?;
                                depth += rv_depth;
                                UnitExpr::new_sb(unit_dev_net, param, unit_num)
                            }
                            Dev::Var(k) => {
                                let alias = self.aliases.get(&k.into());
                                match alias {
                                    Some(UnitAlias::Dev(unit_dev)) => {
                                        UnitExpr::new_s(*unit_dev, param, unit_num)
                                    }
                                    Some(UnitAlias::Net(unit_dev_net)) => {
                                        UnitExpr::new_sb(*unit_dev_net, param, unit_num)
                                    }
                                    Some(UnitAlias::Num(hash)) => {
                                        let unit_dev_net = UnitDevNet::Num(*hash);
                                        UnitExpr::new_sb(unit_dev_net, param, unit_num)
                                    }
                                    _ => unreachable!("{:?}", alias),
                                }
                            }
                            Dev::DB => UnitExpr::new_s(UnitDev::DB, param, unit_num),
                        };
                        self.push_unit(unit_expr, comment.take());
                        depth += 1;
                    }
                    _ => unreachable!(),
                }
            },
        }
        Ok(depth)
    }
}
