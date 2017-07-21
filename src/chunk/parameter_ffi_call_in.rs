use analysis;
use library;

#[derive(Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: library::TypeId,
    pub instance_parameter: bool,
    pub direction: library::ParameterDirection,
    pub transfer: library::Transfer,
    pub nullable: library::Nullable,
    pub ref_mode: analysis::ref_mode::RefMode,
    pub to_glib_extra: String,
    pub is_into: bool,
}

impl Parameter {
    pub fn new(orig: &analysis::function_parameters::CParameter) -> Parameter {
        Parameter {
            name: orig.name.clone(),
            typ: orig.typ,
            instance_parameter: orig.instance_parameter,
            direction: orig.direction,
            transfer: orig.transfer,
            nullable: orig.nullable,
            ref_mode: orig.ref_mode,
            to_glib_extra: orig.to_glib_extra.clone(),
            is_into: orig.is_into,
        }
    }
}
