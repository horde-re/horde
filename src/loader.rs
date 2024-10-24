pub mod binary;
pub mod section;
pub mod symbol;

use crate::loader::binary::{Binary, BinaryArch, BinaryType};
use crate::loader::section::{Section, SectionType};
use crate::loader::symbol::{Symbol, SymbolType};

use goblin::pe::section_table::IMAGE_SCN_CNT_CODE;
use goblin::{error, Object};
use std::borrow::Cow;
use std::path::PathBuf;

fn load_pe(buffer: &[u8]) -> anyhow::Result<Binary> {
    let pe = goblin::pe::PE::parse(buffer)?;

    let mut bin = Binary {
        binary_type: BinaryType::Pe,
        binary_arch: BinaryArch::X86,
        bits: if pe.is_64 { 64 } else { 32 },
        entry: pe.entry as u64,
        ..Default::default()
    };

    // Load sections
    for section in &pe.sections {
        let name = String::from(section.name().unwrap_or("Unknown name"));
        let offset = section.pointer_to_raw_data as usize;
        let address = section.virtual_address as u64 + pe.image_base as u64;
        let size = section.virtual_size as u64;

        let content = buffer[offset..(offset + size as usize)].to_vec();

        // Not the best way to determine section type, but it works for now
        let section_type = if section.characteristics & IMAGE_SCN_CNT_CODE != 0 {
            SectionType::Code
        } else {
            SectionType::Data
        };

        bin.sections
            .push(Section::new(name, address, size, content, section_type));
    }

    // Load symbols
    // 1. Load imports
    for import in pe.imports {
        // Convert Cow to String
        let symbol_name = match import.name {
            Cow::Borrowed(name) => name.to_string(),
            Cow::Owned(name) => name,
        };
        let lib_name = import.dll.to_string();
        let address = import.rva as u64 + pe.image_base as u64; // convert rva to virtual address

        // Determine symbol type by finding the section that contains it
        let symbol_type = bin
            .sections
            .iter()
            .find(|section| section.contains(address))
            .map_or(SymbolType::Unknown, |section| {
                println!(
                    "Found section {:?}, type = {:?}",
                    section.name, section.section_type
                );

                if section.section_type == SectionType::Code {
                    SymbolType::Function
                } else {
                    SymbolType::Unknown // we don't support data yet
                }
            });

        bin.symbols.push(Symbol::new(
            symbol_name,
            address,
            symbol_type,
            Some(lib_name),
        ));
    }

    // temporarily print imports
    // for symbol in &bin.symbols {
    //     println!("{}", symbol);
    // }

    // 2. Load exports

    Ok(bin)
}

#[allow(unused_variables)]
fn load_elf(buffer: &[u8]) -> anyhow::Result<Binary> {
    todo!();
}

/// Load the binary file.
/// If `binary_type` is `BinaryType::Auto`, the binary type will be auto-detected.
/// Otherwise, the binary type will be used as specified.
/// Returns a `Binary` struct.
/// # Errors
/// Returns an error if the binary format is not supported.
#[allow(clippy::wildcard_in_or_patterns)]
pub fn load(path: PathBuf, binary_type: BinaryType) -> anyhow::Result<Binary> {
    let buffer = std::fs::read(path)?;

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
