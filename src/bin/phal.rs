/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

#[macro_use]
extern crate phal;

use phal::{
    limb::{Limb, LimbTypes},
    server,
};

use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    sync::Mutex,
};

use xu4_hal::gpio as xu4;

fn file_contents<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut reader = File::open(path).map(BufReader::new).ok()?;
    let mut s = String::new();
    reader.read_to_string(&mut s).ok()?;
    Some(s)
}

fn main() {
    let types = limb_types![("output-pin", xu4::OutputPin), ("input-pin", xu4::InputPin)];
    let config = args()
        .nth(1)
        .and_then(file_contents)
        .expect("Failed to read config");
    server::rocket(types, config)
        .expect("Failed to initialise server")
        .launch();
}
