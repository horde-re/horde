use std::fmt::Display;

/// Symbol type and symbol structure for binary files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Unknown,
    Function,
}

impl Default for SymbolType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown symbol type"),
            Self::Function => write!(f, "Function"),
        }
    }
}

/// A symbol in a binary file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub address: u64,
    pub symbol_type: SymbolType,
}

impl Symbol {
    /// Create a new symbol with the given name, address, and type.
    pub fn new(name: String, address: u64, symbol_type: SymbolType) -> Self {
        Self {
            name,
            address,
            symbol_type,
        }
    }
}
