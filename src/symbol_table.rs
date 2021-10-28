use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let table = HashMap::new();
        let mut st = SymbolTable { table };
        st.add_predefined();
        st
    }

    fn add_predefined(&mut self) {
        self.add_entry("SP".to_string(), 0);
        self.add_entry("LCL".to_string(), 1);
        self.add_entry("ARG".to_string(), 2);
        self.add_entry("THIS".to_string(), 3);
        self.add_entry("THAT".to_string(), 4);
        self.add_entry("SCREEN".to_string(), 16384);
        self.add_entry("KBD".to_string(), 24576);
        for i in 0..=15 {
            self.add_entry(format!("R{}", i), i);
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> u16 {
        *self.table.get(symbol).unwrap()
    }
}
