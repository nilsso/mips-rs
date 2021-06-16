#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::HashMap;
use std::{
    fmt,
    fmt::{Debug, Display},
};
use std::convert::{TryFrom, TryInto};

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
            UnitDevNet::Lit(num) => Self::Lit(num as f64),
            UnitDevNet::Var(var) => Self::Var(var),
        }
    }
}

impl TryFrom<UnitReturn> for UnitNum {
    type Error = ();

    fn try_from(rtn: UnitReturn) -> Result<Self, ()> {
        if let UnitReturn::Num(unit_num) = rtn {
            Ok(unit_num)
        } else {
            Err(())
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
    type Error = ();

    fn try_from(rtn: UnitReturn) -> Result<Self, ()> {
        if let UnitReturn::Dev(unit_dev) = rtn {
            Ok(unit_dev)
        } else {
            Err(())
        }
    }
}

// ================================================================================================
// Unit network device type
// ================================================================================================

#[derive(Copy, Clone, Debug)]
pub enum UnitDevNet {
    Lit(i64),
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
            Self::Lit(hash) => write!(f, "{}", hash),
            Self::Var(v) => write!(f, "{}", v),
        }
    }
}

impl TryFrom<UnitReturn> for UnitDevNet {
    type Error = ();

    fn try_from(rtn: UnitReturn) -> Result<Self, ()> {
        if let UnitReturn::Net(unit_dev_net) = rtn {
            Ok(unit_dev_net)
        } else {
            Err(())
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

// ================================================================================================
// Unit expression enumeration
// ================================================================================================

// Unit expression enumeration and implementation helper macro.
macro_rules! def_impl_unit_expr {
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
            Dummy,
        }

        impl UnitExpr {
            $(
                pub fn $new($($arg: $argty,)*) -> Self {
                    UnitExpr::$variant([$($arg.into(),)*])
                }
            )*

            /// Iterator over references to the arguments of this unit expression.
            pub fn iter_args<'a>(&'a self) -> Box<dyn Iterator<Item = &UnitArg> + 'a> {
                match self {
                    $( UnitExpr::$variant(args) => Box::new(args.iter()), )*
                    UnitExpr::Dummy => Box::new(std::iter::empty()),
                }
            }

            /// Iterator over mutable references to the arguments of this unit expression.
            pub fn iter_args_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut UnitArg> + 'a> {
                match self {
                    $( UnitExpr::$variant(args) => Box::new(args.iter_mut()), )*
                    UnitExpr::Dummy => Box::new(std::iter::empty()),
                }
            }

            /// Returns a `Some` reference to the last argument of the unit expression,
            /// or `None` if it has no arguments.
            pub fn last(&self) -> Option<&UnitArg> {
                match self {
                    $( UnitExpr::$variant(args) => args.last(), )*
                    UnitExpr::Dummy => None,
                }
            }

            /// Returns a `Some` mutable reference to the last argument of the unit expression,
            /// or `None` if it has no arguments.
            pub fn last_mut(&mut self) -> Option<&mut UnitArg> {
                match self {
                    $( UnitExpr::$variant(args) => args.last_mut(), )*
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
    // (Trunc

    // Logic
    // (And
    // (Nor
    // (Or
    // (Xor

    // Stack
    // (Peek
    // (Pop
    // (Push

    // Misc
    (Alias,  2, "alias",  new_alias,  [(String, a), (UnitDev, d)]),
    // (Define
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
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(comment) = &self.comment {
            write!(f, "{} {}", self.unit_expr, comment)
        } else {
            write!(f, "{}", self.unit_expr)
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
// Translator
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

impl Debug for UnitAliasKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "String({})", s),
            Self::Var(v)    => write!(f, "Var({:?})", v),
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
            Self::Var(v)    => write!(f, "Var({:?})", v),
            Self::Num(n)    => write!(f, "Num({:?})", n),
            Self::Dev(d)    => write!(f, "Dev({:?})", d),
            Self::Net(n)    => write!(f, "Net({:?})", n),
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
}

mod optimize;
use optimize::registers;

impl Translator {
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
            aliases: HashMap::new(),
            var_next_id: 0,
            var_lifetimes: Vec::new(),
            vars_fixed: Vec::new(),
            branch_tails: Vec::new(),
        }
    }

    fn insert_alias<K: Into<UnitAliasKey>>(&mut self, k: K, alias: UnitAlias) {
        self.aliases.insert(k.into(), alias);
    }

    fn next_var(&mut self, line: usize) -> UnitVar {
        let var = UnitVar(self.var_next_id);
        self.var_lifetimes.push((line, line));
        self.var_next_id += 1;
        var
    }

    fn get_var<K: Into<UnitAliasKey>>(&mut self, k: K, fix: bool, line: usize) -> UnitVar {
        let unit_var = if fix {
            self.next_var(line)
        } else {
            self
                .aliases
                .get(&k.into())
                .and_then(UnitAlias::try_as_var)
                .cloned()
                .unwrap_or_else(|| self.next_var(line))
        };

        self.update_lifetime_unitvar(&unit_var, line);

        unit_var
    }

    fn lookup_alias<K: Into<UnitAliasKey>>(&self, k: K) -> MypsLexerResult<&UnitAlias> {
        let k = k.into();
        self.aliases.get(&k).ok_or(MypsLexerError::undefined_alias(k))
    }

    fn lookup_num<K: Into<UnitAliasKey>>(&self, k: K) -> MypsLexerResult<&UnitNum> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Num(unit_num) => Ok(unit_num),
            // UnitAlias::Var(var) => UnitNum::Var(var),
            // _ => unreachable!("{:?}", alias),
            _ => Err(MypsLexerError::wrong_alias("UnitNum", alias)),
        }
    }

    fn lookup_dev<K: Into<UnitAliasKey>>(&self, k: K) -> MypsLexerResult<&UnitDev> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Dev(unit_dev) => Ok(unit_dev),
            // UnitAlias::Num(i) => UnitDev::Lit(i as usize),
            // UnitAlias::Var(var) => UnitDev::Var(var),
            // _ => unreachable!("{:?}", alias),
            _ => Err(MypsLexerError::wrong_alias("UnitDev", alias)),
        }
    }

    fn lookup_dev_net<K: Into<UnitAliasKey>>(&self, k: K) -> MypsLexerResult<&UnitDevNet> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Net(unit_dev_net) => Ok(unit_dev_net),
            // UnitAlias::Num(i) => UnitDevNet::Lit(i as i64),
            // UnitAlias::Var(var) => UnitDevNet::Var(var),
            // _ => unreachable!("{:?}", alias),
            _ => Err(MypsLexerError::wrong_alias("UnitDevNet", alias)),
        }
    }

    fn lookup_var<K: Into<UnitAliasKey>>(&self, k: K) -> MypsLexerResult<&UnitVar> {
        let alias = self.lookup_alias(k)?;
        match alias {
            UnitAlias::Var(unit_var) => {
                // println!("GET VAR {} {}", unit_var, line);
                Ok(unit_var)
            },
            _ => Err(MypsLexerError::wrong_alias("UnitVar", alias)),
        }
    }

    /// Parse, lex and translate a MYPS source string.
    pub fn parse_lex_and_translate(source: &str) -> MypsLexerResult<Self> {
        let peg = MypsParser::parse(Rule::program, source)?;
        let (program_item, lexer) = Lexer::lex(peg)?;
        let mut translator = Self::new();
        translator.translate_item(program_item, 0);
        Ok(translator)
    }

    fn push_unit(&mut self, unit_expr: UnitExpr, comment: Option<String>) {
        self.units.push(Unit::new(unit_expr, comment));
    }

    /// Translate a top-level item, treating it as the program block.
    pub fn translate(program_item: Item) -> Self {
        let mut translator = Self::new();
        translator.translate_item(program_item, 0);
        // Replace the temporary UnitLine::Indexed members of If/Elif/Else units with their
        // corresponding tail lines
        let tails = translator
            .branch_tails
            .iter()
            .map(|i| UnitLine::Lit(*i as i64))
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        for unit in translator.units.iter_mut() {
            if let Some(line) = unit.unit_expr.last_mut().and_then(UnitArg::as_unit_line_mut) {
                if let UnitLine::Indexed(i) = line {
                    *line = tails[*i];
                }
            }
        }
        translator
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

    fn update_lifetime_s_unitvar(&mut self, var: &UnitVar, line: usize) {
        println!("UPDATE S LIFETIME {} {}", var, line);
        self.var_lifetimes[var.0].0 = line;
    }

    fn update_lifetime_unitvar(&mut self, var: &UnitVar, line: usize) {
        println!("UPDATE E LIFETIME {} {}", var, line);
        self.var_lifetimes[var.0].1 = line;
    }

    fn update_lifetime_s_num(&mut self, num: &UnitNum, line: usize) {
        if let UnitNum::Var(var) = &num {
            self.update_lifetime_s_unitvar(var, line);
        }
    }

    fn update_lifetime_num(&mut self, num: &UnitNum, line: usize) {
        if let UnitNum::Var(var) = &num {
            self.update_lifetime_unitvar(var, line);
        }
    }

    fn update_lifetime<V: TryInto<UnitVar>>(&mut self, v: V, line: usize) {
        if let Ok(unit_var) = v.try_into() {
            self.var_lifetimes[unit_var.0].1 = line;
        }
    }

    fn translate_int(&self, int: Int) -> UnitNum {
        match int {
            Int::Lit(n) => UnitNum::Lit(n as f64),
            Int::Var(k) => {
                self.lookup_num(k).unwrap().clone()
            }
        }
    }

    fn translate_dev(&mut self, dev: Dev, line: usize) -> (UnitDev, usize) {
        match dev {
            Dev::Lit(box rvalue) => {
                let (rtn, num_depth) = self.translate_r_value(rvalue, line);
                let num = UnitNum::try_from(rtn).unwrap();
                // let unit_dev = UnitDev
                let unit_dev = match num {
                    UnitNum::Lit(n) => UnitDev::Lit(n as u64),
                    UnitNum::Var(v) => UnitDev::Var(v),
                };
                (unit_dev, num_depth)
            }
            Dev::Var(k) => (self.lookup_dev(k).unwrap().clone(), 0),
            Dev::DB => (UnitDev::DB, 0),
            _ => unreachable!("{:?}", dev),
        }
    }

    fn translate_dev_net(&mut self, dev: Dev, line: usize) -> (UnitDevNet, usize) {
        match dev {
            Dev::Net(box rvalue) => {
                let (rtns, num_depth) = self.translate_r_value(rvalue, line);
                let num = UnitNum::try_from(rtns).unwrap();
                let unit_dev_net = match num {
                    UnitNum::Lit(n) => UnitDevNet::Lit(n as i64),
                    UnitNum::Var(v) => UnitDevNet::Var(v),
                };
                (unit_dev_net, num_depth)
            }
            Dev::Var(k) => (self.lookup_dev_net(k).unwrap().clone(), 0),
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

    fn translate_r_value(
        &mut self,
        rvalue: RValue,
        line: usize,
        ) -> (UnitReturn, usize) {
        match rvalue {
            RValue::Num(num) => {
                let num = match num {
                    Num::Lit(n) => UnitNum::Lit(n),
                    Num::Var(k) => {
                        let num = self.lookup_num(k).unwrap();
                        // self.update_lifetime_mem(&mem, line);
                        num.clone()
                    }
                };
                (UnitReturn::Num(num), 0)
            }
            RValue::Dev(dev) => match dev {
                Dev::Lit(box id_r_value) => {
                    let (id_rtn, depth) = self.translate_r_value(id_r_value, line);
                    let unit_dev = match id_rtn {
                        UnitReturn::Num(UnitNum::Lit(n)) => UnitDev::Lit(n as u64),
                        UnitReturn::Num(UnitNum::Var(v)) => UnitDev::Var(v),
                        _ => unreachable!("{:?}", id_rtn),
                    };
                    (UnitReturn::Dev(unit_dev), depth)
                }
                Dev::Net(box hash_r_value) => {
                    let (hash_rtn, depth) = self.translate_r_value(hash_r_value, line);
                    let unit_dev_net = match hash_rtn {
                        UnitReturn::Num(UnitNum::Lit(n)) => UnitDevNet::Lit(n as i64),
                        UnitReturn::Num(UnitNum::Var(v)) => UnitDevNet::Var(v),
                        _ => unreachable!("{:?}", hash_rtn),
                    };
                    (UnitReturn::Net(unit_dev_net), depth)
                }
                Dev::DB => {
                    return (UnitReturn::Dev(UnitDev::DB), 0);
                }
                Dev::Var(k) => {
                    let alias = self.lookup_alias(k).unwrap();
                    let unit_return = match alias {
                        UnitAlias::Num(..) => unreachable!("{:?}", alias),
                        UnitAlias::Dev(unit_dev) => UnitReturn::Dev(*unit_dev),
                        UnitAlias::Net(unit_dev_net) => UnitReturn::Net(*unit_dev_net),
                        UnitAlias::Var(..) => unreachable!("{:?}", alias),
                    };
                    (unit_return, 0)
                }
            },
            RValue::NetParam(dev, mode, param) => {
                let (dev, dev_depth) = self.translate_dev_net(dev, line);
                let var = self.next_var(line);
                // self.update_lifetime_unitvar(&var, line);
                let unit_expr = UnitExpr::new_lb(var, dev, param, mode);
                let unit = Unit::new(unit_expr, None);
                self.units.push(unit);
                // self.update_lifetime_s_unitvar(&var, line);
                (UnitReturn::Num(var.into()), 1 + dev_depth)
            }
            RValue::DevParam(dev, param) => {
                let (dev, dev_depth) = self.translate_dev(dev, line);
                let var = self.next_var(line);
                // self.update_lifetime_unitvar(&var, line);
                let unit_expr = UnitExpr::new_l(var, dev, param);
                let unit = Unit::new(unit_expr, None);
                self.units.push(unit);
                // self.update_lifetime_s_unitvar(&var, line);
                (UnitReturn::Num(var.into()), 1 + dev_depth)
            }
            RValue::DevSlot(dev, slot, param) => {
                let (dev, dev_depth) = self.translate_dev(dev, line);
                let slot = self.translate_int(slot);
                let var = self.next_var(line);
                let unit_expr = UnitExpr::new_ls(var, dev, slot, param);
                let unit = Unit::new(unit_expr, None);
                self.units.push(unit);
                (UnitReturn::Num(var.into()), 1 + dev_depth)
            }
            RValue::Expr(box expr) => {
                let (unit_num, depth) = self.translate_expr(expr, line);
                (UnitReturn::Num(unit_num), depth)
            } // _ => unreachable!("{:?}", rvalue),
            RValue::Func(func, args) => {
                unreachable!("{:?}", func)
            }
            RValue::Var(k) => {
                let unit_var = *self.lookup_var(k).unwrap();
                println!("RVALUE::VAR {}", unit_var);
                self.update_lifetime_unitvar(&unit_var, line);
                (UnitReturn::Var(unit_var), 0)
                // (self.reduce_alias(k).unwrap(), 0)
            }
        }
    }

    fn reduce_alias<K: Into<UnitAliasKey>>(&self, k: K) -> MypsLexerResult<UnitReturn> {
        let alias = self.lookup_alias(k.into())?;
        match alias {
            UnitAlias::Num(unit_num)     => Ok(UnitReturn::Num(*unit_num)),
            UnitAlias::Dev(unit_dev)     => Ok(UnitReturn::Dev(*unit_dev)),
            UnitAlias::Net(unit_dev_net) => Ok(UnitReturn::Net(*unit_dev_net)),
            UnitAlias::Var(unit_var)     => self.reduce_alias(unit_var),
        }
    }

    // ============================================================================================
    // Translate an expression
    // ============================================================================================
    fn translate_expr(
        &mut self,
        expr: Expr,
        line: usize,
        ) -> (UnitNum, usize) {
        match expr {
            Expr::Unary { op, box rhs } => {
                unreachable!();
            },
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
                let (lhs, d_l) = self.translate_expr(lhs, line);
                let (rhs, d_r) = self.translate_expr(rhs, line + d_l);
                depth += d_l + d_r;

                fn bool_to_num(cond: bool) -> f64 {
                    if cond {
                        1.0
                    } else {
                        0.0
                    }
                }

                match (lhs, rhs) {
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
                        (unit_num, depth)
                    },
                    _ => {
                        // let line = depth - 1;
                        // Update lifetimes of lhs and rhs vars
                        // let line = depth + line - 1;
                        // let line = line + depth - 1;
                        // We need a var to store the result of the operation:
                        let line = line + depth;
                        let var = self.next_var(line);
                        // self.update_lifetime_s_unitvar(&var, line);
                        // self.update_lifetime_unitvar(&var, line);
                        self.update_lifetime(&lhs, line);
                        self.update_lifetime(&rhs, line);
                        // Append this expression
                        let unit_expr = match op {
                            BinaryOp::Add => UnitExpr::new_add(var, lhs, rhs),
                            BinaryOp::Sub => UnitExpr::new_sub(var, lhs, rhs),
                            BinaryOp::Mul => UnitExpr::new_mul(var, lhs, rhs),
                            BinaryOp::Div => UnitExpr::new_div(var, lhs, rhs),
                            BinaryOp::Rem => UnitExpr::new_mod(var, lhs, rhs),

                            BinaryOp::EQ => UnitExpr::new_seq(var, lhs, rhs),
                            BinaryOp::GT => UnitExpr::new_sgt(var, lhs, rhs),
                            BinaryOp::LT => UnitExpr::new_slt(var, lhs, rhs),
                            BinaryOp::NE => UnitExpr::new_sne(var, lhs, rhs),
                            _ => unreachable!("{:?}", op),
                        };
                        self.units.push(Unit::new(unit_expr, None));
                        (var.into(), depth)
                    }
                }
            }
            Expr::Ternary { cond, if_t, if_f } => {
                unreachable!();
            },
            Expr::RValue(rv) => {
                let (rv_rtn, depth) = self.translate_r_value(rv, line);
                // let line = line - depth;
                // self.update_lifetime_s_mem(&mem, line);
                // self.update_lifetime_mem(&mem, line);
                match rv_rtn {
                    UnitReturn::Num(num) => (num, depth),
                    UnitReturn::Var(unit_var) => {
                        self.update_lifetime_unitvar(&unit_var, line);
                        (UnitNum::Var(unit_var), depth)
                    },
                    _ => unreachable!(),
                }
                // if let UnitReturn::Num(num) = rv_rtn {
                //     (num, depth)
                // } else {
                //     unreachable!();
                // }
            }
        }
    }

    // ============================================================================================
    // Translate items
    // ============================================================================================
    fn translate_items(&mut self, items: Vec<Item>, line: usize) -> usize {
        let total_depth = items.into_iter().fold(0, |depth, item| {
            depth + self.translate_item(item, line + depth)
        });
        total_depth
    }

    // ============================================================================================
    // Translate an item
    // ============================================================================================

    fn translate_condition(
        &mut self,
        cond: Expr,
        comment: Option<String>,
        line: usize,
        ) -> (usize, UnitNum, usize) {
        let (rtns, mut depth) = self.translate_expr(cond, line);
        let num = UnitNum::try_from(rtns).unwrap();
        if depth == 0
            || !self
                .units
                .last()
                .map(Unit::is_logical_or_select)
                .unwrap_or(false)
                {
                    self.units.push(Unit::new(UnitExpr::Dummy, comment));
                    depth += 1;
                } else {
                    // Else, by replacing the select expresison by a branch
                    // expression, the var that the select expression assigns to
                    // is not needed
                    self.var_next_id -= 1;
                    self.var_lifetimes.pop();
                }
        let i = self.units.len() - 1;
        (i, num, depth)
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
                UnitExpr::Sne([r, a, b]) => UnitExpr::new_breq(a.into(), b.into(), c),
                // set if greater than -> branch if less-than or equal
                UnitExpr::Sgt([r, a, b]) => UnitExpr::new_brle(a.into(), b.into(), c),
                // set if less than -> branch if greater-than or equal
                UnitExpr::Slt([r, a, b]) => UnitExpr::new_brge(a.into(), b.into(), c),
                // non-relational expr -> branch if equal to zero
                // UnitExpr::Seq(
                UnitExpr::Dummy => UnitExpr::new_breqz(num, c),
                _ => unreachable!("{:?}", unit_expr),
            }
        };
        self.units[i].unit_expr = cond_expr;
    }

    fn translate_item(&mut self, item: Item, line: usize) -> usize {
        let Item {
            item_inner,
            comment,
            ..
        } = item;

        match item_inner {
            ItemInner::Block(Block { branch, items }) => {
                match branch {
                    Branch::Program => self.translate_items(items, 0),
                    // ============================================================================
                    // Loop (infinitely)
                    // ============================================================================
                    Branch::Loop => {
                        let depth = self.translate_items(items, line);
                        let line = UnitLine::Lit(-(depth as i64));
                        let unit_expr = UnitExpr::new_jr(line);
                        let unit = Unit::new(unit_expr, comment);
                        self.units.push(unit);
                        depth + 1
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
                                let unit = if matches!(branch, Branch::Else(..)) {
                                    // TODO: Not sure how to do without clone;
                                    // its always moved, leaving the comment for if/elif invalid
                                    Unit::new(unit_expr, comment.clone())
                                } else {
                                    Unit::new(unit_expr, None)
                                };
                                self.units.push(unit);
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
                                    self.translate_condition(cond, comment, line);
                                depth += cond_depth;
                                Some((i, num))
                            }
                            _ => None,
                        };
                        // Translate branch body
                        let body_depth = self.translate_items(items, line + depth);
                        depth += body_depth;
                        // (If/Elif)
                        if let Some((i, num)) = cond_opt {
                            self.transform_condition(i, num, body_depth);
                        }
                        // Update the branch tail for this index
                        let tail = line + depth;
                        if let Some(line) = self.branch_tails.get_mut(id) {
                            *line = tail;
                        } else {
                            self.branch_tails.push(tail);
                        }
                        depth
                    }
                    // ============================================================================
                    // While loop
                    // ============================================================================
                    // TODO: Same as if, with tail branch
                    Branch::While(cond) => {
                        let mut depth = 0;

                        let (i, num, cond_depth) =
                            self.translate_condition(cond, comment, line);
                        depth += cond_depth;

                        // Translate branch body
                        let body_depth = self.translate_items(items, line + depth);
                        depth += body_depth;

                        self.transform_condition(i, num, body_depth);

                        depth
                    }

                    // ============================================================================
                    // For loop
                    // ============================================================================
                    Branch::For(i, s, e, step_opt) => {
                        let mut depth = 0;
                        // (Start value expression)
                        // let i_var = {
                        //     // TODO: Probably shouldn't need to type constrict here
                        //     if let Some(UnitAlias::Var(i_var)) = self.aliases.get(&i) {
                        //         *i_var
                        //     } else {
                        //         depth += 1;
                        //         self.next_var(line)
                        //     }
                        // };
                        // Fix i_var so that it doesn't get overwritten by register optimization
                        let (s_rtns, s_depth) = self.translate_expr(s, line);
                        let s_num = UnitNum::try_from(s_rtns).unwrap();
                        let i_var = UnitVar::try_from(s_num).unwrap();
                        self.vars_fixed.push(i_var.0);

                        if depth == 1 {
                            let comment = Some(format!("# {} = ({} = {})", i, i_var, s_num));
                            let unit_expr = UnitExpr::new_move(i_var, s_num);
                            let unit = Unit::new(unit_expr, comment);
                            self.units.push(unit);
                            depth += 1;
                        }
                        depth += s_depth;
                        // let i_unit = UnitVar::Var(i_var);
                        // Add new "i" var to lookup
                        self.insert_alias(i, UnitAlias::Var(i_var));
                        let i_num = UnitNum::Var(i_var);
                        // (Body)
                        let mut inner_depth = 1 + self.translate_items(items, line + depth);
                        // (End value expression)
                        let (e_rtns, e_depth) = self.translate_expr(e, line);
                        let e_num = UnitNum::try_from(e_rtns).unwrap();
                        inner_depth += e_depth;
                        // (Step value expression)
                        let step = if let Some(step_expr) = step_opt {
                            let (step_rtns, step_depth) =
                                self.translate_expr(step_expr, line);
                            inner_depth += step_depth;
                            UnitNum::try_from(step_rtns).unwrap()
                        } else {
                            UnitNum::Lit(1.0)
                        };
                        // (Increment and branch statements)
                        let unit_expr = UnitExpr::new_add(i_var, i_num, step);
                        let unit = Unit::new(unit_expr, None);
                        self.units.push(unit);
                        let line = UnitLine::Lit(-(inner_depth as i64));
                        let unit_expr = UnitExpr::new_brlt(i_num, e_num, line);
                        let unit = Unit::new(unit_expr, comment);
                        self.units.push(unit);
                        depth + inner_depth
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
                    _ => unreachable!("{:?}", branch),
                }
                // block.items()
            }
            ItemInner::Stmt(stmt) => self.translate_statement(stmt, comment, line),
            // _ => unreachable!("{:?}", stmt),
        }
    }

    fn translate_statement(
        &mut self,
        stmt: Statement,
        mut comment: Option<String>,
        line: usize,
        ) -> usize {
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

                let rv_returns = r_values
                    .into_iter()
                    .map(|r_value| {
                        let (rv_rtn, rv_depth) = self.translate_r_value(r_value, depth);
                        depth += rv_depth;
                        rv_rtn
                    })
                .collect::<Vec<UnitReturn>>();

                for (lv, rv_rtn) in l_values.into_iter().zip(rv_returns) {
                    depth += self.translate_assignment(lv, rv_rtn, comment.take(), line + depth);
                }

                depth
            }
            // ============================================================================
            // A lone function call
            // ============================================================================
            Statement::FunctionCall(name, args) => {
                match name.as_str() {
                    "yield" => {
                        self.push_unit(UnitExpr::new_yield(), comment);
                        1
                    }
                    "hcf" => {
                        self.push_unit(UnitExpr::new_hcf(), comment);
                        1
                    }
                    // "sleep" => {
                    // self.push_unit(UnitExpr::new_sleep(), comment);
                    // 1
                    // },
                    _ => {
                        unreachable!("{:?}", name)
                    }
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
            }
        }
    }

    fn translate_assignment(
        &mut self,
        l_value: LValue,
        r_value_rtn: UnitReturn,
        comment: Option<String>,
        line: usize,
        ) -> usize {
        let mut depth = 0;

        match r_value_rtn {
            UnitReturn::Dev(unit_dev) => {
                let (k, fix) = match l_value {
                    LValue::Var(k, fix) => (k, fix),
                    LValue::Param(..) => unreachable!(),
                };

                if fix {
                    let unit_expr = UnitExpr::new_alias(k.clone(), unit_dev);
                    let unit = Unit::new(unit_expr, None);
                    self.units.push(unit);
                    depth += 1;
                }
                let alias = UnitAlias::Dev(unit_dev);
                self.insert_alias(k, alias);
            }
            UnitReturn::Net(unit_dev_net) => {
                let (k, fix) = match l_value {
                    LValue::Var(k, fix) => (k.clone(), fix),
                    LValue::Param(..) => unreachable!(),
                };

                let var = self.get_var(&k, fix, line + depth);
                let hash = unit_dev_net.into();
                if fix {
                    let unit_expr = UnitExpr::new_move(var, hash);
                    depth += 1;
                }
                let alias = UnitAlias::Num(hash);
                self.insert_alias(k, alias);
            }
            UnitReturn::Num(unit_num) => {
                match l_value {
                    LValue::Var(k, fix) => {
                        if fix {
                            // If marked fix
                            // - insert an alias from the var to the number, and
                            // - insert an alias from the name to the var
                            let unit_var = self.get_var(k.clone(), fix, line + depth);

                            let unit_expr = UnitExpr::new_move(unit_var, unit_num);
                            let unit = Unit::new(unit_expr, comment);
                            self.units.push(unit);
                            self.insert_alias(unit_var, UnitAlias::Num(unit_num));
                            self.insert_alias(k, UnitAlias::Var(unit_var));
                            depth += 1;
                        } else {
                            // Else if not marked fix
                            // - insert an alias from the name to the number
                            self.insert_alias(k, UnitAlias::Num(unit_num));
                        }
                        // self.aliases.get(&k);
                    },
                    LValue::Param(dev, param) => {
                        let unit_expr = match dev {
                            Dev::Lit(box id_rv) => {
                                let (rtns, rv_depth) = self.translate_r_value(id_rv, line + depth);
                                let unit_dev = UnitDev::try_from(rtns).unwrap();
                                depth += rv_depth;
                                UnitExpr::new_s(unit_dev, param, unit_num)
                            },
                            Dev::Net(box hash_rv) => {
                                let (rtns, rv_depth) = self.translate_r_value(hash_rv, line + depth);
                                let unit_dev_net = UnitDevNet::try_from(rtns).unwrap();
                                depth += rv_depth;
                                UnitExpr::new_sb(unit_dev_net, param, unit_num)
                            },
                            Dev::Var(k) => {
                                let alias = self.aliases.get(&k.into());
                                match alias {
                                    Some(UnitAlias::Dev(unit_dev)) => {
                                        UnitExpr::new_s(*unit_dev, param, unit_num)
                                    },
                                    Some(UnitAlias::Net(unit_dev_net)) => {
                                        UnitExpr::new_sb(*unit_dev_net, param, unit_num)
                                    },
                                    _ => unreachable!("{:?}", alias),
                                }
                            },
                            Dev::DB => {
                                UnitExpr::new_s(UnitDev::DB, param, unit_num)
                            },
                        };
                        let unit = Unit::new(unit_expr, comment);
                        self.units.push(unit);
                        depth += 1;
                    }
                }
            },
            UnitReturn::Var(unit_var) => {
                unreachable!();
            }
        }
        depth
    }
}
