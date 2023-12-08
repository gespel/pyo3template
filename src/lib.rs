use pyo3::{pyclass, pyfunction, pymethods, pymodule, PyResult, Python, wrap_pyfunction};
use pyo3::prelude::PyModule;

#[pyclass]
struct Tester {
    name: String,
    age: i64,
}
#[pymethods]
impl Tester {
    #[new]
    fn new() -> Tester {
        Tester {
            name: "Sten".to_string(),
            age: 25
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_age(&self) -> i64 {
        self.age
    }
    fn set_age(&mut self, age: i64) {
        self.age = age;
    }
}


#[pyfunction]
/// Formats the sum of two numbers as string.
fn add(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
/// A Python module implemented in Rust.
fn pyo3template(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(add))?;
    m.add_class::<Tester>()?;

    Ok(())
}