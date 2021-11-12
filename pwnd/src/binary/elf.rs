use goblin::{
    elf::{
        Elf,
    },
};
use std::path::Path;
use std::fs;
use std::collections::HashMap;

pub struct ELF {
    file_name: String,
    got: HashMap<String, u64>,
    symbols: HashMap<String, u64>
}

impl ELF {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            got: HashMap::new(),
            symbols: HashMap::new()
        }
    }

    pub fn parse(&mut self) {
        let file_path = Path::new(&self.file_name);
        let buffer = fs::read(file_path).unwrap();
        let binary = match Elf::parse(&buffer) {
            Ok(elf) => elf,
            Err(_) => panic!("Not an elf file.!")
        };

        for i in 0..binary.pltrelocs.len() {
            let plt = binary.pltrelocs.get(i).unwrap();
            let sym = binary.dynsyms.get(plt.r_sym).unwrap();
            let sym_name = binary.dynstrtab.get_at(sym.st_name).unwrap();
            self.got.insert(sym_name.to_string(), plt.r_offset);
        }

        for i in 0..binary.syms.len() {
            let sym = binary.syms.get(i).unwrap();
            let sym_name = binary.strtab.get_at(sym.st_name).unwrap();
            if !sym.is_function() {
                continue;
            }
            self.symbols.insert(sym_name.to_string(), sym.st_value);
        }
    }

    pub fn got(&self, name: &str) -> Option<&u64> {
        self.got.get(name)
    }

    pub fn symbols(&self, name: &str) -> Option<&u64> {
        self.symbols.get(name)
    }
}