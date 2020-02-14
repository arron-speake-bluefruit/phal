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

#![feature(proc_macro_hygiene, decl_macro)]

use std::io::{self, Read};

#[macro_use]
extern crate rocket;
use rocket::{
    data::{self, FromDataSimple},
    http::Status,
    Data,
    Outcome::*,
    Request,
};

extern crate gpio;
use gpio::{sysfs::SysFsGpioOutput, GpioOut};

const LIMIT: u64 = 256;

enum PinState {
    High,
    Low,
}

impl FromDataSimple for PinState {
    type Error = String;
    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }
        match string.as_ref() {
            "High" => Success(PinState::High),
            "Low" => Success(PinState::Low),
            _ => Failure((Status::UnprocessableEntity, string)),
        }
    }
}

#[post("/fixture/pin", data = "<state>")]
fn set_pin(state: PinState) -> Result<(), io::Error> {
    let mut pin = SysFsGpioOutput::open(24)?;
    pin.set_value(match state {
        PinState::High => true,
        PinState::Low => false,
    })?;
    Ok(())
}

fn main() {
    rocket::ignite().mount("/", routes![set_pin]).launch();
}
