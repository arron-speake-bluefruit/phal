// Copyright (C) 2020 Arron Speake
// This is a fork of a project licensed under the following:
/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{Limb, LimbBindings, LimbTypes};
use crate::response_data::ResponseData;
use std::net::ToSocketAddrs;
use tiny_http::*;

pub struct PHALServer {
    types: LimbTypes,
    limbs: LimbBindings,
    server: Option<Server>,
}

type PHALServerError =
    std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

impl PHALServer {
    pub fn new(types: LimbTypes, address: impl ToSocketAddrs) -> Result<Self, PHALServerError> {
        let limbs = LimbBindings::new();
        Server::http(address).map(|server| Self {
            types,
            limbs,
            server: Some(server),
        })
    }

    pub fn run(mut self) {
        let server = self.server.take().unwrap();
        for mut request in server.incoming_requests() {
            let response = self.handle_request(&mut request);
            Self::log_response(&request, &response);
            let result = request.respond(response.into());
            if result.is_err() {
                eprintln!("Failed to respond to request.");
            }
        }
    }

    pub fn run_new(types: LimbTypes, address: impl ToSocketAddrs) -> Result<(), PHALServerError> {
        let server = Self::new(types, address)?;
        server.run();
        Ok(())
    }

    fn log_response(request: &Request, response: &ResponseData) {
        println!(
            "[{}] {} {} ~ {} {}",
            request.remote_addr().ip(),
            request.method(),
            request.url(),
            response.code.status_code(),
            response.code.name(),
        )
    }

    fn handle_limb_get_request(limb: &mut Box<dyn Limb>) -> ResponseData {
        match limb.get() {
            Ok(value) => ResponseData::ok(value.as_str()),
            Err(error) => ResponseData::bad_request(error.into()),
        }
    }

    fn set_limb_value(limb: &mut Box<dyn Limb>, value: String) -> ResponseData {
        match limb.set(value) {
            Ok(_) => ResponseData::ok("Limb successfully updated."),
            Err(error) => ResponseData::bad_request(error.into()),
        }
    }

    fn handle_limb_post_request(limb: &mut Box<dyn Limb>, request: &mut Request) -> ResponseData {
        let mut value = String::new();
        let result = request.as_reader().read_to_string(&mut value);
        match result {
            Ok(_) => Self::set_limb_value(limb, value),
            Err(_) => ResponseData::bad_request("Failed to read request"),
        }
    }

    fn handle_limb_request(limb: &mut Box<dyn Limb>, request: &mut Request) -> ResponseData {
        match request.method() {
            Method::Get => Self::handle_limb_get_request(limb),
            Method::Post => Self::handle_limb_post_request(limb, request),
            _ => ResponseData::method_not_allowed("Allowed: GET, POST"),
        }
    }

    fn handle_config_get_request() -> ResponseData {
        ResponseData::not_implemented("Configuration retrieval is not yet implemented.")
    }

    fn update_limb_configuration(&mut self, config: String) -> ResponseData {
        // For reasons beyond me, from_json fails if limbs is not first cleared.
        self.limbs.clear();
        match LimbBindings::from_json(&config, &self.types) {
            Some(new_limbs) => {
                self.limbs = new_limbs;
                ResponseData::configure_success()
            }
            None => ResponseData::bad_request("The provided configuration was ill-formed."),
        }
    }

    fn handle_config_post_request(&mut self, request: &mut Request) -> ResponseData {
        let mut config = String::new();
        let result = request.as_reader().read_to_string(&mut config);
        match result {
            Ok(_) => self.update_limb_configuration(config),
            Err(_) => ResponseData::bad_request("Failed to read request"),
        }
    }

    fn handle_config_request(&mut self, request: &mut Request) -> ResponseData {
        match request.method() {
            Method::Get => Self::handle_config_get_request(),
            Method::Post => self.handle_config_post_request(request),
            _ => ResponseData::bad_request("Allowed: GET, POST"),
        }
    }

    fn try_handle_limb_request<'a, I>(&mut self, mut url: I, request: &mut Request) -> ResponseData
    where
        I: Iterator<Item = &'a str>,
    {
        match url.next() {
            Some(limb_name) => {
                if let Some(limb) = self.limbs.get(limb_name) {
                    Self::handle_limb_request(limb, request)
                } else {
                    ResponseData::limb_not_found()
                }
            }
            None => ResponseData::forbidden(),
        }
    }

    fn handle_info_types_request(&self) -> ResponseData {
        let mut content = String::new();
        for name in self.types.names() {
            content += &format!("{}\n", name);
        }
        ResponseData::ok(content.as_str())
    }

    fn handle_info_limbs_request(&self) -> ResponseData {
        let mut content = String::new();
        for (key, value) in self.limbs.iter() {
            let name: String = key.escape_default().collect();
            content += &format!("\"{}\": {}\n", name, value.type_name());
        }
        ResponseData::ok(content.as_str())
    }

    fn handle_info_request<'a, I>(&mut self, mut url: I) -> ResponseData
    where
        I: Iterator<Item = &'a str>,
    {
        match url.next() {
            Some("types") => self.handle_info_types_request(),
            Some("limbs") => self.handle_info_limbs_request(),
            Some(_) => ResponseData::not_found(),
            None => ResponseData::forbidden(),
        }
    }

    fn handle_request(&mut self, req: &mut Request) -> ResponseData {
        let url_string = req.url().to_owned();
        let mut url = url_string.split('/').filter(|s| !s.is_empty());
        match url.next() {
            Some("limb") => self.try_handle_limb_request(url, req),
            Some("config") => self.handle_config_request(req),
            Some("info") => self.handle_info_request(url),
            Some(_) => ResponseData::not_found(),
            None => ResponseData::site_index(),
        }
    }
}