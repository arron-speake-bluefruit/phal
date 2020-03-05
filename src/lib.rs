/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

#![feature(proc_macro_hygiene, decl_macro)]

pub mod limb;
pub mod pin;
pub mod server;

extern crate embedded_hal;
#[macro_use]
extern crate rocket;
extern crate serde_json;
extern crate xu4_hal;
