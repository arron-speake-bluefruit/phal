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

use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidValue,
    InvalidOperation,
    MissingLimb,
}

pub trait Limb: Send + Sync {
    fn set(&mut self, value: String) -> Result<(), Error>;
    fn get(&mut self) -> Result<String, Error>;
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
