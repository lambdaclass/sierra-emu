use cairo_lang_sierra::extensions::core::CoreTypeConcrete;
use serde::Serialize;
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug, Serialize)]
pub enum Value {
    Felt(Felt),
}

impl Value {
    pub fn is(&self, ty: &CoreTypeConcrete) -> bool {
        todo!()
    }
}
