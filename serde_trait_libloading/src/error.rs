#[derive(Debug)]
pub enum Error {
    ReadFile {
        error: std::io::Error,
        lib_path: String,
    },
    ParseElf {
        error: elf::ParseError,
        lib_path: String,
    },
    DynamicSymbolTable {
        error: elf::ParseError,
        lib_path: String,
    },
    SymbolParse {
        error: elf::ParseError,
        lib_path: String,
        symbols: Vec<String>,
        offset: usize,
    },
    LibLoading {
        error: libloading::Error,
        lib_path: String,
    },
    SymbolFetch {
        error: libloading::Error,
        lib_path: String,
        symbol: String,
    },
}
