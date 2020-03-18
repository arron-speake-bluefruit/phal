/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{Error, Limb};

use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

use embedded_hal::digital::v2::*;
use serde_json as json;
use xu4_hal::gpio as xu4;

pub enum PinState {
    High,
    Low,
}

impl Limb for xu4::OutputPin {
    fn from_json(config: &json::Value) -> Option<Self> {
        pin_from_json(xu4::OutputPin::new, config)
    }

    fn set(&mut self, value: String) -> Result<(), Error> {
        let requested_state = PinState::try_from(value)?;
        match requested_state {
            PinState::High => self.set_high(),
            PinState::Low => self.set_low(),
        }
        .map_err(|_| Error::BrokenLimb)
    }

    fn get(&mut self) -> Result<String, Error> {
        Err(Error::InvalidOperation)
    }
}

impl Limb for xu4::InputPin {
    fn from_json(config: &json::Value) -> Option<Self> {
        pin_from_json(xu4::InputPin::new, config)
    }

    fn set(&mut self, _value: String) -> Result<(), Error> {
        Err(Error::InvalidOperation)
    }

    fn get(&mut self) -> Result<String, Error> {
        if self.is_high().map_err(|_| Error::BrokenLimb)? {
            Ok(String::from("High"))
        } else if self.is_low().map_err(|_| Error::BrokenLimb)? {
            Ok(String::from("Low"))
        } else {
            Err(Error::BrokenLimb)
        }
    }
}

fn pin_from_json<F, T>(pin: F, config: &json::Value) -> Option<T>
where
    F: Fn(xu4::Chip, u32, xu4::Type) -> Result<T, xu4::Error>,
{
    let chip = match &config["chip"] {
        json::Value::String(s) => xu4::Chip::from_str(&s).ok(),
        _ => None,
    }?;
    let line = match &config["line"] {
        json::Value::Number(n) => n.as_u64().and_then(|x| x.try_into().ok()),
        _ => None,
    }?;
    let pin_type = match &config["pin-type"] {
        json::Value::String(s) => match s.as_ref() {
            "push-pull" => Some(xu4::Type::PushPull),
            "open-drain" => Some(xu4::Type::OpenDrain),
            _ => None,
        },
        _ => None,
    }?;
    pin(chip, line, pin_type).ok()
}

impl TryFrom<String> for PinState {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_ref() {
            "High" => Ok(PinState::High),
            "Low" => Ok(PinState::Low),
            _ => Err(Error::InvalidValue),
        }
    }
}
