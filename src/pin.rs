/*
 * Copyright (C) 2020 Callum David O'Brien
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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
