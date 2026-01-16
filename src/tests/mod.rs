use crate::cnf::*;
use crate::expr::*;
use crate::lexer::*;
use crate::parser::*;

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
    // Expr not (a and b) or c
    let a = Expr::Var(String::from("a"));
    let b = Expr::Var(String::from("b"));
    let c = Expr::Var(String::from("c"));
    let expr = Expr::or(Expr::not(Expr::and(a, b)), c);

    let cnf = expr.tseitin();
    assert!(cnf.is_sat());

    // Expr not (a or b) and a
    let a = Expr::Var(String::from("a"));
    let b = Expr::Var(String::from("b"));
    let expr = Expr::and(Expr::not(Expr::or(a.clone(), b)), a);

    let cnf = expr.tseitin();
    assert!(!cnf.is_sat())
}

#[test]
fn lex() {
    let bytes = "T F _TF 9abc_ ()and or not nor nand xor -><->".as_bytes();
    let mut lexer = Lexer::new(bytes).unwrap();
    assert_eq!(lexer.lex_all().unwrap(), vec![
        Tok::T,
        Tok::F,
        Tok::Identifier(String::from("_TF")),
        Tok::Identifier(String::from("9abc_")),
        Tok::LPAREN,
        Tok::RPAREN,
        Tok::And,
        Tok::Or,
        Tok::Not,
        Tok::Nor,
        Tok::Nand,
        Tok::Xor,
        Tok::If,
        Tok::Iff,
    ]);

    let bytes = "$test".as_bytes();
    let mut lexer = Lexer::new(bytes).unwrap();
    assert!(lexer.advance_tok().is_err())
}

#[test]
fn parse() {
    let a = Expr::Var(String::from("a"));
    let b = Expr::Var(String::from("b"));
    assert_eq!(Expr::parse("a and b".as_bytes()).unwrap(), Expr::and(a, b))
}
