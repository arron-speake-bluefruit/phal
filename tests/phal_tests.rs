/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

#[macro_use]
extern crate phal;
extern crate rocket;

use phal::{
    limb::{Error, Limb, LimbTypes},
    server,
};
use rocket::{http::Status, local::Client};
use serde_json as json;

use std::{collections::HashMap, sync::Mutex};

struct MockLimb(String);

impl Limb for MockLimb {
    fn from_json(_config: &json::Value) -> Option<Self> {
        Some(MockLimb(String::new()))
    }

    fn get(&mut self) -> Result<String, Error> {
        Ok(self.0.clone())
    }

    fn set(&mut self, value: String) -> Result<(), Error> {
        self.0 = value;
        Ok(())
    }
}

#[test]
fn server_has_endpoints_for_limbs_in_config() {
    let types = limb_types![("foo", MockLimb)];
    let config = r#"
        {
            "bar": {
                "type": "foo"
            },
            "baz": {
                "type": "foo"
            }
        }
    "#;
    let client = server::rocket(types, config.to_string())
        .and_then(|r| Client::new(r).ok())
        .unwrap();

    assert_eq!(client.get("/limb/bar").dispatch().status(), Status::Ok);
    assert_eq!(client.post("/limb/bar").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/limb/baz").dispatch().status(), Status::Ok);
    assert_eq!(client.post("/limb/baz").dispatch().status(), Status::Ok);
}

#[test]
fn get_and_post_requests_call_get_and_set_on_a_limb() {
    let types = limb_types![("foo", MockLimb)];
    let config = r#"
        {
            "bar": {
                "type": "foo"
            }
        }
    "#;
    let client = server::rocket(types, config.to_string())
        .and_then(|r| Client::new(r).ok())
        .unwrap();

    client.post("/limb/bar").body("baz").dispatch().status();
    assert_eq!(
        client.get("/limb/bar").dispatch().body_string(),
        Some("baz".to_string())
    );
    client.post("/limb/bar").body("quux").dispatch().status();
    assert_eq!(
        client.get("/limb/bar").dispatch().body_string(),
        Some("quux".to_string())
    );
}

#[test]
fn a_limb_is_set_to_its_init_config_property_on_start_up_if_it_exists() {
    let types = limb_types![("foo", MockLimb)];
    let config = r#"
        {
            "bar": {
                "type": "foo",
                "init": "baz"
            }
        }
    "#;
    let client = server::rocket(types, config.to_string())
        .and_then(|r| Client::new(r).ok())
        .unwrap();

    assert_eq!(
        client.get("/limb/bar").dispatch().body_string(),
        Some("baz".to_string())
    );
}
