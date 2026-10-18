use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssetRegistry {
    files: HashMap<String, Vec<u8>>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn register(&mut self, name: String, data: Vec<u8>) {
        self.files.insert(name, data);
    }

    pub fn get(&self, name: &str) -> Option<&Vec<u8>> {
        self.files.get(name)
    }
}

impl Default for AssetRegistry {
    fn default() -> Self {
        Self::new()
    }
}
