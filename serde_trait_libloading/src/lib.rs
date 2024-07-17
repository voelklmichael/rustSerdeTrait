mod error;
pub use error::Error;

pub const PREFIX: &str = "serde_trait_register_";
#[derive(Debug)]
pub struct Registered {
    pub library_path: String,
    pub symbol: String,
}

pub fn register_via_library<S: AsRef<str>>(lib_path: S) -> (Vec<Registered>, Result<(), Error>) {
    register_via_library_internal(lib_path.as_ref())
}
fn register_via_library_internal(lib_path: &str) -> (Vec<Registered>, Result<(), Error>) {
    let register_symbols = match extract_symbols(lib_path) {
        Ok(register_symbols) => register_symbols,
        Err(e) => return (Default::default(), Err(e)),
    };
    register(lib_path, register_symbols)
}
fn register(lib_path: &str, register_symbols: Vec<String>) -> (Vec<Registered>, Result<(), Error>) {
    let mut registered = Vec::new();
    unsafe {
        let lib = match libloading::Library::new(lib_path) {
            Ok(lib) => lib,
            Err(e) => {
                return (
                    registered,
                    Err(Error::LibLoading {
                        error: e,
                        lib_path: lib_path.to_string(),
                    }),
                )
            }
        };
        for symbol in register_symbols.iter().filter(|x| x.starts_with(PREFIX)) {
            let register: libloading::Symbol<unsafe extern "C" fn()> =
                match lib.get(symbol.as_bytes()) {
                    Ok(register) => register,
                    Err(e) => {
                        return (
                            registered,
                            Err(Error::SymbolFetch {
                                error: e,
                                lib_path: lib_path.to_string(),
                                symbol: symbol.to_string(),
                            }),
                        )
                    }
                };
            register();
            registered.push(Registered {
                library_path: lib_path.into(),
                symbol: symbol.into(),
            })
        }
    };
    (registered, Ok(()))
}

fn extract_symbols(lib_path: &str) -> Result<Vec<String>, Error> {
    use elf::{endian::AnyEndian, ElfBytes};
    use std::{fs::read, path::PathBuf};
    let found = {
        let file_data = read(PathBuf::from(lib_path)).map_err(|e| Error::ReadFile {
            error: e,
            lib_path: lib_path.to_string(),
        })?;
        let file = ElfBytes::<AnyEndian>::minimal_parse(file_data.as_slice()).map_err(|e| {
            Error::ParseElf {
                error: e,
                lib_path: lib_path.to_string(),
            }
        })?;
        let dynamics = file
            .dynamic_symbol_table()
            .map_err(|e| Error::DynamicSymbolTable {
                error: e,
                lib_path: lib_path.to_string(),
            })?;
        let mut found = Vec::new();
        if let Some((_, strings)) = dynamics {
            let mut offset = 1;
            loop {
                match strings.get(offset) {
                    Ok(x) => {
                        offset += x.len() + 1;
                        found.push(x.to_string());
                    }
                    Err(elf::ParseError::StringTableMissingNul(_)) => break,
                    Err(e) => {
                        return Err(Error::SymbolParse {
                            error: e,
                            lib_path: lib_path.to_string(),
                            symbols: found,
                            offset,
                        })
                    }
                }
            }
        }
        found
    };
    Ok(found)
}
