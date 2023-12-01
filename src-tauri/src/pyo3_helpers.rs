use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub(crate) fn call_kiara_function<D>(python_code: &str) -> Result<D, String>
where
    for<'p> D: FromPyObject<'p>,
{
    Ok(Python::with_gil(|py| {
        let kiara_import = py.import("kiara.api")?;
        let kiara_api = kiara_import.getattr("KiaraAPI")?;
        let kiara = kiara_api.call_method0("instance")?;
        let locals = [("kiara", kiara)].into_py_dict(py);
        py.run(python_code, None, Some(locals))?;
        let response = locals.get_item("response").unwrap().unwrap();
        response.extract()
    })
    .unwrap())
}
