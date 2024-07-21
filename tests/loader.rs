#[cfg(test)]
mod loader_tests {
    use std::path::PathBuf;

    use horde::loader::binary::*;
    use horde::loader::load;
    use horde::loader::section::*;

    const PE_PATH: &str = "tests/bin/pe.bin";
    // const ELF_PATH: &str = "tests/bin/elf.bin";

    fn assert_basic_pe_info(bin: &Binary) {
        assert_eq!(bin.binary_arch, BinaryArch::X86);
        assert_eq!(bin.bits, 64);
        assert_eq!(bin.entry, 4900);
        assert_eq!(bin.sections.len(), 6);
        assert_eq!(bin.symbols.len(), 42);
    }

    fn assert_basic_pe_section(bin: &Binary) {
        bin.sections
            .iter()
            .for_each(|section| match section.name.as_str() {
                ".text" => {
                    assert_eq!(section.address, 0x0000000140001000);
                    assert_eq!(section.size, 0x00000db8);
                    assert_eq!(section.section_type, SectionType::Code);
                }
                ".rdata" => {
                    assert_eq!(section.address, 0x0000000140002000);
                    assert_eq!(section.size, 0x00000d72);
                    assert_eq!(section.section_type, SectionType::Data);
                }
                ".data" => {
                    assert_eq!(section.address, 0x0000000140003000);
                    assert_eq!(section.size, 0x0000638);
                    assert_eq!(section.section_type, SectionType::Data);
                }
                ".pdata" => {
                    assert_eq!(section.address, 0x0000000140004000);
                    assert_eq!(section.size, 0x00000168);
                    assert_eq!(section.section_type, SectionType::Data);
                }
                ".rsrc" => {
                    assert_eq!(section.address, 0x0000000140005000);
                    assert_eq!(section.size, 0x000001e0);
                    assert_eq!(section.section_type, SectionType::Data);
                }
                ".reloc" => {
                    assert_eq!(section.address, 0x0000000140006000);
                    assert_eq!(section.size, 0x0000001c);
                    assert_eq!(section.section_type, SectionType::Data);
                }
                _ => panic!("Unknown section name: {}", section.name),
            });
    }

    #[test]
    fn test_load_auto_pe() {
        let path = PathBuf::from(PE_PATH);
        let bin = load(path, BinaryType::Auto).expect("Failed to load PE binary");

        assert_eq!(bin.binary_type, BinaryType::Pe);
        assert_basic_pe_info(&bin);
        assert_basic_pe_section(&bin);
    }

    #[test]
    fn test_load_pe() {
        let path = PathBuf::from(PE_PATH);
        let bin = load(path, BinaryType::Pe).expect("Failed to load PE binary");

        assert_eq!(bin.binary_type, BinaryType::Pe);
        assert_basic_pe_info(&bin);
        assert_basic_pe_section(&bin);
    }

    // #[test]
    // fn test_load_auto_elf() {
    //     let path = PathBuf::from(ELF_PATH);
    //     let bin = load(path, BinaryType::Auto).expect("Failed to load PE binary");
    //     todo!()
    // }

    // #[test]
    // fn test_load_elf() {
    //     let path = PathBuf::from(ELF_PATH);
    //     let bin = load(path, BinaryType::Elf).expect("Failed to load ELF binary");
    //     todo!()
    // }
}
