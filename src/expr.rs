use std::cell::RefCell;
use crate::cnf::CNF;

#[derive(Clone)]
#[allow(dead_code)]
pub enum Expr {
    And(And),
    Or(Or),
    Not(Not),
    Var(String),
}

#[allow(dead_code)]
impl Expr {
    // Converts expression into an equisatisfyable CNF via the tseitin transformation
    pub fn tseitin(&self) -> CNF {
        let mut cnf = CNF::new(Vec::new(), Vec::new());
        let id = cnf.gen_var(self) as isize;
        cnf.append_clause(vec![id]);

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
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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
