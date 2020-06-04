/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

pub mod limb;
pub mod pin;
pub mod serial;
pub mod server;

extern crate embedded_hal;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate serial as system_serial;
extern crate tiny_http;
extern crate xu4_hal;
