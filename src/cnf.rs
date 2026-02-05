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
    pub fn find_evidence(&mut self) -> Option<Vec<bool>> {
        let mut truth_assignment = self.gen_empty_truth_assignment();
        self.dpll(&mut truth_assignment)
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

    pub fn gen_empty_truth_assignment(&self) -> Vec<Option<bool>> {
         vec![None; self.symbol_table.len()]
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

    pub fn unit_propigation_old(mut self, truth_assignment: &mut Vec<bool>) -> CNF {
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
    fn dpll_old(mut self, current: isize, truth_assignment: &mut Vec<bool>) -> bool {
        self = self.unit_propigation_old(truth_assignment);
        // TODO pure literal elimination

        if self.clauses.len() == 0 {
            return true;
        }

        if self.contains_empty_clause() {
            return false;
        }

        let next_index = current as usize;

        truth_assignment[next_index] = true;
        if self.conditioned(current).dpll_old(current + 1, truth_assignment) {
            return true;
        }

        truth_assignment[next_index] = false;
        if self.conditioned(-current).dpll_old(current + 1, truth_assignment) {
            return true;
        }

        false
    }

    pub fn is_falsified(clause: &Vec<isize>, truth_assignment: &Vec<Option<bool>>) -> bool {
        for var in clause {
            let value = *var > 0;
            let var_idx = var.abs() as usize - 1;
            if let Some(assigned) = truth_assignment[var_idx] {
                if assigned == value {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn any_falsified(&self, truth_assignment: &Vec<Option<bool>>) -> bool {
        for clause in self.clauses.iter() {
            if Self::is_falsified(clause, truth_assignment) {
                return true;
            }
        }
        false
    }

    pub fn is_unit_clause(clause: &Vec<isize>, truth_assignment: &Vec<Option<bool>>) -> bool {
        Self::implied_assignment(clause, truth_assignment).is_some()
    }

    // Checks if clause is a unit clause
    // If so we return the index and value of the implied variable
    // If not we return None
    pub fn implied_assignment(clause: &Vec<isize>, truth_assignment: &Vec<Option<bool>>) -> Option<(usize, bool)> {
        let mut undef = None;
        for var in clause {
            let value = *var > 0;
            let idx = var.abs() as usize - 1;

            match truth_assignment[idx] {
                None => {
                    if undef.is_some() {            // More than one undefined var
                        return None;
                    } else {
                        undef = Some((idx, value));
                    }
                }

                Some(assigned) => {
                    if assigned == value {          // Entire clause cancles because literal evalues to true
                        return None;
                    }
                }
            }
        }
        undef
    }

    fn find_implied_assignment(&self, truth_assignment: &Vec<Option<bool>>) -> Option<(usize, bool)> {
        for clause in self.clauses.iter() {
            let implied_assignment = Self::implied_assignment(clause, truth_assignment);
            if implied_assignment.is_some() {
                return implied_assignment;
            }
        } 
        None
    }

    fn next_undef(truth_assignment: &Vec<Option<bool>>) -> Option<usize> {
        for (idx, assignment) in truth_assignment.iter().enumerate() {
            if assignment.is_none() {
                return Some(idx);
            }
        }
        None
    }

    fn try_assignment(&mut self, truth_assignment: &mut Vec<Option<bool>>, idx: usize, value: bool) -> Option<Vec<bool>> {
        let mut truth_assignment = truth_assignment.clone();
        truth_assignment[idx] = Some(value);
        let assignment =  self.dpll(&mut truth_assignment);
        // truth_assignment[idx] = None;
        assignment
    }

    // Uses dpll algorithm to check for SAT
    // Returns satisfying truth assignment if SAT,
    // Returns None if UNSAT
    fn dpll(&mut self, truth_assignment: &mut Vec<Option<bool>>) -> Option<Vec<bool>> {
        if self.any_falsified(truth_assignment) {
            return None;
        }

        while let Some(assignment) = self.find_implied_assignment(truth_assignment) {
            let (idx, value) = assignment;
            truth_assignment[idx] = Some(value);

            if self.any_falsified(truth_assignment) {
                return None;
            }
        }

        let next = match Self::next_undef(truth_assignment) {
            None => return Some(truth_assignment.clone().into_iter().map(|v| v.unwrap()).collect()),
            Some(idx) => idx,
        };

        
        if let Some(satisfying_assignment) = self.try_assignment(truth_assignment, next, true) {
            Some(satisfying_assignment)
        }
        else if let Some(satisfying_assignment) = self.try_assignment(truth_assignment, next, false) {
            Some(satisfying_assignment)
        } else {
            None
        }
    }
}
