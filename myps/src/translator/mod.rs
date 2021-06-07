#![allow(unused_imports)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::{
    fmt,
    fmt::{Debug, Display},
};

use crate::superprelude::*;

#[derive(Copy, Clone, Debug)]
pub struct UnitVar(usize);

impl Display for UnitVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct UnitAlias(String);

impl Display for UnitAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnitMem {
    Lit(usize, usize),
    Var(UnitVar),
}

impl Display for UnitMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitMem::Lit(b, i) => {
                write!(f, "r")?;
                for _ in 0..*i {
                    write!(f, "r")?;
                }
                write!(f, "{}", b)
            }
            UnitMem::Var(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mem {
    Num(f64),
    Unit(UnitMem),
}

impl From<UnitVar> for Mem {
    fn from(unit_var: UnitVar) -> Self {
        Self::Unit(UnitMem::Var(unit_var))
    }
}

impl From<Var> for Mem {
    fn from(var: Var) -> Self {
        match var {
            Var::Num(n) => Self::Num(n),
            Var::Mem(u) => Self::Unit(u),
            Var::Var(v) => Self::Unit(UnitMem::Var(v)),
            _ => unreachable!("{:?}", var),
        }
    }
}

impl Display for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mem::Num(n) => write!(f, "{}", n),
            Mem::Unit(u) => write!(f, "{}", u),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnitDev {
    Lit(usize, usize),
    Indexed(Mem),
    Var(UnitVar),
}

impl From<Var> for UnitDev {
    fn from(var: Var) -> Self {
        match var {
            Var::Dev(UnitDev::Lit(b, i)) => Self::Lit(b, i),
            _ => unreachable!("{:?}", var),
        }
    }
}

impl Display for UnitDev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(b, i) => {
                write!(f, "d")?;
                for _ in 0..*i {
                    write!(f, "r")?;
                }
                write!(f, "{}", b)
            }
            Self::Indexed(id) => {
                unreachable!()
                // TODO: Match id; if int, then d?; if mem then dr?
                // match 
                // write!(
            },
            Self::Var(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnitDevNet {
    Lit(i64),
    Var(UnitVar),
}

impl From<Var> for UnitDevNet {
    fn from(var: Var) -> Self {
        match var {
            Var::Num(n) => Self::Lit(n as i64),
            Var::Var(v) => Self::Var(v),
            _ => unreachable!("{:?}", var),
        }
    }
}

impl Display for UnitDevNet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lit(hash) => write!(f, "{}", hash),
            Self::Var(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Var {
    Num(f64),
    Var(UnitVar),
    Mem(UnitMem),
    Dev(UnitDev),
    DevNet(UnitDevNet),
}

impl From<Mem> for Var {
    fn from(mem: Mem) -> Self {
        match mem {
            Mem::Unit(u) => Self::Mem(u),
            Mem::Num(n) => Self::Num(n),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Unit {
    unit_expr: UnitExpr,
    comment: Option<String>,
}

impl Unit {
    pub fn new(unit_expr: UnitExpr, comment: Option<String>) -> Self {
        Self { unit_expr, comment }
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

#[derive(Clone, Debug)]
pub enum UnitExpr {
    L(UnitMem, UnitDev, String),
    LB(UnitMem, UnitDevNet, String, Mode),

    JR(Mem),

    Add(UnitMem, Mem, Mem),
    Sub(UnitMem, Mem, Mem),
    Mul(UnitMem, Mem, Mem),
    Div(UnitMem, Mem, Mem),

    // move r? a(r?|num)
    Alias(UnitAlias, Dev),
    Move(UnitMem, Mem),
}

impl Display for UnitExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::L(r, d, param) => write!(f, "l {} {} {}", r, d, param),
            Self::LB(r, hash, param, mode) => write!(f, "lb {} {} {} {}", r, hash, param, mode),
            Self::JR(a) => write!(f, "jr {}", a),

            Self::Add(r, a, b) => write!(f, "add {} {} {}", r, a, b),
            Self::Sub(r, a, b) => write!(f, "sub {} {} {}", r, a, b),
            Self::Mul(r, a, b) => write!(f, "mul {} {} {}", r, a, b),
            Self::Div(r, a, b) => write!(f, "div {} {} {}", r, a, b),

            Self::Alias(s, d) => write!(f, "alias {} {}", s, d),
            Self::Move(r, a) => write!(f, "move {} {}", r, a),
            // _ => unreachable!("{:?}", self),
        }
    }
}

#[derive(Debug)]
pub struct Translator {
    pub units: Vec<Unit>,
    pub var_next_id: usize,
    pub var_lookup: HashMap<String, Var>,
}

impl Translator {
    pub fn parse_and_translate(source: &str) -> MypsLexerResult<Self> {
        let peg = MypsParser::parse(Rule::program, source)?;
        let (program, _alias_table) = lex(peg)?;
        let mut translator = Self::new();
        // println!("{:#?}", program);
        translator.translate_item(program);
        Ok(translator)
    }

    pub fn new() -> Self {
        let units = Vec::new();
        let var_next_id = 0;
        let var_lookup = HashMap::new();
        Self {
            units,
            var_next_id,
            var_lookup,
        }
    }

    fn next_var(&mut self) -> UnitVar {
        let var = UnitVar(self.var_next_id);
        self.var_next_id += 1;
        var
    }

    fn lookup_mem(&self, k: &String) -> Var {
        let var = self.var_lookup[k];
        match var {
            Var::Mem(..) => var,
            Var::Num(..) => var,
            Var::Var(..) => var,
            _ => unreachable!("{:?}", var),
        }
    }

    fn lookup_dev(&self, k: &String) -> Var {
        let var = self.var_lookup[k];
        match var {
            Var::Dev(..) => var,
            Var::Var(..) => var,
            _ => unreachable!("{:?}", var),
        }
    }

    fn translate_rvalue(&mut self, rvalue: RValue, comment: Option<String>, depth: usize) -> (Mem, Option<String>, usize) {
        match rvalue {
            RValue::Num(num) => {
                let mem = match num {
                    Num::Lit(n) => Mem::Num(n),
                    Num::Var(k) => self.lookup_mem(&k).into(),
                };
                (mem, comment, depth)
            }
            RValue::NetParam(hash, mode, param) => {
                let dev_net = match hash {
                    Int::Lit(n) => UnitDevNet::Lit(n),
                    Int::Var(k) => self.lookup_mem(&k).into(),
                };
                // lb r? type var batchMode
                let var = self.next_var();
                let unit_expr = UnitExpr::LB(UnitMem::Var(var), dev_net, param, mode);
                let unit = Unit::new(unit_expr, comment);
                self.units.push(unit);
                (var.into(), None, depth + 1)
            },
            RValue::DevParam(dev, param) => {
                let dev = match dev {
                    Dev::Lit(b, i) => UnitDev::Lit(b, i),
                    Dev::Indexed(id) => {
                        let id = match id {
                            Int::Lit(n) => Mem::Num(n as f64),
                            Int::Var(var) => {
                                unreachable!()
                                // TODO
                                // Mem::Unit(UnitMem::Var(UnitVar(var.clone()))),
                            },
                        };
                        UnitDev::Indexed(id)
                    },
                    Dev::Var(k) => self.lookup_dev(&k).into(),
                    Dev::Batch(hash) => unreachable!(),
                };
                let var = self.next_var();
                let unit_expr = UnitExpr::L(UnitMem::Var(var), dev, param);
                let unit = Unit::new(unit_expr, comment);
                self.units.push(unit);
                (var.into(), None, depth + 1)
            },
            RValue::Expr(box expr) => {
                let (mem, depth) = self.translate_expr(expr, comment, depth);
                (mem, None, depth)
            },
            // _ => unreachable!("{:?}", rvalue),
        }
    }

    fn translate_expr(&mut self, expr: Expr, comment: Option<String>, depth: usize) -> (Mem, usize) {
        match expr {
            // Expr::Unary { op, box rhs } => {
            // },
            Expr::Binary {
                op,
                box lhs,
                box rhs,
            } => {
                let (lhs, d_l) = self.translate_expr(lhs, None, depth);
                let (rhs, d_r) = self.translate_expr(rhs, None, depth);
                let depth = d_l + d_r;

                let var = UnitMem::Var(self.next_var());
                let unit_expr = match op {
                    BinaryOp::Add => UnitExpr::Add(var, lhs, rhs),
                    BinaryOp::Sub => UnitExpr::Sub(var, lhs, rhs),
                    BinaryOp::Mul => UnitExpr::Mul(var, lhs, rhs),
                    BinaryOp::Div => UnitExpr::Div(var, lhs, rhs),
                    _ => unreachable!("{:?}", op),
                };
                self.units.push(Unit::new(unit_expr, comment));
                (Mem::Unit(var), depth + 1)
            }
            // Expr::Ternary { cond, if_t, if_f } => {
            // },
            // Expr::RValue(rv) => {
            //     self.translate_rvalue(rv);
            // },
            Expr::RValue(rv) => {
                let (mem, _, depth) = self.translate_rvalue(rv, comment, depth);
                (mem, depth)
            },
            _ => unreachable!("{:?}", expr),
        }
    }

    fn translate_items(&mut self, items: Vec<Item>) -> usize {
        items
            .into_iter()
            .fold(0, |depth, item| {
                // println!("{:#?}", item.stmt);
                depth + self.translate_item(item)
            })
    }

    pub fn translate_item(&mut self, item: Item) -> usize {
        let Item { item_inner, comment, .. } = item;
        // println!("{:?}", comment);
        match item_inner {
            ItemInner::Block(block) => {
                match block.branch {
                    Branch::Program => {
                        self.translate_items(block.items)
                    }
                    Branch::Loop => {
                        let depth = self.translate_items(block.items);
                        let unit_expr = UnitExpr::JR(Mem::Num(-(depth as f64)));
                        let unit = Unit::new(unit_expr, comment);
                        self.units.push(unit);
                        depth + 1
                    }
                    _ => unreachable!("{:?}", block.branch),
                }
                // block.items()
            }
            ItemInner::Stmt(stmt) => {
                match stmt {
                    Statement::AssignAlias(a, d) => {
                        match d {
                            Dev::Lit(b, i) => {
                                let unit_alias = UnitAlias(a.clone());
                                let dev = Dev::Lit(b, i);
                                let unit_expr = UnitExpr::Alias(unit_alias, dev);
                                let unit = Unit::new(unit_expr, comment);
                                self.units.push(unit);
                                self.var_lookup.insert(a, Var::Dev(UnitDev::Lit(b, i)));
                            },
                            Dev::Indexed(_) => { unreachable!(); },
                            Dev::Batch(hash) => { unreachable!(); },
                            Dev::Var(v) => { unreachable!(); },
                        }
                        1
                    },
                    Statement::AssignValue(l, r) => {
                        // println!("ASSIGN VALUE {:?}={:?}", l, r);
                        let (mem, comment, depth) = self.translate_rvalue(r, comment, 0);
                        match l {
                            // TODO: Optimization levels
                            LValue::Var(k) => {
                                // println!("DEPTH = {}", depth);
                                // if depth == 0 {
                                    self.var_lookup.insert(k, mem.into());
                                    depth
                                // } else {
                                //     let var = self.next_var();
                                //     let unit_expr = UnitExpr::Move(UnitMem::Var(var), mem);
                                //     let unit = Unit::new(unit_expr, comment);
                                //     self.units.push(unit);
                                //     self.var_lookup.insert(k, Var::Var(var));
                                //     depth + 1
                                // }
                            },
                            _ => unreachable!("{:?}", l),
                        }
                    }
                    Statement::FunctionCall(_) => {
                        // TODO
                        unreachable!()
                    }
                }
            }
            // _ => unreachable!("{:?}", stmt),
        }
    }
}
