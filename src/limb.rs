// Copyright (C) 2020 Arron Speake
// This is a fork of a project licensed under the following:
/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use std::{
    collections::HashMap,
};

use serde_json as json;

#[derive(Debug)]
pub enum Error {
    BrokenLimb,
    InvalidValue,
    InvalidOperation,
    WriteFailed,
    ReadFailed,
    Timeout,
}

impl Into<&'static str> for Error {
    fn into(self) -> &'static str {
        use Error::*;
        match self {
            BrokenLimb => "Broken limb",
            InvalidValue => "Invalid value",
            InvalidOperation => "Invalid operation",
            WriteFailed => "Write failed",
            ReadFailed => "Read failed",
            Timeout => "Timeout",
        }
    }
}

pub trait Limb: Send + Sync {
    fn from_json(config: &json::Value) -> Option<Self>
    where
        Self: Sized;
    fn set(&mut self, value: String) -> Result<(), Error>;
    fn get(&mut self) -> Result<String, Error>;
    fn type_name(&self) -> &'static str;
}

type LimbTypesHashMapKey = Box<dyn Fn(&serde_json::Value) -> Option<Box<dyn Limb>>>;
type LimbTypesHashMap = HashMap<String, LimbTypesHashMapKey>;

pub struct LimbTypes(LimbTypesHashMap);

pub struct LimbBindings(HashMap<String, Box<dyn Limb>>);

impl LimbBindings {
    pub fn new() -> Self {
        LimbBindings(HashMap::new())
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Box<dyn Limb>> {
        self.0.iter()
    }

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

    pub fn clear(&mut self) {
        self.0 = HashMap::new();
    }
}

impl LimbTypes {
    pub fn from(h: LimbTypesHashMap) -> Self {
        LimbTypes(h)
    }

    pub fn names(&self) -> std::collections::hash_map::Keys<String, LimbTypesHashMapKey> {
        self.0.keys()
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
