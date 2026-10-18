// House/Faction for RA
use cnc_core::{ClassId, Fixed16_16};
use std::collections::VecDeque;

pub struct House { pub id: ClassId, pub name: String, pub credits: i32, pub power: i32, pub sub_faction: Option<String> }

impl House { pub fn new(id: ClassId, name: String) -> Self { Self { id, name, credits: 0, power: 0, sub_faction: None } } }
