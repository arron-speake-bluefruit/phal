/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2020 Callum David O'Brien
 */

use crate::limb::{Error, Limb};

use serde_json as json;
use serial::{self, SerialPort};

use std::{
    convert::TryInto,
    io::{Read, Write},
};

pub struct Serial(serial::SystemPort);

impl Limb for Serial {
    fn from_json(config: &json::Value) -> Option<Self> {
        let mut port = match &config["device"] {
            json::Value::String(s) => serial::open(&s).ok().map(Serial),
            _ => None,
        }?;
        let settings = port_settings_from_json(config)?;
        port.0
            .reconfigure(&|s| {
                s.set_baud_rate(settings.baud_rate)?;
                s.set_char_size(settings.char_size);
                s.set_parity(settings.parity);
                s.set_stop_bits(settings.stop_bits);
                s.set_flow_control(settings.flow_control);
                Ok(())
            })
            .ok();
        Some(port)
    }

    fn set(&mut self, value: String) -> Result<(), Error> {
        self.0
            .write_all(value.as_bytes())
            .map_err(|_| Error::BrokenLimb)
    }

    fn get(&mut self) -> Result<String, Error> {
        let mut bytes = Vec::new();
        let _ = self.0.read_to_end(&mut bytes);
        // ^ Returns error when reaching EOF for some reason. For now, just
        // ignore the error and return partial/empty result below.
        String::from_utf8(bytes)
            .map_err(|_| Error::BrokenLimb)
    }
}

fn port_settings_from_json(config: &json::Value) -> Option<serial::PortSettings> {
    let baud_rate = match &config["baud-rate"] {
        json::Value::Number(n) => Some(match n.as_u64()? {
            110 => serial::BaudRate::Baud110,
            300 => serial::BaudRate::Baud300,
            600 => serial::BaudRate::Baud600,
            1200 => serial::BaudRate::Baud1200,
            2400 => serial::BaudRate::Baud2400,
            4800 => serial::BaudRate::Baud4800,
            9600 => serial::BaudRate::Baud9600,
            19200 => serial::BaudRate::Baud19200,
            38400 => serial::BaudRate::Baud38400,
            57600 => serial::BaudRate::Baud57600,
            115200 => serial::BaudRate::Baud115200,
            m => serial::BaudRate::BaudOther(m.try_into().ok()?),
        }),
        _ => None,
    }?;
    let char_size = match &config["char-size"] {
        json::Value::Number(n) => match n.as_u64()? {
            5 => Some(serial::CharSize::Bits5),
            6 => Some(serial::CharSize::Bits6),
            7 => Some(serial::CharSize::Bits7),
            8 => Some(serial::CharSize::Bits8),
            _ => None,
        },
        _ => None,
    }?;
    let parity = match &config["parity"] {
        json::Value::String(s) => match s.as_ref() {
            "none" => Some(serial::Parity::ParityNone),
            "odd" => Some(serial::Parity::ParityOdd),
            "even" => Some(serial::Parity::ParityEven),
            _ => None,
        },
        _ => None,
    }?;
    let stop_bits = match &config["stop-bits"] {
        json::Value::Number(n) => match n.as_u64()? {
            1 => Some(serial::StopBits::Stop1),
            2 => Some(serial::StopBits::Stop2),
            _ => None,
        },
        _ => None,
    }?;
    let flow_control = match &config["flow-control"] {
        json::Value::String(s) => match s.as_ref() {
            "none" => Some(serial::FlowControl::FlowNone),
            "software" => Some(serial::FlowControl::FlowSoftware),
            "hardware" => Some(serial::FlowControl::FlowHardware),
            _ => None,
        },
        _ => None,
    }?;
    Some(serial::PortSettings {
        baud_rate: baud_rate,
        char_size: char_size,
        parity: parity,
        stop_bits: stop_bits,
        flow_control: flow_control,
    })
}
