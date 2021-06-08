#![allow(unused_imports)]
// #![allow(unused_variables)]
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
        write!(f, "r{}", self.0)
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
    RA,
    SP,
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
            Mem::RA => write!(f, "ra"),
            Mem::SP => write!(f, "sp"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnitDev {
    Lit(usize, usize),
    Indexed(Mem),
    Var(UnitVar),
    DB,
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
            Self::Indexed(_id) => {
                unreachable!()
                // TODO: Match id; if int, then d?; if mem then dr?
                // match
                // write!(
            }
            Self::Var(v) => write!(f, "{}", v),
            Self::DB => write!(f, "db"),
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
    Function(f64),
}

impl From<Mem> for Var {
    fn from(mem: Mem) -> Self {
        match mem {
            Mem::Unit(u) => Self::Mem(u),
            Mem::Num(n) => Self::Num(n),
            _ => unreachable!("{:?}", mem),
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
    SB(UnitDevNet, String, Mem),

    BRLT(Mem, Mem, Mem),
    J(Mem),
    JR(Mem),
    JAL(Mem),

    BRNE(Mem, Mem, Mem),

    SNE(UnitVar, Mem, Mem),

    Add(UnitVar, Mem, Mem),
    Sub(UnitVar, Mem, Mem),
    Mul(UnitVar, Mem, Mem),
    Div(UnitVar, Mem, Mem),

    // move r? a(r?|num)
    Alias(UnitAlias, Dev),
    Move(UnitMem, Mem),

    Dummy,
}

// impl UnitExpr {
//     pub fn lead_unit_mem_mut(&mut self) -> Option<&mut UnitMem> {
//         match self {
//             Self::Add(um, ..) => Some(um),
//             _ => unreachable!("{:?}", self),
//         }
//     }
// }

impl Display for UnitExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::L(r, d, param) => write!(f, "l {} {} {}", r, d, param),
            Self::LB(r, hash, param, mode) => write!(f, "lb {} {} {} {}", r, hash, param, mode),
            Self::SB(hash, param, a) => write!(f, "sb {} {} {}", hash, param, a),

            Self::BRLT(a, b, c) => write!(f, "brlt {} {} {}", a, b, c),
            Self::J(a) => write!(f, "j {}", a),
            Self::JR(a) => write!(f, "jr {}", a),
            Self::JAL(a) => write!(f, "jal {}", a),

            Self::BRNE(a, b, c) => write!(f, "brne {} {} {}", a, b, c),

            Self::SNE(r, a, b) => write!(f, "sne {} {} {}", r, a, b),

            Self::Add(r, a, b) => write!(f, "add {} {} {}", r, a, b),
            Self::Sub(r, a, b) => write!(f, "sub {} {} {}", r, a, b),
            Self::Mul(r, a, b) => write!(f, "mul {} {} {}", r, a, b),
            Self::Div(r, a, b) => write!(f, "div {} {} {}", r, a, b),

            Self::Alias(s, d) => write!(f, "alias {} {}", s, d),
            Self::Move(r, a) => write!(f, "move {} {}", r, a),

            #[allow(unreachable_patterns)]
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
        let peg = MypsParser::parse(Rule::program, source)?;
        let (program, _alias_table) = lex(peg)?;
        let mut translator = Self::new();
        translator.translate_item(program, 0);
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

    fn translate_rvalue(
        &mut self,
        rvalue: RValue,
        comment: Option<String>,
        var_opt: Option<UnitVar>,
    ) -> (Mem, Option<String>, usize) {
        match rvalue {
            RValue::Num(num) => {
                let mem = match num {
                    Num::Lit(n) => Mem::Num(n),
                    Num::Var(k) => self.lookup_mem(&k).into(),
                };
                (mem, comment, 0)
            }
            RValue::NetParam(hash, mode, param) => {
                let dev_net = match hash {
                    Int::Lit(n) => UnitDevNet::Lit(n),
                    Int::Var(k) => self.lookup_mem(&k).into(),
                };
                // lb r? type var batchMode
                let var = var_opt.unwrap_or_else(|| self.next_var());
                let unit_expr = UnitExpr::LB(UnitMem::Var(var), dev_net, param, mode);
                let unit = Unit::new(unit_expr, comment);
                self.units.push(unit);
                (var.into(), None, 1)
            }
            RValue::DevParam(dev, param) => {
                let dev = match dev {
                    Dev::Lit(b, i) => UnitDev::Lit(b, i),
                    Dev::Indexed(id) => {
                        let id = match id {
                            Int::Lit(n) => Mem::Num(n as f64),
                            Int::Var(_var) => {
                                unreachable!()
                                // TODO
                                // Mem::Unit(UnitMem::Var(UnitVar(var.clone()))),
                            }
                        };
                        UnitDev::Indexed(id)
                    }
                    Dev::Var(k) => self.lookup_dev(&k).into(),
                    Dev::Batch(_hash) => unreachable!(),
                };
                let var = var_opt.unwrap_or_else(|| self.next_var());
                let unit_expr = UnitExpr::L(UnitMem::Var(var), dev, param);
                let unit = Unit::new(unit_expr, comment);
                self.units.push(unit);
                (var.into(), None, 1)
            }
            RValue::Expr(box expr) => {
                let (mem, depth) = self.translate_expr(expr, comment, var_opt);
                (mem, None, depth)
            } // _ => unreachable!("{:?}", rvalue),
        }
    }

    fn translate_expr(
        &mut self,
        expr: Expr,
        comment: Option<String>,
        var_opt: Option<UnitVar>,
    ) -> (Mem, usize) {
        match expr {
            // Expr::Unary { op, box rhs } => {
            // },
            Expr::Binary {
                op,
                box lhs,
                box rhs,
            } => {
                let mut depth = 0;
                // We need a var to store the result of the operation:
                // let var = UnitMem::Var(if let Some(var) = var_opt {
                //     // If a var was provided then it is used and we don't add one to the depth,
                //     // because the expression was part of an l-value r-value assignment.
                //     var
                // } else {
                //     // Else this expression requires a placeholder mem.
                //     // We get the next var and add one to the depth
                //     depth += 1;
                //     self.next_var()
                // });
                let var = self.next_var();
                // Translate the lhs and rhs expressions.
                // If they are num r-values the the depth is zero and the.
                let (lhs, d_l) = self.translate_expr(lhs, None, None);
                let (rhs, d_r) = self.translate_expr(rhs, None, None);
                depth += d_l + d_r;
                let unit_expr = match op {
                    BinaryOp::Add => UnitExpr::Add(var, lhs, rhs),
                    BinaryOp::Sub => UnitExpr::Sub(var, lhs, rhs),
                    BinaryOp::Mul => UnitExpr::Mul(var, lhs, rhs),
                    BinaryOp::Div => UnitExpr::Div(var, lhs, rhs),
                    BinaryOp::NE => UnitExpr::SNE(var, lhs, rhs),
                    _ => unreachable!("{:?}", op),
                };
                self.units.push(Unit::new(unit_expr, comment));
                (Mem::Unit(UnitMem::Var(var)), depth + 1)
            }
            // Expr::Ternary { cond, if_t, if_f } => {
            // },
            // Expr::RValue(rv) => {
            //     self.translate_rvalue(rv);
            // },
            Expr::RValue(rv) => {
                let (mem, _, depth) = self.translate_rvalue(rv, comment, var_opt);
                (mem, depth)
            }
            _ => unreachable!("{:?}", expr),
        }
    }

    fn translate_items(&mut self, items: Vec<Item>, line: usize) -> usize {
        let total_depth = items.into_iter().fold(0, |depth, item| {
            depth + self.translate_item(item, line + depth)
        });
        total_depth
    }

    pub fn translate_item(&mut self, item: Item, line: usize) -> usize {
        let Item {
            item_inner,
            comment,
            ..
        } = item;
        match item_inner {
            ItemInner::Block(block) => {
                match block.branch {
                    Branch::Program => self.translate_items(block.items, 0),
                    // ============================================================================
                    // (INFINITE) LOOP
                    // ============================================================================
                    Branch::Loop => {
                        let depth = self.translate_items(block.items, line);
                        let unit_expr = UnitExpr::JR(Mem::Num(-(depth as f64)));
                        let unit = Unit::new(unit_expr, comment);
                        self.units.push(unit);
                        depth + 1
                    }
                    // ============================================================================
                    // IF STATEMENT
                    // ============================================================================
                    Branch::If(cond) => {
                        // Translate the condition expression and save the index of the final
                        // expression unit for later
                        let (mem, cond_depth) = self.translate_expr(cond, None, None);
                        if cond_depth == 0 {
                            // But if cond_depth == 0 then condition is simply a single value,
                            // and we need to insert a dummy unit for the time being.
                            self.units.push(Unit::new(UnitExpr::Dummy, None));
                        }
                        let i = self.units.len() - 1;
                        // Translate branch body
                        let depth = self.translate_items(block.items, line);
                        // Add the branching statement.
                        let c = Mem::Num((1 + depth) as f64);
                        if cond_depth > 0 {
                            // Convert the final condition expression unit (a variable selection
                            // expresson) into an equivalent branching expression unit.
                            let cond_expr = match self.units[i].unit_expr {
                                UnitExpr::SNE(_r, a, b) => UnitExpr::BRNE(a, b, c),
                                _ => unreachable!("{:?}", self.units[i].unit_expr),
                            };
                            self.units[i].unit_expr = cond_expr;
                        } else {
                            // Else we'll treat the value as true if it is non-zero.
                            let cond_expr = UnitExpr::BRNE(mem, Mem::Num(0.0), c);
                            self.units[i].unit_expr = cond_expr;
                        }
                        cond_depth + depth + 1
                    }
                    // ============================================================================
                    // FOR LOOP
                    // ============================================================================
                    Branch::For(i, s, e, step_opt) => {
                        let mut depth = 0;
                        // START
                        // ===========================
                        let i_var = {
                            // TODO: Probably shouldn't need to type constrict here
                            if let Some(Var::Var(i_var)) = self.var_lookup.get(&i) {
                                *i_var
                            } else {
                                depth += 1;
                                self.next_var()
                            }
                        };
                        let (s, s_depth) = self.translate_expr(s, None, Some(i_var));
                        if depth == 1 {
                            let comment = format!("# {} = ({} = {})", i, i_var, s);
                            self.push_move(i_var, s, Some(comment));
                            depth += 1;
                        }
                        depth += s_depth;
                        let i_unit = UnitMem::Var(i_var);
                        // Add new "i" var to lookup
                        self.var_lookup.insert(i, Var::Mem(i_unit));
                        let i_mem = Mem::Unit(i_unit);
                        // ITEMS
                        // ===========================
                        let mut inner_depth = self.translate_items(block.items, line + depth);
                        // END
                        // ===========================
                        let (e_mem, e_depth) = self.translate_expr(e, None, None);
                        inner_depth += e_depth;
                        // INCREMEMENT AND BRANCH
                        // ===========================
                        let step = if let Some(step_expr) = step_opt {
                            let (step_mem, step_depth) = self.translate_expr(step_expr, None, None);
                            inner_depth += step_depth;
                            step_mem
                        } else {
                            Mem::Num(1.0)
                        };
                        let unit_expr = UnitExpr::Add(i_var, i_mem, step);
                        let unit = Unit::new(unit_expr, None);
                        self.units.push(unit);
                        let unit_expr =
                            UnitExpr::BRLT(i_mem, e_mem, Mem::Num(-(inner_depth as f64)));
                        let unit = Unit::new(unit_expr, comment);
                        self.units.push(unit);
                        depth + inner_depth + 1
                    }
                    // ============================================================================
                    // FUNCTION DEFINITION
                    // ============================================================================
                    Branch::Def(name) => {
                        // Dummy unit for later JR
                        self.units.push(Unit::new(UnitExpr::Dummy, None));
                        // Body
                        let depth = self.translate_items(block.items, line);
                        self.var_lookup
                            .insert(name, Var::Function((1 + line) as f64));
                        // Replace dummy unit with jump relative to skip the body
                        let i = self.units.len() - depth - 1;
                        self.units[i].unit_expr = UnitExpr::JR(Mem::Num((2 + depth) as f64));
                        // Jump to previous location
                        self.units.push(Unit::new(UnitExpr::J(Mem::RA), None));
                        depth + 2
                    }
                    _ => unreachable!("{:?}", block.branch),
                }
                // block.items()
            }
            ItemInner::Stmt(stmt) => {
                match stmt {
                    // ============================================================================
                    // ASSIGN ALIAS
                    // ============================================================================
                    Statement::AssignAlias(a, d) => {
                        match d {
                            Dev::Lit(b, i) => {
                                let unit_alias = UnitAlias(a.clone());
                                let dev = Dev::Lit(b, i);
                                let unit_expr = UnitExpr::Alias(unit_alias, dev);
                                let unit = Unit::new(unit_expr, comment);
                                self.units.push(unit);
                                let alias = Var::Dev(UnitDev::Lit(b, i));
                                self.var_lookup.insert(a, alias);
                            }
                            Dev::Indexed(_) => {
                                unreachable!();
                            }
                            Dev::Batch(hash) => {
                                let alias = Var::DevNet(UnitDevNet::Lit(hash));
                                self.var_lookup.insert(a, alias);
                            }
                            Dev::Var(_v) => {
                                unreachable!();
                            }
                        }
                        1
                    }
                    // ============================================================================
                    // ASSIGN VALUE
                    // ============================================================================
                    Statement::AssignValue(l, r) => {
                        let mut depth = 0;

                        // let (mem, _, r_depth) = self.translate_rvalue(r, None, Some(l_var));
                        // match l {
                        //     LValue::Var(k) => {
                        //     },
                        //     LValue::Param(dev, param) => {
                        //     },
                        // }


                        match l {
                            LValue::Var(k) => {
                                let l_var = if let Some(var) = self.var_lookup.get(&k) {
                                    match var {
                                        Var::Var(uv) => *uv,
                                        _ => unreachable!("{:?}", var),
                                    }
                                } else {
                                    let var = self.next_var();
                                    self.var_lookup.insert(k, Var::Var(var));
                                    // let unit_expr = UnitExpr::Move(UnitMem::Var(var),
                                    depth += 1;
                                    var
                                };
                                let (mem, _, r_depth) = self.translate_rvalue(r, None, Some(l_var));
                                // if depth == 0 {
                                let unit_expr = UnitExpr::Move(UnitMem::Var(l_var), mem);
                                let unit = Unit::new(unit_expr, None);
                                self.units.push(unit);
                                depth += 1;
                                // }
                                depth += r_depth;
                            }
                            LValue::Param(dev, param) => {
                                // let l_var = self.next_var();
                                let (mem, _, r_depth) =
                                    self.translate_rvalue(r, None, None);
                                match dev {
                                    Dev::Var(k) => {
                                        let dev = match self.var_lookup[&k] {
                                            Var::DevNet(dev) => dev,
                                            _ => unreachable!("{:?}", self.var_lookup[&k]),
                                        };
                                        let unit_expr = UnitExpr::SB(dev, param, mem);
                                        let unit = Unit::new(unit_expr, comment);
                                        self.units.push(unit);
                                        depth += 1;
                                    }
                                    _ => unreachable!("{:?}", dev),
                                }
                                depth += r_depth;
                            }
                        };
                        depth
                    }
                    // ============================================================================
                    // FUNCTION CALL
                    // ============================================================================
                    Statement::FunctionCall(name) => {
                        let alias = self.var_lookup.get(&name).unwrap();
                        match alias {
                            Var::Function(line) => {
                                let unit_expr = UnitExpr::JAL(Mem::Num(*line));
                                let unit = Unit::new(unit_expr, comment);
                                self.units.push(unit);
                            }
                            _ => unreachable!("{:?}", alias),
                        }
                        1
                    }
                }
            } // _ => unreachable!("{:?}", stmt),
        }
    }

    fn push_move(&mut self, var: UnitVar, mem: Mem, comment: Option<String>) {
        let unit_expr = UnitExpr::Move(UnitMem::Var(var), mem);
        let unit = Unit::new(unit_expr, comment);
        self.units.push(unit);
    }
}
