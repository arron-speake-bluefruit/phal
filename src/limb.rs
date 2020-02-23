/*
 * Copyright (C) 2020 Callum David O'Brien
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
