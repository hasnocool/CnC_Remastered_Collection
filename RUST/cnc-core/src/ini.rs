//! Westwood INI file parser with inheritance support

use anyhow::{Result, Context, bail};
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IniValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<IniValue>),
}

impl IniValue {
    pub fn as_string(&self) -> Option<&str> {
        match self { IniValue::String(s) => Some(s), _ => None }
    }
    pub fn as_str(&self) -> Result<&str> { self.as_string().context("Value is not a string") }
    pub fn as_i64(&self) -> Result<i64> {
        match self {
            IniValue::Integer(i) => Ok(*i),
            IniValue::Float(f) => Ok(*f as i64),
            IniValue::Boolean(b) => Ok(i64::from(*b)),
            IniValue::String(s) => Self::parse_integer(s).context("Failed to parse integer"),
            _ => bail!("Value is not an integer"),
        }
    }
    pub fn as_f64(&self) -> Result<f64> {
        match self {
            IniValue::Float(f) => Ok(*f),
            IniValue::Integer(i) => Ok(*i as f64),
            IniValue::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            IniValue::String(s) => s.parse().context("Failed to parse float"),
            _ => bail!("Value is not a float"),
        }
    }
    /// Parse decimal or Westwood-style hexadecimal (0x-prefixed) integers.
    pub fn parse_integer(value: &str) -> Option<i64> {
        let value = value.trim();
        let (negative, digits) = match value.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, value),
        };
        let parsed = if let Some(hex) = digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")) {
            i64::from_str_radix(hex, 16).ok()?
        } else {
            digits.parse::<i64>().ok()?
        };
        Some(if negative { -parsed } else { parsed })
    }
    pub fn as_bool(&self) -> Result<bool> {
        match self {
            IniValue::Boolean(b) => Ok(*b),
            IniValue::Integer(i) => Ok(*i != 0),
            IniValue::String(s) => Ok(matches!(s.to_lowercase().as_str(), "yes"|"true"|"1"|"on")),
            _ => bail!("Value is not a boolean"),
        }
    }
    pub fn as_array(&self) -> Result<&[IniValue]> {
        match self { IniValue::Array(a) => Ok(a), _ => bail!("Value is not an array") }
    }
}

impl From<String> for IniValue { fn from(s: String) -> Self { IniValue::String(s) } }
impl From<&str> for IniValue { fn from(s: &str) -> Self { IniValue::String(s.to_string()) } }
impl From<i64> for IniValue { fn from(i: i64) -> Self { IniValue::Integer(i) } }
impl From<f64> for IniValue { fn from(f: f64) -> Self { IniValue::Float(f) } }
impl From<bool> for IniValue { fn from(b: bool) -> Self { IniValue::Boolean(b) } }
impl From<Vec<IniValue>> for IniValue { fn from(v: Vec<IniValue>) -> Self { IniValue::Array(v) } }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IniSection {
    pub name: String,
    pub values: BTreeMap<String, IniValue>,
    pub parent: Option<String>,
}

impl IniSection {
    pub fn new(name: String) -> Self { Self { name, values: BTreeMap::new(), parent: None } }
    pub fn set(&mut self, key: String, value: IniValue) { self.values.insert(key, value); }
    pub fn get(&self, key: &str) -> Option<&IniValue> { self.values.get(key) }
    /// Case-insensitive lookup; Westwood INI keys are not case sensitive.
    pub fn get_ci(&self, key: &str) -> Option<&IniValue> {
        self.values.get(key).or_else(|| {
            self.values
                .iter()
                .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
                .map(|(_, value)| value)
        })
    }
    pub fn get_string(&self, key: &str) -> Option<&str> { self.values.get(key).and_then(|v| v.as_string()) }
    pub fn get_i64(&self, key: &str) -> Result<i64> { self.values.get(key).context("Key not found")?.as_i64() }
    pub fn get_f64(&self, key: &str) -> Result<f64> { self.values.get(key).context("Key not found")?.as_f64() }
    pub fn get_bool(&self, key: &str) -> Result<bool> { self.values.get(key).context("Key not found")?.as_bool() }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IniFile {
    pub sections: BTreeMap<String, IniSection>,
    pub globals: BTreeMap<String, IniValue>,
}

impl IniFile {
    pub fn new() -> Self { Self::default() }

    pub fn parse(input: &str) -> Result<Self> {
        let mut ini = Self::new();
        let mut current_section: Option<String> = None;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') { continue; }

            if line.starts_with('[') && line.ends_with(']') {
                let section_name = line[1..line.len()-1].trim().to_string();
                current_section = Some(section_name.clone());
                if let Some(colon_pos) = section_name.find(':') {
                    let name = section_name[..colon_pos].trim().to_string();
                    let parent = section_name[colon_pos+1..].trim().to_string();
                    let mut section = IniSection::new(name.clone());
                    section.parent = Some(parent);
                    ini.sections.insert(name.clone(), section);
                    current_section = Some(name);
                } else {
                    ini.sections.entry(section_name.clone()).or_insert_with(|| IniSection::new(section_name));
                }
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value_str = line[eq_pos+1..].trim();
                let value_str = value_str.split(';').next().unwrap_or("").trim();
                let value_str = value_str.split('#').next().unwrap_or("").trim();
                let value = Self::parse_value(value_str)?;

                if let Some(section_name) = &current_section {
                    if let Some(section) = ini.sections.get_mut(section_name) { section.set(key, value); }
                } else { ini.globals.insert(key, value); }
            }
        }

        ini.resolve_inheritance()?;
        Ok(ini)
    }

    fn parse_value(s: &str) -> Result<IniValue> {
        let s = s.trim();
        if s.contains(',') {
            let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
            let mut values = Vec::with_capacity(parts.len());
            for part in parts { values.push(Self::parse_single_value(part)?); }
            return Ok(IniValue::Array(values));
        }
        Self::parse_single_value(s)
    }

    fn parse_single_value(s: &str) -> Result<IniValue> {
        let s = s.trim();
        if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
            return Ok(IniValue::String(s[1..s.len()-1].to_string()));
        }
        // Numbers (including 0x-prefixed hex) win over boolean words so that
        // Rules.ini values such as "1" remain usable as integers.
        if let Some(i) = IniValue::parse_integer(s) { return Ok(IniValue::Integer(i)); }
        if let Ok(f) = s.parse::<f64>() { return Ok(IniValue::Float(f)); }
        let lower = s.to_lowercase();
        if matches!(lower.as_str(), "yes"|"true"|"on") { return Ok(IniValue::Boolean(true)); }
        if matches!(lower.as_str(), "no"|"false"|"off") { return Ok(IniValue::Boolean(false)); }
        Ok(IniValue::String(s.to_string()))
    }

    fn resolve_inheritance(&mut self) -> Result<()> {
        let section_names: Vec<String> = self.sections.keys().cloned().collect();
        for name in section_names {
            // Walk the whole ancestor chain so [C:A] inherits through [B] even when
            // the parent itself derives from [A]. Cycles are rejected explicitly.
            let mut chain = vec![name.clone()];
            let mut current = name.clone();
            while let Some(parent) = self.sections.get(&current).and_then(|s| s.parent.clone()) {
                if chain.contains(&parent) {
                    bail!("circular INI inheritance involving [{}]", name);
                }
                if !self.sections.contains_key(&parent) {
                    bail!("Parent section [{}] of [{}] not found", parent, name);
                }
                chain.push(parent.clone());
                current = parent;
            }
            // Merge nearest ancestor first so it wins over more distant ancestors.
            for ancestor in chain.iter().skip(1) {
                let inherited = self.sections[ancestor].values.clone();
                let child = self.sections.get_mut(&name).expect("child section exists");
                for (key, value) in inherited {
                    child.values.entry(key).or_insert(value);
                }
            }
        }
        Ok(())
    }

    pub fn section(&self, name: &str) -> Option<&IniSection> { self.sections.get(name) }
    pub fn section_mut(&mut self, name: &str) -> Option<&mut IniSection> { self.sections.get_mut(name) }
    pub fn section_or_create(&mut self, name: &str) -> &mut IniSection {
        self.sections.entry(name.to_string()).or_insert_with(|| IniSection::new(name.to_string()))
    }
    pub fn global(&self, key: &str) -> Option<&IniValue> { self.globals.get(key) }
    pub fn set_global(&mut self, key: String, value: IniValue) { self.globals.insert(key, value); }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        if !self.globals.is_empty() {
            for (key, value) in &self.globals { out.push_str(&format!("{}={}\n", key, Self::value_to_string(value))); }
            out.push('\n');
        }
        for (name, section) in &self.sections {
            if let Some(parent) = &section.parent { out.push_str(&format!("[{}:{}]\n", name, parent)); }
            else { out.push_str(&format!("[{}]\n", name)); }
            for (key, value) in &section.values { out.push_str(&format!("{}={}\n", key, Self::value_to_string(value))); }
            out.push('\n');
        }
        out
    }

    fn value_to_string(value: &IniValue) -> String {
        match value {
            IniValue::String(s) => if s.contains(' ') || s.contains(',') || s.contains(';') || s.contains('#') { format!("\"{}\"", s) } else { s.clone() },
            IniValue::Integer(i) => i.to_string(),
            IniValue::Float(f) => f.to_string(),
            IniValue::Boolean(b) => if *b { "yes".to_string() } else { "no".to_string() },
            IniValue::Array(arr) => arr.iter().map(Self::value_to_string).collect::<Vec<_>>().join(","),
        }
    }

    pub fn load(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).with_context(|| format!("Failed to read INI file: {:?}", path))?;
        Self::parse(&content)
    }
    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        let content = self.to_string();
        std::fs::write(path, content).with_context(|| format!("Failed to write INI file: {:?}", path))?;
        Ok(())
    }
}

impl fmt::Display for IniFile { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.to_string()) } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_values_survive_boolean_like_literals() {
        let ini = IniFile::parse("[General]\nAttackInterval=3\nAllowed=1\nBlocked=0\n").unwrap();
        let general = ini.section("General").unwrap();
        assert_eq!(general.get_i64("AttackInterval").unwrap(), 3);
        assert_eq!(general.get_i64("Allowed").unwrap(), 1);
        assert!(general.get_bool("Allowed").unwrap());
        assert_eq!(general.get_bool("Blocked").unwrap(), false);
        assert_eq!(general.get_f64("AttackInterval").unwrap(), 3.0);
    }

    #[test]
    fn parses_hex_and_negative_literals() {
        let ini = IniFile::parse("[General]\nPowerEmergencyFraction=0x00C0\nNegative=-0x10\n").unwrap();
        let section = ini.section("General").unwrap();
        assert_eq!(section.get_i64("PowerEmergencyFraction").unwrap(), 0x00C0);
        assert_eq!(section.get_i64("Negative").unwrap(), -16);
    }

    #[test]
    fn resolves_multi_level_inheritance() {
        let ini = IniFile::parse(
            "[Base]\nCost=100\nStrength=50\n\n[Mid:Base]\nStrength=80\n\n[Leaf:Mid]\nCost=200\n",
        )
        .unwrap();
        let leaf = ini.section("Leaf").unwrap();
        assert_eq!(leaf.get_i64("Cost").unwrap(), 200);
        assert_eq!(leaf.get_i64("Strength").unwrap(), 80);
        assert_eq!(leaf.parent.as_deref(), Some("Mid"));
    }

    #[test]
    fn rejects_missing_parent_and_cycles() {
        assert!(IniFile::parse("[Child:Missing]\nCost=1\n").is_err());
        assert!(IniFile::parse("[A:B]\nX=1\n[B:A]\nY=2\n").is_err());
    }

    #[test]
    fn case_insensitive_lookup_and_strings() {
        let ini = IniFile::parse("[General]\nBuildSpeed=1.5\nName=\"Hello World\"\nList=A,B,C\n").unwrap();
        let section = ini.section("General").unwrap();
        assert_eq!(section.get_ci("buildspeed").unwrap().as_f64().unwrap(), 1.5);
        assert_eq!(section.get_string("Name"), Some("Hello World"));
        let list = section.get("List").unwrap().as_array().unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[2].as_string(), Some("C"));
    }

    #[test]
    fn round_trips_sections_and_globals() {
        let ini = IniFile::parse("[Weapons]\n1=Gun\n2=M16\n[Gun]\nDamage=25\nROF=30\n").unwrap();
        let text = ini.to_string();
        let reparsed = IniFile::parse(&text).unwrap();
        assert_eq!(reparsed.section("Gun").unwrap().get_i64("Damage").unwrap(), 25);
        assert_eq!(reparsed.section("Weapons").unwrap().get("2").unwrap().as_string(), Some("M16"));
    }
}
