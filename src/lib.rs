#[cfg(test)]
mod tests;

pub mod expr;
pub mod cnf;
pub mod lexer;
pub mod parser;
pub mod runtime;

// Python API
use pyo3::prelude::*;
#[pymodule]
pub mod easypls {
    use pyo3::exceptions::PyException;
    use pyo3::prelude::*;

    use crate::cnf::CNF;

    #[pyclass(name="CNF")]
    struct PyCNF {
        cnf: CNF
    }

    impl PyCNF {
        pub fn new(cnf: CNF) -> PyCNF {
            PyCNF { cnf }
        }
    }

    #[pymethods]
    impl PyCNF {
        fn is_sat(&self) -> bool {
            self.cnf.clone().find_evidence().is_some()
        }

        fn sat_evidence(&self) -> Option<Vec<bool>> {
            self.cnf.clone().find_evidence()
        }
        fn get_symbol_table(&self) -> Vec<String> {
            self.cnf.get_symbol_table()
        }
    }


    use crate::expr::Expr;
    #[pyclass(name="Expr")]
    #[derive(Clone)]
    struct PyExpr {
        expr: Expr,
    }

    impl PyExpr {
        pub fn new(expr: Expr) -> PyExpr {
            PyExpr { expr }
        }
    }

    #[pymethods]
    impl PyExpr {
        #[classattr]
        const T: PyExpr = PyExpr { expr: Expr::Literal(true) };
        #[classattr]
        const F: PyExpr = PyExpr { expr: Expr::Literal(false) };

        pub fn is_tautology(&self) -> bool {
            !Expr::not(self.expr.clone()).tseitin(false).find_evidence().is_some()
        }

        pub fn is_contradiction(&self) -> bool {
            !self.expr.clone().tseitin(false).find_evidence().is_some()
        }

        pub fn is_logically_eq(&self, other: &PyExpr) -> bool {
            !Expr::not(Expr::iff(self.expr.clone(), other.expr.clone()))
                .tseitin(false)
                .find_evidence()
                .is_some()
        }

        #[staticmethod]
        #[pyo3(name="And")]
        fn and(l: Bound<'_, PyExpr>, r: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let l= l.extract::<PyExpr>()?.expr;
            let r = r.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::and(l, r)))
        }

        #[staticmethod]
        #[pyo3(name="Or")]
        fn or(l: Bound<'_, PyExpr>, r: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let l= l.extract::<PyExpr>()?.expr;
            let r = r.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::or(l, r)))
        }

        #[staticmethod]
        #[pyo3(name="Not")]
        fn not(subexpr: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let subexpr = subexpr.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::not(subexpr)))
        }

        #[staticmethod]
        #[pyo3(name="Var")]
        fn var(name: String) -> PyResult<PyExpr> {
            Ok(PyExpr::new(Expr::Var(name)))
        }

        #[staticmethod]
        #[pyo3(name="If")]
        fn eif(l: Bound<'_, PyExpr>, r: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let l= l.extract::<PyExpr>()?.expr;
            let r = r.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::eif(l, r)))
        }

        #[staticmethod]
        #[pyo3(name="Iff")]
        fn iff(l: Bound<'_, PyExpr>, r: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let l= l.extract::<PyExpr>()?.expr;
            let r = r.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::iff(l, r)))
        }

        #[staticmethod]
        #[pyo3(name="xor")]
        fn xor(l: Bound<'_, PyExpr>, r: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let l= l.extract::<PyExpr>()?.expr;
            let r = r.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::xor(l, r)))
        }

        #[staticmethod]
        #[pyo3(name="nand")]
        fn nand(l: Bound<'_, PyExpr>, r: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let l= l.extract::<PyExpr>()?.expr;
            let r = r.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::nand(l, r)))
        }

        #[staticmethod]
        #[pyo3(name="nor")]
        fn nor(l: Bound<'_, PyExpr>, r: Bound<'_, PyExpr>) -> PyResult<PyExpr> {
            let l= l.extract::<PyExpr>()?.expr;
            let r = r.extract::<PyExpr>()?.expr;
            Ok(PyExpr::new(Expr::nor(l, r)))
        }

        #[staticmethod]
        fn parse(src: String) -> PyResult<PyExpr> {
            let expr = Expr::parse(src.as_bytes()).map_err(|msg| PyException::new_err(msg))?;
            Ok(PyExpr::new(expr))
        }

        fn tseitin(&self) -> PyCNF {
            PyCNF::new(self.expr.tseitin(false))
        }

        fn display_tseitin(&self) -> PyCNF {
            PyCNF::new(self.expr.tseitin(true))
        }
    }

    use crate::runtime::{vm::VM, env::Env};
    #[pyclass(name="Engine")]
    pub struct PyEngine {
        env: Env,
    }   

    #[pymethods]
    impl PyEngine {
        #[new]
        pub fn new() -> PyEngine {
            PyEngine { env: Env::new() }
        }

        fn define(&mut self, name: String, val: bool) {
            self.env.define(name, val);
        }

        fn undefine(&mut self, name: String) {
            self.env.undefine(&name);
        }

        fn eval(&mut self, expr: Bound<'_, PyExpr>) -> PyResult<bool> {
            let expr = expr.extract::<PyExpr>().unwrap();
            let mut vm = VM::new(&mut self.env, expr.expr.compile());

            vm.eval().map_err(|msg| PyException::new_err(msg))
        }
    }

    #[pyfunction]
    fn display_truth_table(prop: String) -> PyResult<()> {
        Expr::truth_table(prop)
            .map_err(|msg| PyException::new_err(msg))
    }

    #[pyfunction]
    fn is_valid_argument(premises: Vec<PyExpr>, conclusion: PyExpr) -> bool {
        let premises_conjunction = premises.into_iter()
            .map(|pyexpr| pyexpr.expr.clone())
            .reduce(|acc, expr| Expr::and(acc, expr))
            .unwrap_or(Expr::Literal(true));

        PyExpr::new(Expr::eif(premises_conjunction, conclusion.expr)).is_tautology()
    }

}
