/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use std::{collections::HashMap, sync::Mutex};

#[derive(Debug)]
pub enum Error {
    Io,
    BrokenLimb,
    InvalidValue,
    InvalidOperation,
    MissingLimb,
}

pub trait Limb: Send + Sync {
    fn set(&mut self, value: String) -> Result<(), Error>;
    fn get(&mut self) -> Result<String, Error>;
}

pub struct LimbBindings(HashMap<String, Box<Mutex<dyn Limb>>>);

impl LimbBindings {
    pub fn from(limbs: HashMap<String, Box<Mutex<dyn Limb>>>) -> Self {
        LimbBindings(limbs)
    }

    pub fn get(&self, name: &str) -> Option<&Box<Mutex<dyn Limb>>> {
        self.0.get(name)
    }
}

#[macro_export]
macro_rules! limbs {
    ( $( ($x:expr, $y:expr) ), * ) => {
	{
	    let mut limbs: HashMap<String, Box<Mutex<dyn Limb>>> = HashMap::new();
	    $(
		limbs.insert(String::from($x), Box::new(Mutex::new($y)));
	    )*
            LimbBindings::from(limbs)
	}
    };
}
