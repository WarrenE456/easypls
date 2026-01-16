#[cfg(test)]
mod tests;

use pyo3::prelude::*;



#[pymodule]
pub mod easypls {
    use pyo3::prelude::*;

    use std::cell::RefCell;
    use std::collections::HashMap;

    #[derive(Clone)]
    pub enum Expr {
        And(And),
        Or(Or),
        Not(Not),
        Var(String),
    }

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

    // Representation of a boolean expression in conjunctive normal form
    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct CNF {
        // Symbol_table[id - 1] represents symbol of variable with id
        // We define it this way because 0 isn't distict from -0
        symbol_table: Vec<String>,

        // Maps variable name to ID
        name_to_id: HashMap<String, usize>, 

        // Outer list represents conjunction of inner lists which
        // represent disjunctions of variables, represented by their ids
        // -id represents negation
        clauses: Vec<Vec<isize>>,

        // Note: used for Tseitin transformations
        // Counter of intermediate variables we've created, TODO base 64
        counter: usize,
    }

    impl CNF {
        // Geterate intermediate variable for expression and return its id
        fn gen_var(&mut self, expr: &Expr) -> usize {
            // Handle simple variable
            if let Expr::Var(name) = expr {
                if let Some(id) = self.name_to_id.get(name) {
                    return *id;
                } else {
                    let id = self.add_variable(name.clone());
                    self.name_to_id.insert(name.clone(), id);
                    return id;
                }
            }

            // Handle sub-expression
            let name = format!("${}", self.counter);
            self.counter += 1;

            let id = self.add_variable(name);

            id
        }

        pub fn set_symbol_name(&mut self, id: usize, name: String) {
            self.symbol_table[id - 1] = name;
        }

        // Add variabel and returns its id
        pub fn add_variable(&mut self, name: String) -> usize {
            let id = self.symbol_table.len();
            self.symbol_table.push(name);

            id + 1
        }

        pub fn append_clause(&mut self, clause: Vec<isize>) {
            self.clauses.push(clause);
        }

        // Returns first unit clause or None if there are no unit cclauses
        pub fn find_unit_clause(&self) -> Option<isize> {
            for clause in self.clauses.iter() {
                if clause.len() == 1 {
                    return Some(clause[0]);
                }
            }
            None
        }

        pub fn new(symbol_table: Vec<String>, clauses: Vec<Vec<isize>>) -> CNF {
            CNF { symbol_table, clauses, counter: 0, name_to_id: HashMap::new() }
        }

        pub fn get_clauses_clone(&self) -> Vec<Vec<isize>> {
            self.clauses.clone()
        }

        // Create new CNF with same symbol table
        pub fn from_self(&self, clauses: Vec<Vec<isize>>) -> CNF {
            CNF { symbol_table: self.symbol_table.clone(), clauses, counter: 0, name_to_id: HashMap::new() }
        }

        // Return a CNF after conditioning on some variable target
        // TODO optimize with binary search
        pub fn conditioned(&self, target: isize) -> CNF {
            let mut new_clauses = Vec::new();

            'clause_loop: for clause in self.clauses.iter() {
                let mut new_clause = Vec::new();

                for var in clause {
                    if *var == target {
                        continue 'clause_loop;
                    }
                    if *var != -target {
                        new_clause.push(*var)
                    }
                }

                new_clauses.push(new_clause);
            }

            self.from_self(new_clauses)
        }

        fn contains_empty_clause(&self) -> bool {
            for clause in self.clauses.iter() {
                if clause.len() == 0 {
                    return true;
                }
            }
            false
        }

        pub fn unit_propigation(mut self) -> CNF {
            let mut unit_clause = self.find_unit_clause();

            while let Some(clause) = unit_clause {
                self = self.conditioned(clause);

                unit_clause = self.find_unit_clause();
            }

            self
        }

        // Returns if CNF is satisfiable (using DPLL algorithm), takes the variable we want to condition on
        // TODO undobacktracking instead of cloneing
        fn dpll(mut self, current: isize) -> bool {
            self = self.unit_propigation();
            // TODO pure literal elimination

            if self.clauses.len() == 0 {
                return true;
            }

            if self.contains_empty_clause() {
                return false;
            }

            self.conditioned(current).dpll(current + 1) || self.conditioned(-current).dpll(current + 1)
        }
    }

    #[pymethods]
    impl CNF {
        // Returns whether a CNF is satisfiable
        pub fn is_sat(&self) -> bool {
            self.clone().dpll(1)
        }
    }
}
