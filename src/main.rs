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

#![feature(proc_macro_hygiene, decl_macro)]

mod limb;
mod pin;

extern crate gpio;
#[macro_use]
extern crate rocket;

use limb::{Limb, Error};
use pin::OutputPin;

use std::{collections::HashMap, sync::Mutex};

use rocket::State;

#[post("/limb/<name>", data = "<value>")]
fn post_limb(limbs: State<HashMap<String, Box<Mutex<dyn Limb>>>>, name: String, value: String) -> Result<(), Error> {
    match limbs.get(&name) {
        Some(limb) => limb.lock().unwrap().set(value),
        None => Err(Error::MissingLimb),
    }
}

fn main() {
    let mut limbs: HashMap<String, Box<Mutex<dyn Limb>>> = HashMap::new();
    limbs.insert(String::from("pin"),
		 Box::new(Mutex::new(OutputPin::new(24).unwrap())));
    rocket::ignite()
        .manage(limbs)
        .mount("/", routes![post_limb])
        .launch();
}
