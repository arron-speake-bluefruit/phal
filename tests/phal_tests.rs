/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

#[macro_use]
extern crate phal;
extern crate ureq;

use phal::{
    limb::{Error, Limb, LimbTypes},
    server,
};
use serde_json as json;
use std::{collections::HashMap, thread, time};

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
    thread::spawn(|| {
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
        server::run(&types, config, "localhost:2000").unwrap()
    });
    thread::sleep(time::Duration::from_millis(10));
    assert!(ureq::get("http://localhost:2000/limb/bar").call().ok());
    assert!(ureq::post("http://localhost:2000/limb/bar")
        .send_string("foo")
        .ok());
    assert!(ureq::get("http://localhost:2000/limb/baz").call().ok());
    assert!(ureq::post("http://localhost:2000/limb/baz")
        .send_string("foo")
        .ok());
}

#[test]
fn get_and_post_requests_call_get_and_set_on_a_limb() {

    thread::spawn(|| {
        let types = limb_types![("foo", MockLimb)];
        let config = r#"
        {
            "bar": {
                "type": "foo"
            }
        }
        "#;
        server::run(&types, config, "localhost:2001").unwrap()
    });
    thread::sleep(time::Duration::from_millis(10));
    ureq::post("http://localhost:2001/limb/bar").send_string("baz");
    assert_eq!(
        ureq::get("http://localhost:2001/limb/bar")
            .call()
            .into_string()
            .unwrap(),
        "baz".to_string()
    );
    ureq::post("http://localhost:2001/limb/bar").send_string("quux");
    assert_eq!(
        ureq::get("http://localhost:2001/limb/bar")
            .call()
            .into_string()
            .unwrap(),
        "quux".to_string()
    );
}

#[test]
fn a_limb_is_set_to_its_init_config_property_on_start_up_if_it_exists() {
    thread::spawn(|| {
        let types = limb_types![("foo", MockLimb)];
        let config = r#"
        {
            "bar": {
                "type": "foo",
                "init": "baz"
            }
        }
        "#;
        server::run(&types, config, "localhost:2002").unwrap()
    });
    thread::sleep(time::Duration::from_millis(10));
    assert_eq!(
        ureq::get("http://localhost:2002/limb/bar")
            .call()
            .into_string()
            .unwrap(),
        "baz".to_string()
    );
}

#[test]
fn config_can_be_updated_via_config_endpoint() {
    thread::spawn(|| {
        let types = limb_types![("foo", MockLimb)];
        let config = "{}";
        server::run(&types, config, "localhost:2003").unwrap()
    });
    thread::sleep(time::Duration::from_millis(10));
    let new_config = r#"
        {
            "bar": {
                "type": "foo",
                "init": "baz"
            }
        }
    "#;
    assert!(ureq::post("http://localhost:2003/config").send_string(new_config).ok());
    assert_eq!(
        ureq::get("http://localhost:2003/limb/bar")
            .call()
            .into_string()
            .unwrap(),
        "baz".to_string()
    );
}
