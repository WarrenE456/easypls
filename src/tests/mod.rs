use crate::easypls::*;

use std::sync::Arc;

#[test]
fn unit_clause_finding_and_resolution() {
    let symbol_table = vec![String::from("x"), String::from("y"), String::from("z")];
    let symbol_table = Arc::new(symbol_table);

    let cnf = CNF::new(Arc::clone(&symbol_table), vec![vec![1], vec![1, -2]]);
    let unit_clauses = cnf.find_unit_clauses();

    assert_eq!(unit_clauses, vec![1]);
    assert_eq!(cnf.conditioned(1).get_clauses_clone(), Vec::<Vec<isize>>::new());


    let cnf = CNF::new(Arc::clone(&symbol_table), vec![vec![1, 2, -3], vec![-2], vec![3]]);
    let unit_clauses = cnf.find_unit_clauses();

    assert_eq!(unit_clauses, vec![-2, 3]);
    assert_eq!(cnf.conditioned(-2).conditioned(3).get_clauses_clone(), vec![vec![1]]);
}
