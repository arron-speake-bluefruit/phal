// Copyright (C) 2020 Arron Speake
// This is a fork of a project licensed under the following:
/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

#[macro_use]
extern crate phal;

use phal::{
    limb::{Limb, LimbTypes},
    pin, serial, server::PHALServer,
};

use std::collections::HashMap;

fn main() {
    let address = "0.0.0.0:8000";
    let types = limb_types![
        ("output-pin", pin::OutputPin),
        ("input-pin", pin::InputPin),
        ("serial", serial::Serial)
    ];

    match PHALServer::run_new(types, address) {
        Ok(_) =>
            eprintln!("The server stopped unexpectedly."),
        Err(error) =>
            eprintln!("Failed to start server: {}", error),
    }
}
