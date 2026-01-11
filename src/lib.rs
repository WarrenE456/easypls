#[cfg(test)]
mod tests;

use pyo3::prelude::*;


#[pymodule]
pub mod easypls {
    use pyo3::prelude::*;
    use std::sync::Arc;

    // Representation of a boolean expression in conjunctive normal form
    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct CNF {
        // Symbol_table[id - 1] represents symbol of variable with id
        // We define it this way because 0 isn't distict from -0
        symbol_table: Arc<Vec<String>>,

        // Outer list represents conjunction of inner lists which
        // represent disjunctions of variables, represented by their ids
        // -id represents negation
        clauses: Vec<Vec<isize>>,
    }

    impl CNF {
        // Returns first unit clause or None if there are no unit cclauses
        pub fn find_unit_clause(&self) -> Option<isize> {
            for clause in self.clauses.iter() {
                if clause.len() == 1 {
                    return Some(clause[0]);
                }
            }
            None
        }

        pub fn new(symbol_table: Arc<Vec<String>>, clauses: Vec<Vec<isize>>) -> CNF {
            CNF {symbol_table, clauses }
        }

        pub fn get_clauses_clone(&self) -> Vec<Vec<isize>> {
            self.clauses.clone()
        }

        // Create new CNF with same symbol table
        pub fn from_self(&self, clauses: Vec<Vec<isize>>) -> CNF {
            CNF { symbol_table: Arc::clone(&self.symbol_table), clauses }
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

            print!("before: ");
            dbg!(&self);
            while let Some(clause) = unit_clause {
                println!("Conditioning on {clause}");
                self = self.conditioned(clause);

                unit_clause = self.find_unit_clause();
            }
            print!("after: ");
            dbg!(&self);

            self
        }

        // Returns if CNF is satisfiable (using DPLL algorithm), takes the variable we want to condition on
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
