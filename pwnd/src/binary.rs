pub mod elf;

use crate::binary::elf::ELF;

pub fn elf(file_name: &str) -> ELF {
    let mut elf = ELF::new(file_name);
    elf.parse();
    elf
}