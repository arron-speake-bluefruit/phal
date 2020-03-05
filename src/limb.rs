/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use std::{collections::HashMap, sync::Mutex};

use serde_json as json;

#[derive(Debug)]
pub enum Error {
    BrokenLimb,
    InvalidValue,
    InvalidOperation,
    MissingLimb,
}

pub trait Limb: Send + Sync {
    fn from_json(config: &json::Value) -> Option<Self>
    where
        Self: Sized;
    fn set(&mut self, value: String) -> Result<(), Error>;
    fn get(&mut self) -> Result<String, Error>;
}

pub struct LimbTypes(HashMap<String, Box<dyn Fn(&json::Value) -> Option<Box<Mutex<dyn Limb>>>>>);

pub struct LimbBindings(HashMap<String, Box<Mutex<dyn Limb>>>);

impl LimbBindings {
    pub fn from_json(json: String, types: LimbTypes) -> Option<Self> {
        let mut limbs = HashMap::new();
        match json::from_str(&json).ok()? {
            json::Value::Object(o) => {
                for (k, v) in o.iter() {
                    let limb = match &v["type"] {
                        json::Value::String(s) => types.0[s](v),
                        _ => None,
                    }?;
                    limbs.insert(String::from(k), limb);
                }
            }
            _ => return None,
        }
        Some(LimbBindings(limbs))
    }

    pub fn get(&self, name: &str) -> Option<&Box<Mutex<dyn Limb>>> {
        self.0.get(name)
    }
}

impl LimbTypes {
    pub fn from(
        h: HashMap<String, Box<dyn Fn(&serde_json::Value) -> Option<Box<Mutex<dyn Limb>>>>>,
    ) -> Self {
        LimbTypes(h)
    }
}

#[macro_export]
macro_rules! limb_types {
	( $( ($x:expr, $y:ty) ), * ) => {
		{
			let mut types: HashMap<String, Box<dyn Fn(&serde_json::Value) -> Option<Box<Mutex<dyn Limb>>>>> = HashMap::new();
			$(
				types.insert(String::from($x), Box::new(|v| <$y>::from_json(v).map(|l| {
					let limb: Box<Mutex<dyn Limb>> = Box::new(Mutex::new(l));
					limb
				})));
			)*
				LimbTypes::from(types)
		}
	};
}
