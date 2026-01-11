#[cfg(test)]
mod tests;

use pyo3::prelude::*;


#[pymodule]
pub mod easypls {
    use pyo3::prelude::*;
    use std::sync::Arc;

    // Representation of a boolean expression in conjunctive normal form
    #[pyclass]
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
        // Return list of all unit clauses
        pub fn find_unit_clauses(&self) -> Vec<isize> {
            let mut unit_clauses = Vec::new();
            for clause in self.clauses.iter() {
                if clause.len() == 1 {
                    unit_clauses.push(clause[0]);
                }
            }
            unit_clauses
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
                        break 'clause_loop;
                    }
                    if *var != -target {
                        new_clause.push(*var)
                    }
                }

                new_clauses.push(new_clause);
            }

            self.from_self(new_clauses)
        }
        
        // Returns if CNF is sat, takes the variable we want to condition on
        fn is_sat_helper(&self, current: isize) -> Option<Vec<usize>> {
            // Unit propigation
            // Check if there are no clauses remaining, if so return true
            // Check if there are any empty clauses, if so return false
            // Condition on the next variable
            todo!()
        }
    }

    #[pymethods]
    impl CNF {
        // Returns whether a CNF is satisfiable
        // if so we return the signature that makes the CNF true.
        // otherwise we return None
        pub fn is_sat(&self) -> Option<Vec<usize>> {
            self.is_sat_helper(1)
        }
    }
}
