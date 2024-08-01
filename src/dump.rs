use crate::value::Value;
use cairo_lang_sierra::{ids::VarId, program::StatementIdx};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use serde::{ser::SerializeMap, Serialize};
use std::collections::BTreeMap;

pub struct ProgramTrace {
    states: Vec<StateDump>,
}

impl ProgramTrace {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }
}

pub struct StateDump {
    statement_idx: StatementIdx,
    items: BTreeMap<u64, Value>,
}

impl StateDump {
    fn new(statement_idx: StatementIdx, state: OrderedHashMap<VarId, Value>) -> Self {
        Self {
            statement_idx,
            items: state
                .into_iter()
                .map(|(id, value)| (id.id, value))
                .collect(),
        }
    }
}

impl Serialize for StateDump {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = s.serialize_map(Some(self.items.len()))?;
        self.items
            .iter()
            .try_for_each(|(id, value)| s.serialize_entry(id, value))?;
        s.end()
    }
}
