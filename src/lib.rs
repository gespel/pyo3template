use pyo3::{pyclass, pyfunction, pymethods, pymodule, PyResult, Python, wrap_pyfunction};
use pyo3::prelude::PyModule;
use serde::{Serialize, Deserialize};

#[pyclass]
#[derive(Serialize, Deserialize)]
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
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}


#[no_mangle]
pub extern fn add(a: usize, b: usize) -> usize {
    a + b
}

#[pymodule]
fn pyo3template(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_wrapped(wrap_pyfunction!(add))?;
    m.add_class::<Tester>()?;

    Ok(())
}