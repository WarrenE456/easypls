use crate::easypls::*;

#[test]
fn unit_propigation() {
    let symbol_table = vec![String::from("x"), String::from("y"), String::from("z"), String::from("w")];
    let symbol_table = symbol_table;

    let cnf = CNF::new(symbol_table.clone(), vec![vec![1], vec![1, -2]]);
    assert_eq!(cnf.unit_propigation().get_clauses_clone(), Vec::<Vec<isize>>::new());


    let cnf = CNF::new(symbol_table.clone(), vec![vec![-3, 1, 2, 4], vec![-2], vec![3]]);

    assert_eq!(cnf.unit_propigation().get_clauses_clone(), vec![vec![1, 4]]);
}

#[test]
fn dpll() {
    let symbol_table = vec![String::from("x"), String::from("y"), String::from("z")];
    let symbol_table = symbol_table;

    let cnf = CNF::new(symbol_table.clone(), vec![vec![1], vec![-1]]);
    assert!(!cnf.is_sat());

    // Argument x -> y, x, therefore y
    let cnf = CNF::new(symbol_table.clone(), vec![vec![-1, 2], vec![1], vec![-2]]);
    assert!(!cnf.is_sat());

    // Invalid argument x -> y, y, therefore x
    let cnf = CNF::new(symbol_table.clone(), vec![vec![-1, 2], vec![2], vec![-1]]);
    assert!(cnf.is_sat());
}

#[test]

fn tseitin() {
    let expr = Expr::Or(Or::new(Box::new(Expr::Var("a".to_string())), Box::new(Expr::Var("b".to_string()))));
    let cnf = expr.into_cnf();
    println!("CNF: {:?}", cnf);
}
