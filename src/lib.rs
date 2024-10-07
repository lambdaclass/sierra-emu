use cairo_lang_sierra::program::{GenFunction, Program, StatementIdx};

pub use self::{dump::*, value::*, vm::VirtualMachine};

mod debug;
mod dump;
mod gas;
pub mod starknet;
mod test_utils;
mod value;
mod vm;

pub fn find_entry_point_by_idx(
    program: &Program,
    entry_point_idx: usize,
) -> Option<&GenFunction<StatementIdx>> {
    program
        .funcs
        .iter()
        .find(|x| x.id.id == entry_point_idx as u64)
}

pub fn find_entry_point_by_name<'a>(
    program: &'a Program,
    name: &str,
) -> Option<&'a GenFunction<StatementIdx>> {
    program
        .funcs
        .iter()
        .find(|x| x.id.debug_name.as_ref().map(|x| x.as_str()) == Some(name))
}
