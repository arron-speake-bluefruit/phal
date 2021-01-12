use crate::{
    limb::{Error, Limb},
};
use serde_json as json;

pub struct XModem;

impl Limb for XModem {
    fn from_json(_config: &json::Value) -> Option<Self> {
        unimplemented!()
    }

    fn set(&mut self, _value: String) -> Result<(), Error> {
        unimplemented!()
    }

    fn get(&mut self) -> Result<String, Error> {
        unimplemented!()
    }

    fn type_name(&self) -> &'static str {
        unimplemented!()
    }
}
