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

extern crate embedded_hal;
#[macro_use]
extern crate rocket;
extern crate xu4_hal;

use limb::{Error, Limb, LimbBindings};

use std::{collections::HashMap, sync::Mutex};

use rocket::State;
use xu4_hal::gpio as xu4;

#[post("/limb/<name>", data = "<value>")]
fn post_limb(limbs: State<LimbBindings>, name: String, value: String) -> Result<(), Error> {
    match limbs.get(&name) {
        Some(limb) => limb.lock().unwrap().set(value),
        None => Err(Error::MissingLimb),
    }
}

#[get("/limb/<name>")]
fn get_limb(limbs: State<LimbBindings>, name: String) -> Result<String, Error> {
    match limbs.get(&name) {
        Some(limb) => limb.lock().unwrap().get(),
        None => Err(Error::MissingLimb),
    }
}

fn main() {
    let limbs = limbs![
        ("red", xu4::OutputPin::new(xu4::Chip::Gpa2, 3).unwrap()),
        ("green", xu4::OutputPin::new(xu4::Chip::Gpa0, 2).unwrap()),
        ("blue", xu4::OutputPin::new(xu4::Chip::Gpx2, 0).unwrap()),
        ("input", xu4::InputPin::new(xu4::Chip::Gpa2, 6).unwrap())
    ];
    rocket::ignite()
        .manage(limbs)
        .mount("/", routes![post_limb, get_limb])
        .launch();
}
