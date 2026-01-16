#[cfg(test)]
mod tests;

use pyo3::prelude::*;


#[pymodule]
pub mod easypls {
    use pyo3::prelude::*;
    use std::cell::RefCell;

    pub enum Expr {
        And(And),
        Or(Or),
        Not(Not),
        Var(String),
    }

    impl Expr {
        // Converts expression into an CNF via the tseintein transformation
        pub fn into_cnf(&self) -> CNF {
            let mut cnf = CNF::new(Vec::new(), Vec::new());
            let id = cnf.gen_var() as isize;

            let cnf_refcell = RefCell::new(cnf);
            self.tseitin(id, &cnf_refcell);

            cnf = cnf_refcell.into_inner();
            cnf
        }

        // Performs a Tseitin transformation
        // Takes its own id in the CNF, and a refrence to the CNF which we are building
        // Mutate the CNF rather than returning a value
        pub fn tseitin(&self, id: isize, cnf: &RefCell<CNF>) {
            match self {
                Expr::Var(name) => self.sub_var_name(name.clone(), id as usize, cnf),
                Expr::Or(or) => or.tseitin(id, cnf),
                Expr::And(_) => todo!(),
                Expr::Not(_) => todo!(),
            }
        }

        // Substitute variable's temporary name for its actual name
        pub fn sub_var_name(&self, name: String, id: usize, cnf: &RefCell<CNF>) {
            cnf.borrow_mut().set_symbol_name(id, name);
        }
    }

    pub struct And {
        l: Box<Expr>,       // left-hand side
        r: Box<Expr>,       // right-hand side
    }

    impl And {
        pub fn new(l: Box<Expr>, r: Box<Expr>) -> And {
            And {l, r}
        }
    }

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

                let l_id = cnf_ref.gen_var() as isize;
                let r_id = cnf_ref.gen_var() as isize;

                cnf_ref.append_clause(vec![-id, l_id, r_id]);
                cnf_ref.append_clause(vec![id, -l_id]);
                cnf_ref.append_clause(vec![id, -r_id]);

                (l_id, r_id)
            };

            self.l.tseitin(l_id, cnf);
            self.r.tseitin(r_id, cnf);
        }
    }

    pub struct Not {
        expr: Box<Expr>
    }

    impl Not {
        pub fn new(expr: Box<Expr>) -> Not {
            Not { expr }
        }
    }

    // Representation of a boolean expression in conjunctive normal form
    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct CNF {
        // Symbol_table[id - 1] represents symbol of variable with id
        // We define it this way because 0 isn't distict from -0
        symbol_table: Vec<String>,

        // Outer list represents conjunction of inner lists which
        // represent disjunctions of variables, represented by their ids
        // -id represents negation
        clauses: Vec<Vec<isize>>,

        // Note: used for Tseitin transformations
        // Counter of intermediate variables we've created, TODO base 64
        counter: usize,
    }

    impl CNF {
        // Geterate intermediate variable and return its id
        fn gen_var(&mut self) -> usize {
            let name = format!("${}", self.counter);
            self.counter += 1;

            let id = self.add_variable(name);

            id
        }

        pub fn set_symbol_name(&mut self, id: usize, name: String) {
            self.symbol_table[id] = name;
        }

        // Add variabel and returns its id
        pub fn add_variable(&mut self, name: String) -> usize {
            let id = self.symbol_table.len();
            self.symbol_table.push(name);

            id
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
            CNF { symbol_table, clauses, counter: 0 }
        }

        pub fn get_clauses_clone(&self) -> Vec<Vec<isize>> {
            self.clauses.clone()
        }

        // Create new CNF with same symbol table
        pub fn from_self(&self, clauses: Vec<Vec<isize>>) -> CNF {
            CNF { symbol_table: self.symbol_table.clone(), clauses, counter: 0 }
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
