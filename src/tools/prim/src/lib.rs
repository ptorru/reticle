pub mod ultrascale;

use anyhow::Result;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use thiserror::Error;
// use std::fmt::Display;

#[derive(Clone, Debug, Default, Eq)]
pub struct Param<T> {
    pub name: String,
    pub width: Option<u32>,
    pub value: T,
}

#[derive(Clone, Debug, Default, Eq)]
pub struct Port {
    pub name: String,
    pub width: u32,
}

pub type ParamSet<T> = HashSet<Param<T>>;
pub type PortSet = HashSet<Port>;

// T ~> Param value type
#[derive(Clone, Debug, Default)]
pub struct Prim<T> {
    pub name: String,
    pub param: ParamSet<T>,
    pub input: PortSet,
    pub output: PortSet,
}

#[derive(Error, Debug)]
pub enum PrimError {
    #[error("Invalid Value: {0}")]
    InvalidValue(String),
    #[error("Missing parameter: {0}")]
    MissingParam(String),
}

pub trait ToPrim<T> {
    fn to_name(&self) -> String;
    fn to_param(&self) -> ParamSet<T>;
    fn to_input(&self) -> PortSet;
    fn to_output(&self) -> PortSet;
    fn to_prim(&self) -> Prim<T> {
        Prim {
            name: self.to_name(),
            param: self.to_param(),
            input: self.to_input(),
            output: self.to_output(),
        }
    }
}

impl<T> PartialEq for Param<T> {
    fn eq(&self, other: &Param<T>) -> bool {
        self.name == other.name
    }
}

impl<T> Hash for Param<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<T> Borrow<String> for Param<T> {
    fn borrow(&self) -> &String {
        &self.name
    }
}

impl<T> Borrow<str> for Param<T> {
    fn borrow(&self) -> &str {
        &self.name.as_str()
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Port) -> bool {
        self.name == other.name
    }
}

impl Hash for Port {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Borrow<String> for Port {
    fn borrow(&self) -> &String {
        &self.name
    }
}

impl Borrow<str> for Port {
    fn borrow(&self) -> &str {
        &self.name.as_str()
    }
}

impl<T> Param<T> {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn width(&self) -> Option<u32> {
        self.width
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

impl<T: Eq + Default + std::fmt::Debug> Prim<T> {
    pub fn new() -> Self {
        Prim::default()
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn param(&self) -> &ParamSet<T> {
        &self.param
    }
    pub fn input(&self) -> &PortSet {
        &self.input
    }
    pub fn output(&self) -> &PortSet {
        &self.output
    }
    pub fn set_param<U>(&mut self, name: &str, value: U) -> Result<()>
    where
        U: Into<T>,
    {
        if let Some(old) = self.param.get(name) {
            let value: T = value.into();
            if old.value() == &value {
                let param = Param {
                    name: old.name(),
                    width: old.width(),
                    value,
                };
                self.param.replace(param);
                Ok(())
            } else {
                Err(PrimError::InvalidValue(String::from("error")).into())
            }
        } else {
            Err(PrimError::MissingParam(name.into()).into())
        }
    }
}
