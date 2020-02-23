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

use std::{convert::TryFrom, io};

use gpio::{sysfs::SysFsGpioOutput, GpioOut};

pub struct OutputPin(SysFsGpioOutput);

pub enum PinState {
    High,
    Low,
}

impl OutputPin {
    pub fn new(sysfs_gpio_number: u16) -> Result<Self, io::Error> {
        Ok(OutputPin(SysFsGpioOutput::open(sysfs_gpio_number)?))
    }
}

impl Limb for OutputPin {
    fn set(&mut self, value: String) -> Result<(), Error> {
        let requested_state = PinState::try_from(value)?;
        self.0.set_value(match requested_state {
            PinState::High => true,
            PinState::Low => false,
        })?;
        Ok(())
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
