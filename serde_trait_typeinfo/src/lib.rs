#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TypeInfo {
    pub type_name: String,
    pub module: String,
    pub crate_name: String,
    pub crate_version: String,
    pub generic_parameters: Vec<Self>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TypeInfoWithRustc {
    pub type_info: TypeInfo,
    pub rustc_version: String,
}
