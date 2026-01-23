use std::collections::HashMap;
use crate::expr::Expr;

// Representation of a boolean expression in conjunctive normal form
#[derive(Clone, Debug)]
#[allow(dead_code)]
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

#[allow(dead_code)]
impl CNF {
    // Checks if the CNF is satisfiable
    // If the formula is SAT, returns a list of the truth assignments where truth_assignment[i]
    // is the truth assignment of variable with id i + 1
    // Otherwise returns Nonesy
    pub fn find_evidence(self) -> Option<Vec<bool>> {
        let mut truth_assignment = self.gen_empty_truth_assignment();
        if self.dpll(1, &mut truth_assignment) {
            Some(truth_assignment)
        } else {
            None
        }
    }

    // Enforce a certain variable to be either true or false
    pub fn enforce(&mut self, id: isize, value: bool) {
        self.clauses.push(vec![id * if value { 1 } else { -1 }])
    }

    pub fn from_id(&self, id: isize) -> String {
        self.symbol_table[id.abs() as usize - 1].clone()
    }

    // Geterate intermediate variable for expression and return its id
    pub fn gen_var(&mut self, expr: &Expr) -> usize {
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

    pub fn get_symbol_table(&self) -> Vec<String> {
        self.symbol_table.clone()
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

    pub fn gen_empty_truth_assignment(&self) -> Vec<bool> {
         vec![false; self.symbol_table.len()]
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

    pub fn unit_propigation(mut self, truth_assignment: &mut Vec<bool>) -> CNF {
        let mut unit_clause = self.find_unit_clause();

        while let Some(clause) = unit_clause {
            let var_index = (clause.abs() as usize) - 1;
            let truth_value = clause > 0;

            truth_assignment[var_index] = truth_value;
 
            self = self.conditioned(clause);

            unit_clause = self.find_unit_clause();
        }

        self
    }

    // Returns if CNF is satisfiable (using DPLL algorithm), takes the variable we want to condition on
    // TODO undobacktracking instead of cloneing
    fn dpll(mut self, current: isize, truth_assignment: &mut Vec<bool>) -> bool {
        self = self.unit_propigation(truth_assignment);
        // TODO pure literal elimination

        if self.clauses.len() == 0 {
            return true;
        }

        if self.contains_empty_clause() {
            return false;
        }

        let next_index = current as usize;

        truth_assignment[next_index] = true;
        if self.conditioned(current).dpll(current + 1, truth_assignment) {
            return true;
        }

        truth_assignment[next_index] = false;
        if self.conditioned(-current).dpll(current + 1, truth_assignment) {
            return true;
        }

        false
    }
}
