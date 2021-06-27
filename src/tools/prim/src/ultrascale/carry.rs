use crate::{Param, ParamSet, PortSet, ToPrim};
use derive_more::{Display, From};

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum CarryType {
    #[display(fmt = "SINGLE_CY8")]
    Dual,
    #[display(fmt = "DUAL_CY4")]
    Single,
}

#[derive(Clone, Debug, From, Eq, Display)]
pub enum CarryParam {
    CarryType(CarryType),
}

#[derive(Clone, Debug, Default)]
pub struct Carry;

impl PartialEq for CarryParam {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Default for CarryParam {
    fn default() -> Self {
        CarryParam::from(CarryType::Dual)
    }
}

impl ToPrim<CarryParam> for Carry {
    fn to_name(&self) -> String {
        String::from("CARRY8")
    }
    fn to_param(&self) -> ParamSet<CarryParam> {
        let mut param = ParamSet::new();
        let ty = Param {
            name: "CARRY_TYPE".to_string(),
            width: None,
            value: CarryType::Single.into(),
        };
        param.insert(ty);
        param
    }
    fn to_input(&self) -> PortSet {
        PortSet::new()
    }
    fn to_output(&self) -> PortSet {
        PortSet::new()
    }
}
