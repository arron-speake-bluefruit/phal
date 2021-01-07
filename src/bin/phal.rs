/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

#[macro_use]
extern crate phal;

use phal::{
    limb::{Limb, LimbTypes},
    pin, serial, server,
};

use std::collections::HashMap;

fn main() {
    let types = limb_types![
        ("output-pin", pin::OutputPin),
        ("input-pin", pin::InputPin),
        ("serial", serial::Serial)
    ];
    let result = server::run(&types, "0.0.0.0:8000");
    if result.is_none() {
        eprintln!("The server stopped unexpectedly.");
    }
}
