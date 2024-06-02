pub mod binary;
pub mod section;
pub mod symbol;

use crate::loader::binary::{Binary, BinaryArch, BinaryType};
use crate::loader::section::{Section, SectionType};
use crate::loader::symbol::Symbol;
use goblin::{error, Object};
use std::path::PathBuf;

fn load_pe(buffer: &Vec<u8>) -> anyhow::Result<Binary> {
    let pe = goblin::pe::PE::parse(&buffer)?;

    let mut bin = Binary::default();

    bin.binary_arch = BinaryArch::X86;
    bin.bits = if pe.is_64 { 64 } else { 32 };
    bin.entry = pe.entry as u64;

    // Load sections
    for section in &pe.sections {
        let name = String::from(section.name().unwrap_or("Unknown name"));
        let address = section.virtual_address as u64;
        let size = section.virtual_size as u64;
        let content = buffer[section.pointer_to_raw_data as usize
            ..(section.pointer_to_raw_data + section.virtual_size) as usize]
            .to_vec();
        let section_type = SectionType::Unknown;

        bin.sections
            .push(Section::new(name, address, size, content, section_type));
    }

    // Load symbols
    todo!();

    Ok(bin)
}

fn load_elf(buffer: &Vec<u8>) -> anyhow::Result<Binary> {
    todo!();
}

/// Load the binary file.
pub fn load(path: PathBuf, binary_type: BinaryType) -> anyhow::Result<Binary> {
    let buffer = std::fs::read(&path)?;

    // Auto detect binary type if not specified
    let bin_type = if binary_type == BinaryType::Auto {
        match Object::parse(&buffer)? {
            Object::Elf(_) => BinaryType::Elf,
            Object::PE(_) => BinaryType::Pe,
            Object::Unknown(_) | _ => {
                return Err(error::Error::Malformed("Unsupported binary format".to_string()).into())
            }
        }
    } else {
        binary_type
    };

    match bin_type {
        BinaryType::Elf => load_elf(&buffer),
        BinaryType::Pe => load_pe(&buffer),
        BinaryType::Auto => {
            unreachable!("Binary type should be auto-detected, or explicitly specified")
        }
    }
}
