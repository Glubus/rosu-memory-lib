

/// Macro pour créer une structure Python avec des champs simples
/// 
/// # Syntax
/// ```rust
/// py_struct! {
///     #[pyclass]
///     pub struct StructName {
///         field1: Type1,
///         field2: Type2,
///     }
/// }
/// ```
/// 
/// # Generated
/// - Structure avec #[pyclass] et #[derive(Debug, Clone)]
/// - Implémentation automatique des getters
/// - Implémentation From automatique
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

/// Macro pour créer une structure Python avec des champs numériques (pas de clone)
/// 
/// # Syntax
/// ```rust
/// py_struct_numeric! {
///     #[pyclass]
///     pub struct StructName {
///         field1: i32,
///         field2: f64,
///     }
/// }
/// ```
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

/// Macro pour générer seulement les getters pour une structure existante
/// 
/// # Syntax
/// ```rust
/// py_getters! {
///     impl StructName {
///         field1: Type1,
///         field2: Type2,
///     }
/// }
/// ```
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

/// Macro pour créer une structure Python principale (avec des champs complexes)
/// 
/// # Syntax
/// ```rust
/// py_struct_main! {
///     #[pyclass]
///     pub struct StructName {
///         field1: ComplexType1,
///         field2: ComplexType2,
///     }
/// }
/// ```
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

/// Macro pour implémenter From automatiquement avec mapping direct
/// 
/// # Syntax
/// ```rust
/// py_from_impl_direct! {
///     PyStructName => RustStructName {
///         field1,
///         field2,
///     }
/// }
/// ```
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

/// Macro pour créer une fonction Python qui wrapper une fonction Rust
/// 
/// # Syntax
/// ```rust
/// py_function! {
///     pub fn function_name(process: &PyProcess, state: &mut PyState) -> PyResult<ReturnType> {
///         rust_function(&process.0, &mut state.0)
///     }
/// }
/// ```
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


