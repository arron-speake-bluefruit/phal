/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{self, Limb, LimbBindings, LimbTypes};
use std::net::ToSocketAddrs;
use tiny_http::*;

#[derive(Clone, Copy)]
enum HTTPStatusCode {
    OK,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotImplemented,
}

impl HTTPStatusCode {
    pub fn status_code(&self) -> u16 {
        use HTTPStatusCode::*;
        match self {
            OK => 200,
            BadRequest => 400,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            NotImplemented => 501,
        }
    }

    pub fn name(&self) -> &'static str {
        use HTTPStatusCode::*;
        match self {
            OK => "OK",
            BadRequest => "Bad request",
            Forbidden => "Forbidden",
            NotFound => "Not found",
            MethodNotAllowed => "Method not allowed",
            NotImplemented => "Not implemented",
        }
    }
}

struct ResponseData {
    pub code: HTTPStatusCode,
    pub content: String,
}

impl ResponseData {
    pub fn configure_success() -> Self {
        Self {
            code: HTTPStatusCode::OK,
            content: "Configuration completed successfullly.".to_owned()
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: HTTPStatusCode::NotFound,
            content: "".to_owned(),
        }
    }

    pub fn limb_not_found() -> Self {
        Self {
            code: HTTPStatusCode::NotFound,
            content: "That limb does not exist.".to_owned()
        }
    }

    pub fn ok(content: &str) -> Self {
        Self {
            code: HTTPStatusCode::OK,
            content: content.to_owned(),
        }
    }

    pub fn bad_request(content: &str) -> Self {
        Self {
            code: HTTPStatusCode::BadRequest,
            content: content.to_owned(),
        }
    }

    pub fn method_not_allowed(content: &str) -> Self {
        Self {
            code: HTTPStatusCode::MethodNotAllowed,
            content: content.to_owned(),
        }
    }

    pub fn not_implemented(content: &str) -> Self {
        Self {
            code: HTTPStatusCode::NotImplemented,
            content: content.to_owned(),
        }
    }

    pub fn forbidden() -> Self {
        Self {
            code: HTTPStatusCode::Forbidden,
            content: "".to_owned(),
        }
    }

    pub fn site_index() -> Self {
        Self::ok("PHAL Server")
    }
}

impl Into<ResponseBox> for ResponseData {
    fn into(self) -> tiny_http::ResponseBox {
        let code = self.code.status_code();
        let name = self.code.name();
        let message = format!("{} {}\n{}", code, name, self.content);
        Response::from_string(message)
            .with_status_code(code)
            .boxed()
    }
}

fn get_limb_error_name(error: limb::Error) -> &'static str {
    use limb::Error::*;
    match error {
        BrokenLimb => "Broken limb",
        InvalidValue => "Invalid Value",
        InvalidOperation => "Invalid operation",
    }
}

fn handle_limb_get_request(limb: &mut Box<dyn Limb>) -> ResponseData {
    match limb.get() {
        Ok(value) => ResponseData::ok(value.as_str()),
        Err(error) => ResponseData::bad_request(get_limb_error_name(error))
    }
}

fn set_limb_value(
    limb: &mut Box<dyn Limb>,
    value: String
) -> ResponseData {
    match limb.set(value) {
        Ok(_) => ResponseData::ok("Limb successfully updated."),
        Err(error) => ResponseData::bad_request(get_limb_error_name(error)),
    }
}

fn handle_limb_post_request(
    limb: &mut Box<dyn Limb>,
    request: &mut Request,
) -> ResponseData {
    let mut value = String::new();
    let result = request.as_reader().read_to_string(&mut value);
    match result {
        Ok(_) => set_limb_value(limb, value),
        Err(_) => ResponseData::bad_request("Failed to read request"),
    }
}

fn handle_limb_request(
    limb: &mut Box<dyn Limb>,
    request: &mut Request,
) -> ResponseData {
    match request.method() {
        Method::Get => handle_limb_get_request(limb),
        Method::Post => handle_limb_post_request(limb, request),
        _ => ResponseData::method_not_allowed("Allowed: GET, POST"),
    }
}

fn handle_config_get_request() -> ResponseData {
    ResponseData::not_implemented(
        "Configuration retrieval is not yet implemented.")
}

fn update_limb_configuration(
    config: String,
    types: &LimbTypes,
    limbs: &mut LimbBindings
) -> ResponseData {
    // For reasons beyond me, from_json fails if limbs is not first cleared.
    limbs.clear();
    match LimbBindings::from_json(&config, types) {
        Some(new_limbs) => {
            *limbs = new_limbs;
            ResponseData::configure_success()
        }
        None => ResponseData::bad_request(
            "The provided configuration was ill-formed."),
    }
}

fn handle_config_post_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    request: &mut Request,
) -> ResponseData {
    let mut config = String::new();
    let result = request.as_reader().read_to_string(&mut config);
    match result {
        Ok(_) =>
            update_limb_configuration(config, types, limbs),
        Err(_) => ResponseData::bad_request("Failed to read request"),
    }
}

fn handle_config_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    request: &mut Request,
) -> ResponseData {
    match request.method() {
        Method::Get =>
            handle_config_get_request(),
        Method::Post =>
            handle_config_post_request(types, limbs, request),
        _ =>
            ResponseData::bad_request("Allowed: GET, POST"),
    }
}

fn try_handle_limb_request<'a, I> (
    mut url: I,
    limbs: &mut LimbBindings,
    request: &mut Request,
) -> ResponseData where I: Iterator<Item = &'a str> {
    match url.next() {
        Some(limb_name) => {
            if let Some(limb) = limbs.get(limb_name) {
                handle_limb_request(limb, request)
            } else {
                ResponseData::limb_not_found()
            }
        },
        None => ResponseData::forbidden()
    }
}

fn handle_request(
    types: &LimbTypes,
    limbs: &mut LimbBindings,
    req: &mut Request,
) -> ResponseData {
    let url_string = req.url().to_owned();
    let mut url = url_string.split('/')
        .filter(|s| !s.is_empty());
    match url.next() {
        Some("limb") => try_handle_limb_request(url, limbs, req),
        Some("config") => handle_config_request(types, limbs, req),
        Some(_) => ResponseData::not_found(),
        None => ResponseData::site_index(),
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
        let result = request.respond(response.into());
        if result.is_err() {
            eprintln!("Failed to respond to request.");
        }
    }

    Some(())
}