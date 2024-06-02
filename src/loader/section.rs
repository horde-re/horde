/// The type of a section in a binary file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Unknown,
    Code,
    Data,
}

/// A section in a binary file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub name: String,
    pub address: u64, // virtual memory address
    pub size: u64,
    pub content: Vec<u8>,
    pub section_type: SectionType,
}

impl Section {
    /// Create a new section with the given name, address, size, content, and type.
    pub fn new(
        name: String,
        address: u64,
        size: u64,
        content: Vec<u8>,
        section_type: SectionType,
    ) -> Self {
        Self {
            name,
            address,
            size,
            content,
            section_type,
        }
    }

    /// Check if the given address is within the range of this section.
    pub fn contains(&self, address: u64) -> bool {
        self.address >= address && address - self.address < self.size
    }
}
