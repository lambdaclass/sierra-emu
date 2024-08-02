use crate::value::Value;
use cairo_lang_sierra::{ids::VarId, program::StatementIdx};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use serde::{ser::SerializeMap, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ProgramTrace<'a> {
    states: Vec<StateDump<'a>>,
    // TODO: Syscall data.
}

impl<'a> ProgramTrace<'a> {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    pub fn push(&mut self, state: StateDump<'a>) {
        self.states.push(state);
    }
}

#[derive(Clone, Debug)]
pub struct StateDump<'a> {
    statement_idx: StatementIdx,
    items: BTreeMap<u64, Value<'a>>,
}

impl<'a> StateDump<'a> {
    pub fn new(statement_idx: StatementIdx, state: OrderedHashMap<VarId, Value<'a>>) -> Self {
        Self {
            statement_idx,
            items: state
                .into_iter()
                .map(|(id, value)| (id.id, value))
                .collect(),
        }
    }
}

impl<'a> Serialize for StateDump<'a> {
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
