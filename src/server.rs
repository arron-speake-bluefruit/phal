/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{self, Limb, LimbBindings, LimbTypes};
use regex::Regex;
use std::{net::ToSocketAddrs, sync::mpsc::*, thread};
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

fn handle_request(limbs: &mut LimbBindings, req: &mut Request) -> ResponseBox {
    lazy_static! {
        static ref LIMB_RE: Regex = Regex::new(r"^/limb/([^/]+)$").unwrap();
        static ref CONFIG_RE: Regex = Regex::new(r"^/config$").unwrap();
    }
    LIMB_RE
        .captures(&req.url().to_string())
        .and_then(|c| Some(c.get(1)?.as_str()))
        .map(|name| {
            limbs
                .get(name)
                .map(|limb| handle_limb_request(limb, req))
                .unwrap_or(Response::empty(404).boxed())
        })
        .unwrap_or(if CONFIG_RE.is_match(req.url()) {
            Response::empty(501).boxed()
        } else {
            Response::empty(400).boxed()
        })
}

pub fn run(types: LimbTypes, config: &str, addr: impl ToSocketAddrs) -> Option<Sender<()>> {
    let mut limbs = LimbBindings::from_json(config, types)?;
    let server = Server::http(addr).ok()?;
    let (tx, rx) = channel();
    thread::spawn(move || {
        for mut req in server.incoming_requests() {
            let resp = handle_request(&mut limbs, &mut req);
            req.respond(resp).unwrap();
            if rx.try_recv().is_ok() {
                break;
            }
        }
    });
    Some(tx)
}
