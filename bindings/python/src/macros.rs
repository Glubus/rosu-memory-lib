use pyo3::prelude::*;

#[macro_export]
macro_rules! py_struct {
    (
        #[pyclass]
        pub struct $struct_name:ident {
            $(
                $field_name:ident: $field_type:ty,
            )*
        }
    ) => {
        #[pyclass]
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(
                pub $field_name: $field_type,
            )*
        }

        #[pymethods]
        impl $struct_name {
            $(
                #[getter]
                fn $field_name(&self) -> PyResult<$field_type> {
                    Ok(self.$field_name.clone())
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! py_struct_numeric {
    (
        #[pyclass]
        pub struct $struct_name:ident {
            $(
                $field_name:ident: $field_type:ty,
            )*
        }
    ) => {
        #[pyclass]
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(
                pub $field_name: $field_type,
            )*
        }

        #[pymethods]
        impl $struct_name {
            $(
                #[getter]
                fn $field_name(&self) -> PyResult<$field_type> {
                    Ok(self.$field_name)
                }
            )*
        }
    };
}
#[macro_export]
macro_rules! py_getters {
    (
        impl $struct_name:ident {
            $(
                $field_name:ident: $field_type:ty,
            )*
        }
    ) => {
        #[pymethods]
        impl $struct_name {
            $(
                #[getter]
                fn $field_name(&self) -> PyResult<$field_type> {
                    Ok(self.$field_name.clone())
                }
            )*
        }
    };
}


#[macro_export]
macro_rules! py_struct_main {
    (
        #[pyclass]
        pub struct $struct_name:ident {
            $(
                $field_name:ident: $field_type:ty,
            )*
        }
    ) => {
        #[pyclass]
        #[derive(Debug)]
        pub struct $struct_name {
            $(
                pub $field_name: $field_type,
            )*
        }

        #[pymethods]
        impl $struct_name {
            $(
                #[getter]
                fn $field_name(&self) -> PyResult<$field_type> {
                    Ok(self.$field_name.clone())
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! py_from_impl_direct {
    (
        $py_struct:ident => $rust_struct:ident {
            $(
                $field_name:ident,
            )*
        }
    ) => {
        impl From<$rust_struct> for $py_struct {
            fn from(source: $rust_struct) -> Self {
                $py_struct {
                    $(
                        $field_name: source.$field_name,
                    )*
                }
            }
        }
    };
}


#[macro_export]
macro_rules! py_function {
    (
        pub fn $fn_name:ident($process:ident: &PyProcess, $state:ident: &mut PyState) -> PyResult<$ret_type:ty> {
            $rust_fn:ident($($args:tt)*)
        }
    ) => {
        #[pyfunction]
        pub fn $fn_name($process: &PyProcess, $state: &mut PyState) -> PyResult<$ret_type> {
            match $rust_fn($($args)*) {
                Ok(result) => Ok(result),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())),
            }
        }
    };
}


