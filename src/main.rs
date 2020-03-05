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

use limb::{Error, Limb, LimbBindings, LimbTypes};

use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    sync::Mutex,
};

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

fn file_contents<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut reader = File::open(path).map(BufReader::new).ok()?;
    let mut s = String::new();
    reader.read_to_string(&mut s).ok()?;
    Some(s)
}

fn main() {
    let types = limb_types![("output-pin", xu4::OutputPin), ("input-pin", xu4::InputPin)];
    let limbs = args()
        .nth(1)
        .and_then(file_contents)
        .and_then(|s| LimbBindings::from_json(s, types))
        .expect("Failed to read config");
    rocket::ignite()
        .manage(limbs)
        .mount("/", routes![post_limb, get_limb])
        .launch();
}
