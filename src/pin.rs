/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{Error, Limb};

use std::convert::TryFrom;

use embedded_hal::digital::v2::*;
use xu4_hal::gpio as xu4;

pub enum PinState {
    High,
    Low,
}

impl Limb for xu4::OutputPin {
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
