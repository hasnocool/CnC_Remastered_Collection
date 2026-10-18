// Mission
use cnc_core::ClassId;
use std::collections::HashMap;

pub struct Mission { pub name: String, pub triggers: Vec<Trigger>, pub scripts: Vec<Script> }

pub struct Trigger { pub id: u32, pub condition: String, pub action: String }

pub struct Script { pub name: String, pub actions: Vec<String> }

impl Mission { pub fn new(name: String) -> Self { Self { name, triggers: Vec::new(), scripts: Vec::new() } } }
