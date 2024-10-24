use std::fmt::Display;

/// Symbol type and symbol structure for binary files.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum SymbolType {
    #[default]
    Unknown,
    Function,
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
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Symbol {
    pub name: String,
    pub address: u64,
    pub symbol_type: SymbolType,
    pub library: Option<String>,
}

impl Symbol {
    /// Create a new symbol with the given name, address, and type.
    pub fn new(
        name: String,
        address: u64,
        symbol_type: SymbolType,
        library: Option<String>,
    ) -> Self {
        Self {
            name,
            address,
            symbol_type,
            library,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = 35;
        write!(
            f,
            "{:<width$} {:<width$} 0x{:016x} {}",
            self.library.as_deref().unwrap_or("None"),
            self.name,
            self.address,
            self.symbol_type,
        )
    }
}
