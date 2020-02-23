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

use gpio_cdev::{chips, LineHandle, LineRequestFlags};

pub struct OutputPin(LineHandle);

pub enum PinState {
    High,
    Low,
}

impl OutputPin {
    pub fn new(gpio_pin_name: &str) -> Result<Self, Error> {
        let pin_name_parts: Vec<&str> = gpio_pin_name.split('.').collect();
        for chip_result in chips().map_err(|_| Error::Io)? {
            let mut chip = chip_result.map_err(|_| Error::Io)?;
            if pin_name_parts[0].to_lowercase() == chip.label() {
                let line = pin_name_parts[1].parse().map_err(|_| Error::InvalidValue)?;
                let handle = chip
                    .get_line(line)
                    .map_err(|_| Error::MissingLimb)?
                    .request(LineRequestFlags::OUTPUT, 0, "phal limb")
                    .map_err(|_| Error::Io)?;
                return Ok(OutputPin(handle));
            }
        }
        Err(Error::MissingLimb)
    }
}

impl Limb for OutputPin {
    fn set(&mut self, value: String) -> Result<(), Error> {
        let requested_state = PinState::try_from(value)?;
        self.0
            .set_value(match requested_state {
                PinState::High => 1,
                PinState::Low => 0,
            })
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
