use std::collections::HashSet;
use std::cell::RefCell;

use crate::cnf::CNF;
use crate::runtime::{env::Env, vm::VM};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runtime::vm::OpCode;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Expr {
    And(And),
    Or(Or),
    Not(Not),
    Literal(bool),
    Var(String),
}

#[allow(dead_code)]
impl Expr {
    pub fn parse(src: &[u8]) -> Result<Expr, String> {
        Parser::from(Lexer::new(src)?).statement()
    }

    pub fn get_variables_aux(&self, vars: &mut HashSet<String>) {
        use Expr::*;
        match self {
            And(and) => {
                and.l.get_variables_aux(vars);
                and.r.get_variables_aux(vars);
            }
            Or(or) => {
                or.l.get_variables_aux(vars);
                or.r.get_variables_aux(vars);
            }
            Not(not) => {
                not.expr.get_variables_aux(vars);
            }
            Literal(_) => {}
            Var(name) => {
                vars.insert(name.clone());
            }
        }
    }

    pub fn get_variables(&self) -> Vec<String> {
        let mut vars = HashSet::new();
        self.get_variables_aux(&mut vars);

        vars.into_iter().collect::<Vec<_>>()
    }

    fn truth_table_aux(expr: &Vec<OpCode>, i: usize, vars: &Vec<String>, env: &mut Env) {
        if i >= vars.len() {
            let result = if VM::new(env, expr.clone()).run().unwrap() {
                "T"
            } else {
                "F"
            };

            let vars_list = vars.iter()
                .map(|var| format!("{}={}", var, if env.get(var).unwrap() { "T" } else { "F" }))
                .collect::<Vec<_>>()
                .join(", ");

            println!("{vars_list}: {result}");
            return;
        }

        let var = vars[i].clone();

        env.define(var.clone(), false);
        Self::truth_table_aux(expr, i + 1, vars, env);

        env.define(var.clone(), true);
        Self::truth_table_aux(expr, i + 1, vars, env);
    }

    pub fn truth_table(prop: String) -> Result<(), String> {
        println!("{prop}:");
        let expr = Expr::parse(prop.as_bytes())?;
        let vars = expr.get_variables();

        let mut env = Env::new();
        Self::truth_table_aux(&expr.compile(), 0, &vars, &mut env);
        Ok(())
    }

    // Converts expression into an equisatisfyable CNF via the tseitin transformation
    pub fn tseitin(&self) -> CNF {
        let mut cnf = CNF::new(Vec::new(), Vec::new());
        let id = cnf.gen_var(self) as isize;

        cnf.enforce(id, true);       // Enforces that the entire expression is true

        let cnf_refcell = RefCell::new(cnf);
        self.tseitin_aux(id, &cnf_refcell);

        cnf = cnf_refcell.into_inner();
        cnf
    }

    // Performs a Tseitin transformation
    // Takes its own id in the CNF, and a refrence to the CNF which we are building
    // Mutate the CNF rather than returning a value
    pub fn tseitin_aux(&self, id: isize, cnf: &RefCell<CNF>) {
        match self {
            Expr::Var(name) => self.sub_var_name(name.clone(), id as usize, cnf),
            Expr::Literal(value) => cnf.borrow_mut().enforce(id, *value),
            Expr::Or(or) => or.tseitin(id, cnf),
            Expr::And(and) => and.tseitin(id, cnf),
            Expr::Not(not) => not.tseitin(id, cnf),
        }
    }

    // Substitute variable's temporary name for its actual name
    pub fn sub_var_name(&self, name: String, id: usize, cnf: &RefCell<CNF>) {
        cnf.borrow_mut().set_symbol_name(id, name);
    }

    // Create "and" expression
    pub fn and(l: Expr, r: Expr) -> Expr {
        Expr::And(And::new(Box::new(l), Box::new(r)))
    }

    // Create "or" expression
    pub fn or(l: Expr, r: Expr) -> Expr {
        Expr::Or(Or::new(Box::new(l), Box::new(r)))
    }

    // Create "not" expression
    pub fn not(subexpr: Expr) -> Expr {
        Expr::Not(Not::new(Box::new(subexpr)))
    }

    // Create "if" expression
    pub fn eif(l: Expr, r: Expr) -> Expr {
        Expr::or(Expr::not(l), r)
    }

    // Create "iff" expression
    pub fn iff(l: Expr, r: Expr) -> Expr {
        Expr::and(Expr::eif(l.clone(), r.clone()), Expr::eif(r, l))
    }

    // Create "xor" expression
    pub fn xor(l: Expr, r: Expr) -> Expr {
        Expr::and(Expr::or(l.clone(), r.clone()), Expr::not(Expr::and(l, r)))
    }

    // Create "nand" expression
    pub fn nand(l: Expr, r: Expr) -> Expr {
        Expr::not(Expr::and(l, r))
    }

    // Create "nor" expression
    pub fn nor(l: Expr, r: Expr) -> Expr {
        Expr::not(Expr::or(l, r))
    }

    pub fn compile(&self) -> Vec<OpCode> {
        let mut codes = Vec::new();
        self.compile_aux(&mut codes);
        codes
    }

    fn compile_aux(&self, codes: &mut Vec<OpCode>) {
        match self {
            Expr::Not(not) => {
                not.expr.compile_aux(codes);
                codes.push(OpCode::Not);
            }
            Expr::And(and) => {
                and.l.compile_aux(codes);
                and.r.compile_aux(codes);
                codes.push(OpCode::And);
            }
            Expr::Or(and) => {
                and.l.compile_aux(codes);
                and.r.compile_aux(codes);
                codes.push(OpCode::Or);
            }
            Expr::Literal(b) => {
                codes.push(if *b { OpCode::T } else { OpCode::F });
            }
            Expr::Var(name) => {
                codes.push(OpCode::Load(name.clone()));
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct And {
    l: Box<Expr>,       // left-hand side
    r: Box<Expr>,       // right-hand side
}

impl And {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> And {
        And {l, r}
    }

    pub fn tseitin(&self, id: isize, cnf: &RefCell<CNF>) {
        let (l_id, r_id) = {
            let mut cnf_ref = cnf.borrow_mut();

            let l_id = cnf_ref.gen_var(&self.l) as isize;
            let r_id = cnf_ref.gen_var(&self.r) as isize;

            cnf_ref.append_clause(vec![-id, l_id]);
            cnf_ref.append_clause(vec![-id, r_id]);
            cnf_ref.append_clause(vec![id, -l_id, -r_id]);

            (l_id, r_id)
        };

        self.l.tseitin_aux(l_id, cnf);
        self.r.tseitin_aux(r_id, cnf);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Or {
    l: Box<Expr>,       // left-hand side
    r: Box<Expr>,       // right-hand side
}

impl Or {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Or {
        Or {l, r}
    }

    pub fn tseitin(&self, id: isize, cnf: &RefCell<CNF>) {
        let (l_id, r_id) = {
            let mut cnf_ref = cnf.borrow_mut();

            let l_id = cnf_ref.gen_var(&self.l) as isize;
            let r_id = cnf_ref.gen_var(&self.r) as isize;

            cnf_ref.append_clause(vec![-id, l_id, r_id]);
            cnf_ref.append_clause(vec![id, -l_id]);
            cnf_ref.append_clause(vec![id, -r_id]);

            (l_id, r_id)
        };

        self.l.tseitin_aux(l_id, cnf);
        self.r.tseitin_aux(r_id, cnf);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Not {
    expr: Box<Expr>
}

impl Not {
    pub fn new(expr: Box<Expr>) -> Not {
        Not { expr }
    }

    pub fn tseitin(&self, id: isize, cnf: &RefCell<CNF>) {
        let subexpr_id = {
            let mut cnf_ref = cnf.borrow_mut();

            let subexpr_id = cnf_ref.gen_var(&self.expr) as isize;

            cnf_ref.append_clause(vec![-id, -subexpr_id]);
            cnf_ref.append_clause(vec![id, subexpr_id]);

            subexpr_id
        };

        self.expr.tseitin_aux(subexpr_id, cnf);
    }
}
