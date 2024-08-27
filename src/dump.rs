use crate::value::Value;
use cairo_lang_sierra::{ids::VarId, program::StatementIdx};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use serde::{ser::SerializeMap, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ProgramTrace {
    pub states: Vec<StateDump>,
    // TODO: Syscall data.
}

impl ProgramTrace {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    pub fn push(&mut self, state: StateDump) {
        self.states.push(state);
    }
}

#[derive(Clone, Debug)]
pub struct StateDump {
    statement_idx: StatementIdx,
    items: BTreeMap<u64, Value>,
}

impl StateDump {
    pub fn new(statement_idx: StatementIdx, state: OrderedHashMap<VarId, Value>) -> Self {
        Self {
            statement_idx,
            items: state
                .into_iter()
                .map(|(id, value)| (id.id, value))
                .collect(),
        }
    }

    pub fn item(&self) -> BTreeMap<u64, Value> {
        self.items.clone()
    }
}

impl Serialize for StateDump {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = s.serialize_map(Some(2))?;

        s.serialize_entry("statementIdx", &self.statement_idx.0)?;
        s.serialize_entry("preStateDump", &self.items)?;

        s.end()
    }
}
