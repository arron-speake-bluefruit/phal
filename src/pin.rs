/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{Error, Limb};

use std::convert::TryInto;

use gpio_cdev as cdev;
use serde_json as json;

impl From<cdev::errors::Error> for Error {
    fn from(_: cdev::errors::Error) -> Self {
        Self::BrokenLimb
    }
}

pub struct OutputPin(cdev::LineHandle);

impl Limb for OutputPin {
    fn from_json(config: &json::Value) -> Option<Self> {
        open_line_handle(config, cdev::LineRequestFlags::OUTPUT).map(|h| OutputPin(h))
    }

    fn set(&mut self, value: String) -> Result<(), Error> {
        match value.as_ref() {
            "High" => self.0.set_value(1).map_err(|e| e.into()),
            "Low" => self.0.set_value(0).map_err(|e| e.into()),
            _ => Err(Error::InvalidValue),
        }
    }

    fn get(&mut self) -> Result<String, Error> {
        Err(Error::InvalidOperation)
    }
}

pub struct InputPin(cdev::LineHandle);

impl Limb for InputPin {
    fn from_json(config: &json::Value) -> Option<Self> {
        open_line_handle(config, cdev::LineRequestFlags::INPUT).map(|h| InputPin(h))
    }

    fn set(&mut self, _value: String) -> Result<(), Error> {
        Err(Error::InvalidOperation)
    }

    fn get(&mut self) -> Result<String, Error> {
        let value = self.0.get_value()?;
        match value {
            1 => Ok(String::from("High")),
            0 => Ok(String::from("Low")),
            _ => Err(Error::BrokenLimb),
        }
    }
}

fn open_line_handle(
    config: &json::Value,
    flags: cdev::LineRequestFlags,
) -> Option<cdev::LineHandle> {
    let mut chip = match &config["chip"] {
        json::Value::String(s) => cdev::Chip::new(s).ok(),
        _ => None,
    }?;
    let line = match &config["line"] {
        json::Value::Number(n) => n.as_u64().and_then(|x| x.try_into().ok()),
        _ => None,
    }?;
    let type_flag = match &config["pin-type"] {
        json::Value::String(s) => match s.as_ref() {
            "push-pull" => Some(cdev::LineRequestFlags::empty()),
            "open-drain" => Some(cdev::LineRequestFlags::OPEN_DRAIN),
            _ => None,
        },
        _ => None,
    }?;
    chip.get_line(line)
        .ok()?
        .request(flags | type_flag, 0, "Phal server")
        .ok()
}
