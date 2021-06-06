#![allow(unused_imports)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::{
    fmt,
    fmt::{Debug, Display},
};

use crate::ast::{BinaryOp, Branch, Dev, Expr, Int, LValue, Mode, Num, RValue, UnaryOp};
use crate::lexer::{lex::parse_and_lex, AliasTable, Item, MypsLexerResult, Statement};
// use crate::analyzer::analyze;

#[derive(Copy, Clone, Debug)]
pub struct UnitVar(usize);

impl Display for UnitVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R{}", self.0)
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
    Var(UnitVar),
}

#[derive(Copy, Clone, Debug)]
pub enum UnitDevNet {
    Lit(i64),
    Var(UnitVar),
}

#[derive(Copy, Clone, Debug)]
pub enum Var {
    Mem(UnitMem),
    Dev(UnitDev),
    DevNet(UnitDevNet),
}

impl From<Mem> for Var {
    fn from(mem: Mem) -> Self {
        match mem {
            Mem::Unit(u) => Self::Mem(u),
            _ => unreachable!("{:?}", mem),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Unit {
    L(UnitMem, UnitDev, String),
    LB(UnitMem, UnitDevNet, String, Mode),

    JR(Mem),

    // add r? a(r?|num) b(r?|num)
    Add(UnitMem, Mem, Mem),

    // move r? a(r?|num)
    Move(UnitMem, UnitMem),
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Add(r, a, b) => write!(f, "add {} {} {}", r, a, b),
            _ => unreachable!("{:?}", self),
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
        let (program, _alias_table) = parse_and_lex(source)?;
        let mut translator = Self::new();
        translator.translate_stmt(program.stmt);
        Ok(translator)
    }

    fn new() -> Self {
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

    fn lookup_mem(&self, k: &String) -> UnitMem {
        let var = self.var_lookup[k];
        match var {
            Var::Mem(mu) => mu,
            _ => unreachable!("{:?}", var),
        }
    }

    fn translate_rvalue(&mut self, rvalue: RValue, depth: usize) -> (Mem, usize) {
        match rvalue {
            RValue::Num(num) => {
                let mem = match num {
                    Num::Lit(n) => Mem::Num(n),
                    Num::Var(k) => Mem::Unit(self.lookup_mem(&k)),
                };
                (mem, depth)
            }
            // RValue::NetParam(hash, mode, param) => {
            //     // lb r? type var batchMode
            //     let var = self.next_var();
            //     self.units.push(Unit::LB(UnitMem::Var(var), 0, param, mode));
            //     Mem::Unit(UnitMem::Var(var))
            // },
            // RValue::DevParam(dev, param) => {
            //     let unit_dev = dev.into();
            //     // l r? d? var
            //     let var = self.next_var();
            //     self.units.push(Unit::L(var, unit_dev, param));
            // },
            RValue::Expr(box expr) => self.translate_expr(expr, depth),
            _ => unreachable!("{:?}", rvalue),
        }
    }

    fn translate_expr(&mut self, expr: Expr, depth: usize) -> (Mem, usize) {
        match expr {
            // Expr::Unary { op, box rhs } => {
            // },
            Expr::Binary {
                op,
                box lhs,
                box rhs,
            } => {
                let (lhs, d_l) = self.translate_expr(lhs, depth);
                let (rhs, d_r) = self.translate_expr(rhs, depth);
                let depth = d_l + d_r;

                let mem = match op {
                    BinaryOp::Add => {
                        let var = UnitMem::Var(self.next_var());
                        self.units.push(Unit::Add(var, lhs, rhs));
                        Mem::Unit(var)
                    }
                    _ => unreachable!("{:?}", op),
                };
                (mem, depth + 1)
            }
            // Expr::Ternary { cond, if_t, if_f } => {
            // },
            // Expr::RValue(rv) => {
            //     self.translate_rvalue(rv);
            // },
            Expr::RValue(rv) => self.translate_rvalue(rv, depth),
            _ => unreachable!("{:?}", expr),
        }
    }

    fn translate_stmt(&mut self, stmt: Statement) -> usize {
        match stmt {
            Statement::Block(block) => {
                match block.branch {
                    Branch::Loop => {
                        let depth = block
                            .items
                            .into_iter()
                            .fold(0, |depth, item| depth + self.translate_stmt(item.stmt));
                        self.units.push(Unit::JR(Mem::Num(-(depth as f64))));
                        depth + 1
                    }
                    _ => unreachable!("{:?}", block.branch),
                }
                // block.items()
            }
            // Statement::AssignAlias
            Statement::AssignValue(l, r) => {
                let (mem, depth) = self.translate_rvalue(r, 0);
                match l {
                    LValue::Var(k) => self.var_lookup.insert(k, mem.into()),
                    _ => unreachable!("{:?}", l),
                };
                depth
            }
            _ => unreachable!("{:?}", stmt),
        }
    }
}
