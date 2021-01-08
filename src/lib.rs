// Copyright (C) 2020 Arron Speake
// This is a fork of a project licensed under the following:
/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

mod http_status_code;
mod response_data;

pub mod limb;
pub mod pin;
pub mod serial;
pub mod server;

extern crate gpio_cdev;
extern crate serde_json;
extern crate serial as system_serial;
extern crate tiny_http;
