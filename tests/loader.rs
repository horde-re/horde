#[cfg(test)]
mod loader_tests {
    use std::path::PathBuf;

    use horde::loader::binary::*;
    use horde::loader::load;

    const PE_PATH: &str = "tests/bin/pe.bin";
    const ELF_PATH: &str = "tests/bin/elf.bin";

    #[test]
    fn test_load_auto_pe() {
        let path = PathBuf::from(PE_PATH);
        let bin = load(path, BinaryType::Auto).expect("Failed to load PE binary");

        assert_eq!(bin.binary_type, BinaryType::Pe);
        assert_eq!(bin.binary_arch, BinaryArch::X86);
        assert_eq!(bin.bits, 64);
        assert_eq!(bin.entry, 4900);
        assert_eq!(bin.sections.len(), 6);
        assert_eq!(bin.symbols.len(), 0);
    }

    #[test]
    fn test_load_auto_elf() {
        let path = PathBuf::from(ELF_PATH);
        let bin = load(path, BinaryType::Auto).expect("Failed to load PE binary");

        assert_eq!(bin.binary_type, BinaryType::Elf);
        assert_eq!(bin.binary_arch, BinaryArch::X86);
        assert_eq!(bin.bits, 64);
        assert_eq!(bin.entry, 4900);
        assert_eq!(bin.sections.len(), 6);
        assert_eq!(bin.symbols.len(), 0);
    }

    #[test]
    fn test_load_pe() {
        let path = PathBuf::from(PE_PATH);
        let bin = load(path, BinaryType::Pe).expect("Failed to load PE binary");

        assert_eq!(bin.binary_arch, BinaryArch::X86);
        assert_eq!(bin.bits, 64);
        assert_eq!(bin.entry, 4900);
        assert_eq!(bin.sections.len(), 6);
        assert_eq!(bin.symbols.len(), 0);
    }

    #[test]
    fn test_load_elf() {
        let path = PathBuf::from(ELF_PATH);
        let bin = load(path, BinaryType::Elf).expect("Failed to load ELF binary");

        assert_eq!(bin.binary_arch, BinaryArch::X86);
        assert_eq!(bin.bits, 64);
        assert_eq!(bin.entry, 4900);
        assert_eq!(bin.sections.len(), 6);
        assert_eq!(bin.symbols.len(), 0);
    }
}
