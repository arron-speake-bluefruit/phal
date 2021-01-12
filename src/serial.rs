// Copyright (C) 2020 Arron Speake
// This is a fork of a project licensed under the following:
/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use serde_json as json;
use crate::{
    limb::{Error, Limb},
    port_settings_from_json::port_settings_from_json,
};
use serial::{self, SerialPort};
use std::io::{Read, Write};

pub struct Serial(serial::SystemPort);

impl Limb for Serial {
    fn from_json(config: &json::Value) -> Option<Self> {
        let mut port = match &config["device"] {
            json::Value::String(s) => serial::open(&s).ok().map(Serial),
            _ => None,
        }?;
        let settings = port_settings_from_json(config)?;
        port.0
            .reconfigure(&|s| {
                s.set_baud_rate(settings.baud_rate)?;
                s.set_char_size(settings.char_size);
                s.set_parity(settings.parity);
                s.set_stop_bits(settings.stop_bits);
                s.set_flow_control(settings.flow_control);
                Ok(())
            })
            .ok();
        Some(port)
    }

    fn set(&mut self, value: String) -> Result<(), Error> {
        self.0
            .write_all(value.as_bytes())
            .map_err(|_| Error::BrokenLimb)
    }

    fn get(&mut self) -> Result<String, Error> {
        let mut bytes = Vec::new();
        let _ = self.0.read_to_end(&mut bytes);
        // ^ Returns error when reaching EOF for some reason. For now, just
        // ignore the error and return partial/empty result below.
        String::from_utf8(bytes).map_err(|_| Error::BrokenLimb)
    }

    fn type_name(&self) -> &'static str {
        "serial"
    }
}
