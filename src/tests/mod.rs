use crate::cnf::*;
use crate::expr::*;
use crate::lexer::*;
use crate::runtime::{ vm::*, env::* };


#[test]
fn unit_propigation() {
    let symbol_table = vec![String::from("x"), String::from("y"), String::from("z"), String::from("w")];
    let symbol_table = symbol_table;

    let cnf = CNF::new(symbol_table.clone(), vec![vec![1], vec![1, -2]]);
    let mut truth_assignment = cnf.gen_empty_truth_assignment();

    assert_eq!(cnf.unit_propigation_old(&mut truth_assignment).get_clauses_clone(), Vec::<Vec<isize>>::new());


    let cnf = CNF::new(symbol_table.clone(), vec![vec![-3, 1, 2, 4], vec![-2], vec![3]]);
    let mut truth_assignment = cnf.gen_empty_truth_assignment();

    assert_eq!(cnf.unit_propigation_old(&mut truth_assignment).get_clauses_clone(), vec![vec![1, 4]]);
}

#[test]
fn dpll() {
    let symbol_table = vec![String::from("x"), String::from("y"), String::from("z")];
    let symbol_table = symbol_table;

    let cnf = CNF::new(symbol_table.clone(), vec![vec![1], vec![-1]]);
    assert!(!cnf.find_evidence().is_some());

    // Argument x -> y, x, therefore y
    let cnf = CNF::new(symbol_table.clone(), vec![vec![-1, 2], vec![1], vec![-2]]);
    assert!(!cnf.find_evidence().is_some());

    // Invalid argument x -> y, y, therefore x
    let cnf = CNF::new(symbol_table.clone(), vec![vec![-1, 2], vec![2], vec![-1]]);
    assert!(cnf.find_evidence().is_some());
}

#[test]
fn tseitin() {
    // Expr not (a and b) or c
    let a = Expr::Var(String::from("a"));
    let b = Expr::Var(String::from("b"));
    let c = Expr::Var(String::from("c"));
    let expr = Expr::or(Expr::not(Expr::and(a, b)), c);

    let cnf = expr.tseitin(false);
    assert!(cnf.find_evidence().is_some());

    // Expr not (a or b) and a
    let a = Expr::Var(String::from("a"));
    let b = Expr::Var(String::from("b"));
    let expr = Expr::and(Expr::not(Expr::or(a.clone(), b)), a);

    let cnf = expr.tseitin(false);
    assert!(!cnf.find_evidence().is_some())
}

#[test]
fn lex() {
    let bytes = "T F _TF 9abc_ (h)and or not nor nand xor -><->".as_bytes();
    let mut lexer = Lexer::new(bytes).unwrap();
    assert_eq!(lexer.lex_all().unwrap(), vec![
        Tok::T,
        Tok::F,
        Tok::Identifier(String::from("_TF")),
        Tok::Identifier(String::from("9abc_")),
        Tok::LPAREN,
        Tok::Identifier(String::from("h")),
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
    let c = Expr::Var(String::from("c"));
    let expected = Expr::iff(
        Expr::eif(
            Expr::xor(
                Expr::nand(a.clone(), Expr::or(Expr::and(Expr::nor(a, Expr::not(b.clone())), b.clone()), c.clone())),
                c.clone()
            ),
            b
        ),
        c
    );
    assert_eq!(Expr::parse("a nand (a nor not b and b or c) xor c -> b <-> c".as_bytes()).unwrap(), expected)
}

#[test]
fn vm() {
    use OpCode::*;

    let mut env = Env::new();
    env.define(String::from("a"), true);
    env.define(String::from("b"), false);
    
    let mut vm = VM::new(&mut env, vec![
        Load(String::from("a")),
        T,
        And,
    ]);

    assert!(vm.eval().unwrap());
    
    let mut vm = VM::new(&mut env, vec![
        Load(String::from("a")),
        Not,
    ]);

    assert!(!vm.eval().unwrap());

    let mut vm = VM::new(&mut env, vec![
        Load(String::from("a")),
        Load(String::from("b")),
        Or,
    ]);

    assert!(vm.eval().unwrap());

    let mut vm = VM::new(&mut env, vec![
        Load(String::from("c")),
    ]);

    assert!(vm.eval().is_err());

    let mut vm = VM::new(&mut env, vec![
        T,
        F,
        Or,
        F,
        And,
    ]);

    assert!(!vm.eval().unwrap());
}

#[test]
fn compilation() {
    let mut env = Env::new();
    env.define(String::from("a"), true);
    env.define(String::from("b"), false);
    
    let expr = Expr::parse("(a xor b) and not b".as_bytes()).unwrap();
    let mut vm = VM::new(&mut env, expr.compile());
    assert!(vm.eval().unwrap());

    let expr = Expr::parse("(a nand a) nor b".as_bytes()).unwrap();
    let mut vm = VM::new(&mut env, expr.compile());

    assert!(vm.eval().unwrap());

    let expr = Expr::parse("not (T -> F) <-> F".as_bytes()).unwrap();
    let mut vm = VM::new(&mut env, expr.compile());

    assert!(!vm.eval().unwrap());
}

#[test]
fn sat_evidence() {
    let prop = "(not a and b) or (c xor d) -> (e nand f)";
    let expr = Expr::parse(prop.as_bytes()).unwrap();
    let cnf = expr.tseitin(false);
    let symbol_table = cnf.get_symbol_table();
    let proof = cnf.find_evidence().unwrap();

    assert!(expr.is_valid_sat_proof(&proof, &symbol_table));

    let prop = "not (((a or (b and c) <-> d) xor a or (b and c) <-> d) nand e) or not (not f)";
    let expr = Expr::parse(prop.as_bytes()).unwrap();
    let cnf = expr.tseitin(false);
    let symbol_table = cnf.get_symbol_table();
    let proof = cnf.find_evidence().unwrap();

    assert!(expr.is_valid_sat_proof(&proof, &symbol_table));

    let prop = "(e nand f) and not g <-> (h or i) xor (not j nor k)";
    let expr = Expr::parse(prop.as_bytes()).unwrap();
    let cnf = expr.tseitin(false);
    let symbol_table = cnf.get_symbol_table();
    let proof = cnf.find_evidence().unwrap();

    assert!(expr.is_valid_sat_proof(&proof, &symbol_table));
}

#[test]
fn falsification() {
    assert!(CNF::is_falsified(&vec![-1, 2, 3], &vec![Some(true), Some(false), Some(false)]));
    assert!(!CNF::is_falsified(&vec![1, -2, 3], &vec![Some(false), Some(true), None]));
    assert!(!CNF::is_falsified(&vec![1, -2, 3], &vec![Some(true), Some(false), Some(false)]));
    assert!(!CNF::is_falsified(&vec![1, 2, 3], &vec![Some(true), Some(true), Some(true)]));
    assert!(CNF::is_falsified(&vec![-1, -2, -3], &vec![Some(true), Some(true), Some(true)]));
}

#[test]
fn is_unit_clause() {
    assert!(CNF::is_unit_clause(&vec![-1, 2, 3], &vec![Some(true), Some(false), None]));
    assert!(!CNF::is_unit_clause(&vec![1, -2, 3], &vec![Some(true), Some(true), None]));
    assert!(!CNF::is_unit_clause(&vec![1, -2, 3], &vec![Some(false), Some(true), Some(false)]));
    assert!(CNF::is_unit_clause(&vec![1, 2, 3], &vec![None, Some(false), Some(false)]));
    assert!(!CNF::is_unit_clause(&vec![-1, -2, -3], &vec![Some(true), Some(true), Some(true)]));
}
