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

fn handle_limb_get_request(limb: &mut Box<dyn Limb>) -> ResponseBox {
    match limb.get() {
        Ok(value) => Response::from_string(value).boxed(),
        Err(error) => limb_error_response(error)
    }
}

fn set_limb_value(
    limb: &mut Box<dyn Limb>,
    value: String
) -> ResponseBox {
    match limb.set(value) {
        Ok(_) => Response::empty(200).boxed(),
        Err(_) => Response::empty(400).boxed(),
    }
}

fn handle_limb_post_request(
    limb: &mut Box<dyn Limb>,
    request: &mut Request,
) -> ResponseBox {
    let mut value = String::new();
    let result = request.as_reader().read_to_string(&mut value);
    match result {
        Ok(_) => set_limb_value(limb, value),
        Err(_) => Response::empty(400).boxed(),
    }
}

fn handle_limb_request(
    limb: &mut Box<dyn Limb>,
    request: &mut Request,
) -> ResponseBox {
    match request.method() {
        Method::Get => handle_limb_get_request(limb),
        Method::Post => handle_limb_post_request(limb, request),
        _ => Response::empty(405).boxed(),
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

fn try_handle_limb_request<'a, I> (
    mut url: I,
    limbs: &mut LimbBindings,
    request: &mut Request,
) -> ResponseBox where I: Iterator<Item = &'a str> {
    match url.next() {
        Some(limb_name) => {
            if let Some(limb) = limbs.get(limb_name) {
                handle_limb_request(limb, request)
            } else {
                Response::empty(404).boxed()
            }
        },
        None => Response::empty(403).boxed(),
    }
}

fn handle_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    req: &mut Request,
) -> ResponseBox {
    let url_string = req.url().to_owned();
    let mut url = url_string.split('/')
        .filter(|s| !s.is_empty());
    match url.next() {
        Some("limb") => try_handle_limb_request(url, limbs, req),
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