/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

#![feature(proc_macro_hygiene, decl_macro)]

mod limb;
mod pin;

extern crate embedded_hal;
#[macro_use]
extern crate rocket;
extern crate serde_json;
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
