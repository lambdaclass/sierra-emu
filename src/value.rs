use cairo_lang_sierra::extensions::core::CoreTypeConcrete;
use starknet_types_core::felt::Felt;

#[derive(Clone, Debug)]
pub enum Value {
    Felt(Felt),
}

impl Value {
    pub fn is(&self, ty: &CoreTypeConcrete) -> bool {
        todo!()
    }
}
