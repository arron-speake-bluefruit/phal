/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{self, Limb, LimbBindings, LimbTypes};
use std::net::ToSocketAddrs;
use tiny_http::*;

fn limb_error_response(err: limb::Error) -> ResponseBox {
    use limb::Error::*;
    Response::empty(match err {
        BrokenLimb => 500,
        InvalidValue => 400,
        InvalidOperation => 400,
    })
    .boxed()
}

fn handle_limb_request(limb: &mut Box<dyn Limb>, req: &mut Request) -> ResponseBox {
    match req.method() {
        Method::Get => limb.get().map_or_else(limb_error_response, |val| {
            Response::from_string(val).boxed()
        }),
        Method::Post => {
            let mut val = String::new();
            req.as_reader()
                .read_to_string(&mut val)
                .map(|_| {
                    limb.set(val)
                        .map_or_else(limb_error_response, |_| Response::empty(200).boxed())
                })
                .unwrap_or(Response::empty(400).boxed())
        }
        _ => Response::empty(400).boxed(),
    }
}

fn handle_config_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    req: &mut Request,
) -> ResponseBox {
    match req.method() {
        Method::Get => Response::empty(501).boxed(),
        Method::Post => {
            limbs.clear();
            let mut config = String::new();
            req.as_reader()
                .read_to_string(&mut config)
                .map(|_| match LimbBindings::from_json(&config, types) {
                    Some(new_limbs) => {
                        *limbs = new_limbs;
                        Response::empty(200).boxed()
                    }
                    None => Response::empty(400).boxed(),
                })
                .unwrap_or(Response::empty(400).boxed())
        }
        _ => Response::empty(400).boxed(),
    }
}

fn handle_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    req: &mut Request
) -> ResponseBox {
    let mut url = req.url()
        .split('/')
        .filter(|s| !s.is_empty());
    match url.next() {
        Some("limb") => limbs.get(url.next().unwrap_or(""))
            .map(|limb| handle_limb_request(limb, req))
            .unwrap_or_else(|| { Response::empty(404).boxed() }),
        Some("config") => handle_config_request(types, limbs, req),
        Some(_) => Response::empty(404).boxed(),
        None => Response::from_string("PHAL Server").boxed(),
    }
}

pub fn run(types: &LimbTypes, address: impl ToSocketAddrs) -> Option<()> {
    let mut limbs = LimbBindings::new();
    let server = Server::http(address).ok()?;

    for mut request in server.incoming_requests() {
        let response = handle_request(&types, &mut limbs, &mut request);
        let result = request.respond(response);
        if result.is_err() {
            eprintln!("Failed to respond to request.");
        }
    }

    Some(())
}