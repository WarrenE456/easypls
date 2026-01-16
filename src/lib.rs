#[cfg(test)]
mod tests;

mod expr;
mod cnf;
mod lexer;
mod parser;

// Python API
use pyo3::prelude::*;
#[pymodule]
mod easypls {
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
            self.cnf.clone().is_sat()
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

        fn tseitin(&self) -> PyCNF {
            PyCNF::new(self.expr.tseitin())
        }
    }
}
