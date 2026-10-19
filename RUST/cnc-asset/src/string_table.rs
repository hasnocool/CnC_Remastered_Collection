// String table (.csf)
use std::collections::HashMap;

pub struct CsfFile {
    pub strings: HashMap<String, String>,
}

impl CsfFile {
    pub fn load(_data: &[u8]) -> Result<Self, String> {
        Ok(Self {
            strings: HashMap::new(),
        })
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.strings.get(key)
    }
}
