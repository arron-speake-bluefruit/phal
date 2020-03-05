/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{Error, LimbBindings, LimbTypes};
use rocket::{ignite, Rocket, State};

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

pub fn rocket(types: LimbTypes, config: String) -> Option<Rocket> {
    LimbBindings::from_json(config, types).map(|limbs| {
        ignite()
            .manage(limbs)
            .mount("/", routes![post_limb, get_limb])
    })
}
