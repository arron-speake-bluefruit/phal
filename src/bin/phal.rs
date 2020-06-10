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

use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

fn file_contents<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut reader = File::open(path).map(BufReader::new).ok()?;
    let mut s = String::new();
    reader.read_to_string(&mut s).ok()?;
    Some(s)
}

fn main() {
    let types = limb_types![
        ("output-pin", pin::OutputPin),
        ("input-pin", pin::InputPin),
        ("serial", serial::Serial)
    ];
    let config = args()
        .nth(1)
        .and_then(file_contents)
        .unwrap_or("{}".to_string());
    server::run(&types, &config, "0.0.0.0:8000").expect("Failed to run server");
}
