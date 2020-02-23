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

extern crate gpio_cdev;
#[macro_use]
extern crate rocket;

use limb::{Error, Limb, LimbBindings};
use pin::OutputPin;

use std::{collections::HashMap, sync::Mutex};

use rocket::State;

#[post("/limb/<name>", data = "<value>")]
fn post_limb(limbs: State<LimbBindings>, name: String, value: String) -> Result<(), Error> {
    match limbs.get(&name) {
        Some(limb) => limb.lock().unwrap().set(value),
        None => Err(Error::MissingLimb),
    }
}

fn main() {
    let limbs = limbs![
        ("red", OutputPin::new("GPA2.3").unwrap()),
        ("green", OutputPin::new("GPA0.2").unwrap()),
        ("blue", OutputPin::new("GPX2.0").unwrap())
    ];
    rocket::ignite()
        .manage(limbs)
        .mount("/", routes![post_limb])
        .launch();
}
