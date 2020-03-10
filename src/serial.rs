/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{Error, Limb};

use serde_json as json;
use serial;

use std::io::{Read, Write};

pub struct Serial(serial::SystemPort);

impl Limb for Serial {
    fn from_json(config: &json::Value) -> Option<Self> {
        match &config["device"] {
            json::Value::String(s) => serial::open(&s).ok().map(Serial),
            _ => None,
        }
    }

    fn set(&mut self, value: String) -> Result<(), Error> {
        self.0
            .write_all(value.as_bytes())
            .map_err(|_| Error::BrokenLimb)
    }

    fn get(&mut self) -> Result<String, Error> {
        let mut v: Vec<u8> = Vec::new();
        loop {
            let mut buffer = [0; 1];
            if let Err(e) = self.0.read(&mut buffer) {
                break;
            }
            v.push(buffer[0]);
        }
        String::from_utf8(v).map_err(|_| Error::BrokenLimb)
    }
}
