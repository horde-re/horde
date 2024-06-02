use std::fmt::Display;
use std::path::PathBuf;

use crate::loader::section::{Section, SectionType};
use crate::loader::symbol::Symbol;

/// Binary type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryType {
    Auto,
    Elf,
    Pe,
}

impl Default for BinaryType {
    fn default() -> Self {
        Self::Auto
    }
}

impl Display for BinaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto-detect binary type"),
            Self::Elf => write!(f, "Executable and Linkable Format (ELF)"),
            Self::Pe => write!(f, "Portable Executable (PE)"),
        }
    }
}

/// Binary architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryArch {
    Unknown,
    X86,
}

impl Default for BinaryArch {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for BinaryArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown architecture"),
            Self::X86 => write!(f, "x86"),
        }
    }
}

/// A binary file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary {
    pub path: PathBuf,
    pub binary_type: BinaryType,
    pub binary_arch: BinaryArch,
    pub bits: u8,
    pub entry: u64,
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
}

impl Default for Binary {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            binary_type: BinaryType::default(),
            binary_arch: BinaryArch::default(),
            bits: 64,
            entry: 0,
            sections: Vec::new(),
            symbols: Vec::new(),
        }
    }
}

impl Binary {
    pub fn new(
        path: PathBuf,
        binary_type: BinaryType,
        binary_arch: BinaryArch,
        bits: u8,
        entry: u64,
    ) -> Self {
        let sections: Vec<Section> = Vec::new();
        let symbols: Vec<Symbol> = Vec::new();

        Self {
            path,
            binary_type,
            binary_arch,
            bits,
            entry,
            sections,
            symbols,
        }
    }

    /// Get a section by its name.
    pub fn get_section_by_name(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|section| section.name == name)
    }

    /// Get a section by its type.
    pub fn get_section_by_type(&self, section_type: SectionType) -> Vec<&Section> {
        self.sections
            .iter()
            .filter(|section| section.section_type == section_type)
            .collect()
    }

    /// Get a section by its address.
    pub fn get_section_by_address(&self, address: u64) -> Option<&Section> {
        self.sections
            .iter()
            .find(|section| section.address == address)
    }

    /// Get the section that contains the given address.
    pub fn get_section_of_address(&self, address: u64) -> Option<&Section> {
        self.sections
            .iter()
            .find(|section| section.contains(address))
    }

    /// Get the text section of the binary file.
    pub fn get_text_section(&self) -> Option<&Section> {
        self.get_section_by_name(".text")
    }
}
