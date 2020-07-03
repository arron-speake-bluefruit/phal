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

fn handle_request(types: &LimbTypes, limbs: &mut LimbBindings, req: &mut Request) -> ResponseBox {
    let url_parts: Vec<&str> = req.url().split('/').filter(|s| !s.is_empty()).collect();
    match url_parts[0] {
        "limb" => limbs
            .get(url_parts[1])
            .map(|limb| handle_limb_request(limb, req))
            .unwrap_or(Response::empty(404).boxed()),
        "config" => handle_config_request(types, limbs, req),
        _ => Response::empty(400).boxed()
    }
}

pub fn run(types: &LimbTypes, addr: impl ToSocketAddrs) -> Option<()> {
    let mut limbs = LimbBindings::new();
    let server = Server::http(addr).ok()?;
    for mut req in server.incoming_requests() {
        let resp = handle_request(&types, &mut limbs, &mut req);
        req.respond(resp).unwrap_or_else(|_| {
            eprintln!("Couldn't respond to request!");
        });
    }
    Some(())
}
