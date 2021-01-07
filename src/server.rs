/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{self, Limb, LimbBindings, LimbTypes};
use std::net::ToSocketAddrs;
use tiny_http::*;

#[derive(Clone, Copy)]
enum HTTPError {
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotImplemented,
}

impl HTTPError {
    pub fn status_code(&self) -> u16 {
        use HTTPError::*;
        match self {
            BadRequest => 400,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            NotImplemented => 501,
        }
    }

    pub fn name(&self) -> &'static str {
        use HTTPError::*;
        match self {
            BadRequest => "Bad request",
            Forbidden => "Forbidden",
            NotFound => "Not found",
            MethodNotAllowed => "Method not allowed",
            NotImplemented => "Not implemented",
        }
    }
}

fn generate_error(error: HTTPError, content: &str) -> ResponseBox {
    let code = error.status_code();
    let name = error.name();
    eprintln!("Encountered error {} ({}).", code, name);
    let message = format!("{} {}\n{}", code, name, content);
    Response::from_string(message)
        .with_status_code(code)
        .boxed()
}

fn get_limb_error_name(error: limb::Error) -> &'static str {
    use limb::Error::*;
    match error {
        BrokenLimb => "Broken limb",
        InvalidValue => "Invalid Value",
        InvalidOperation => "Invalid operation",
    }
}

fn handle_limb_get_request(limb: &mut Box<dyn Limb>) -> ResponseBox {
    match limb.get() {
        Ok(value) =>
            Response::from_string(value).boxed(),
        Err(error) =>
            generate_error(HTTPError::BadRequest, get_limb_error_name(error)),
    }
}

fn set_limb_value(
    limb: &mut Box<dyn Limb>,
    value: String
) -> ResponseBox {
    match limb.set(value) {
        Ok(_) =>
            Response::empty(200).boxed(),
        Err(error) =>
            generate_error(HTTPError::BadRequest, get_limb_error_name(error)),
    }
}

fn handle_limb_post_request(
    limb: &mut Box<dyn Limb>,
    request: &mut Request,
) -> ResponseBox {
    let mut value = String::new();
    let result = request.as_reader().read_to_string(&mut value);
    match result {
        Ok(_) =>
            set_limb_value(limb, value),
        Err(_) =>
            generate_error(HTTPError::BadRequest, "Failed to read request"),
    }
}

fn handle_limb_request(
    limb: &mut Box<dyn Limb>,
    request: &mut Request,
) -> ResponseBox {
    match request.method() {
        Method::Get => handle_limb_get_request(limb),
        Method::Post => handle_limb_post_request(limb, request),
        _ =>
            generate_error(HTTPError::MethodNotAllowed, "Allowed: GET, POST"),
    }
}

fn handle_config_get_request() -> ResponseBox {
    generate_error(
        HTTPError::NotImplemented,
        "Config retrieval is not yet implemented.")
}

fn update_limb_configuration(
    config: String,
    types: &LimbTypes,
    limbs: &mut LimbBindings
) -> ResponseBox {
    // For reasons beyond me, from_json fails if limbs is not first cleared.
    limbs.clear();
    match LimbBindings::from_json(&config, types) {
        Some(new_limbs) => {
            *limbs = new_limbs;
            Response::empty(200).boxed()
        }
        None => generate_error(
            HTTPError::BadRequest,
            "The provided configuration was ill-formed."),
    }
}

fn handle_config_post_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    request: &mut Request,
) -> ResponseBox {
    let mut config = String::new();
    let result = request.as_reader().read_to_string(&mut config);
    match result {
        Ok(_) =>
            update_limb_configuration(config, types, limbs),
        Err(_) =>
            generate_error(HTTPError::BadRequest, "Failed to read request"),
    }
}

fn handle_config_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    request: &mut Request,
) -> ResponseBox {
    match request.method() {
        Method::Get =>
            handle_config_get_request(),
        Method::Post =>
            handle_config_post_request(types, limbs, request),
        _ =>
            generate_error(HTTPError::MethodNotAllowed, "Allowed: GET, POST"),
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
                generate_error(HTTPError::NotFound, "That limb does not exist")
            }
        },
        None => generate_error(HTTPError::Forbidden, ""),
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
        Some(_) => generate_error(HTTPError::NotFound, ""),
        None => Response::from_string("PHAL Server").boxed(),
    }
}

pub fn run(types: &LimbTypes, address: impl ToSocketAddrs) -> Option<()> {
    let mut limbs = LimbBindings::new();
    let server = match Server::http(address) {
        Ok(s) => s,
        Err(error) => {
            eprintln!("Server Error: {}", error);
            return None;
        }
    };

    for mut request in server.incoming_requests() {
        let response = handle_request(&types, &mut limbs, &mut request);
        let result = request.respond(response);
        if result.is_err() {
            eprintln!("Failed to respond to request.");
        }
    }

    Some(())
}