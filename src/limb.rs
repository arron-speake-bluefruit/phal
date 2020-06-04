/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use std::collections::HashMap;

use serde_json as json;

#[derive(Debug)]
pub enum Error {
    BrokenLimb,
    InvalidValue,
    InvalidOperation,
}

pub trait Limb: Send + Sync {
    fn from_json(config: &json::Value) -> Option<Self>
    where
        Self: Sized;
    fn set(&mut self, value: String) -> Result<(), Error>;
    fn get(&mut self) -> Result<String, Error>;
}

pub struct LimbTypes(HashMap<String, Box<dyn Fn(&json::Value) -> Option<Box<dyn Limb>>>>);

pub struct LimbBindings(HashMap<String, Box<dyn Limb>>);

impl LimbBindings {
    pub fn from_json(json: &str, types: &LimbTypes) -> Option<Self> {
        let mut limbs = HashMap::new();
        match json::from_str(json).ok()? {
            json::Value::Object(o) => {
                for (k, v) in o.iter() {
                    let mut limb = match &v["type"] {
                        json::Value::String(s) => types.0[s](v),
                        _ => None,
                    }?;
                    if let json::Value::String(init_value) = &v["init"] {
                        limb.set(init_value.to_string()).ok()?;
                    }
                    limbs.insert(String::from(k), limb);
                }
            }
            _ => return None,
        }
        Some(LimbBindings(limbs))
    }

    pub fn get(&mut self, name: &str) -> Option<&mut Box<dyn Limb>> {
        self.0.get_mut(name)
    }
}

impl LimbTypes {
    pub fn from(
        h: HashMap<String, Box<dyn Fn(&serde_json::Value) -> Option<Box<dyn Limb>>>>,
    ) -> Self {
        LimbTypes(h)
    }
}

#[macro_export]
macro_rules! limb_types {
	( $( ($x:expr, $y:ty) ), * ) => {
		{
			let mut types: HashMap<String, Box<dyn Fn(&serde_json::Value) -> Option<Box<dyn Limb>>>> = HashMap::new();
			$(
				types.insert(String::from($x), Box::new(|v| <$y>::from_json(v).map(|l| {
					let limb: Box<dyn Limb> = Box::new(l);
					limb
				})));
			)*
				LimbTypes::from(types)
		}
	};
}
