#[cfg(test)]
mod tests;

mod expr;
mod cnf;


// Python API
use pyo3::prelude::*;
#[pymodule]
mod easypls {
    use pyo3::prelude::*;

    use crate::cnf::CNF;

    #[pyclass(name = "CNF")]
    pub struct PyCNF {
        cnf: CNF
    }

    #[pymethods]
    impl PyCNF {
        fn is_sat(&self) -> bool {
            self.cnf.clone().is_sat()
        }
    }
}
