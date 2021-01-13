// Copyright (C) 2020 Arron Speake
mod packet;
mod xmodem_file_adapter;

use crate::{
    limb::{Error, Limb},
    xmodem::packet::Packet,
    xmodem::xmodem_file_adapter::XModemFileAdapter,
    port_settings_from_json::port_settings_from_json,
    system_serial::SerialPort,
};
use std::{
    fs::File,
    io::{Read, Write},
    thread::sleep,
    time::{Duration, Instant},
};
use serde_json as json;

pub struct XModem {
    port: serial::SystemPort,
    last_status: Option<bool>,
}

impl XModem {
    fn write(&mut self, packet: &Packet) -> Result<(), Error> {
        self.port.write_all(packet.data())
            .map_err(|_| Error::WriteFailed)
    }

    fn read(&mut self) -> Option<bool> {
        const ACKNOWLEDGE : u8 = 0x06;
        const NEGATIVE_ACKNOWLEDGE : u8 = 0x15;
        let mut read_buffer = [0u8; 1];
        self.port.read_exact(&mut read_buffer).ok()?;
        match read_buffer[0] {
            ACKNOWLEDGE => Some(true),
            NEGATIVE_ACKNOWLEDGE => Some(false),
            _ => None,
        }
    }

    fn wait_for_response(&mut self) -> Result<bool, Error> {
        const TIMEOUT : Duration = Duration::from_secs(10);
        const DELAY : Duration = Duration::from_millis(500);
        let timeout_point = Instant::now() + TIMEOUT;

        while Instant::now() < timeout_point {
            let read = self.read();
            if read.is_some() {
                return read.ok_or(Error::ReadFailed);
            }
            sleep(DELAY);
        }

        Err(Error::Timeout)
    }

    fn wait_for_negative_acknowledge(&mut self) -> Result<(), Error> {
        match self.wait_for_response()? {
            false => Ok(()),
            true => Err(Error::ReadFailed),
        }
    }
}

impl Limb for XModem {
    fn from_json(config: &json::Value) -> Option<Self> {
        let device = config["device"].as_str()?;
        let mut port = serial::open(device).ok()?;

        let settings = port_settings_from_json(config)?;
        port.reconfigure(&|s| {
            s.set_baud_rate(settings.baud_rate)?;
            s.set_char_size(settings.char_size);
            s.set_parity(settings.parity);
            s.set_stop_bits(settings.stop_bits);
            s.set_flow_control(settings.flow_control);
            Ok(())
        }).ok()?;

        Some(Self {
            port,
            last_status: None,
        })
    }

    fn set(&mut self, value: String) -> Result<(), Error> {
        let source = File::open(value)
            .map_err(|_| Error::InvalidValue)?;

        self.wait_for_negative_acknowledge()?;
        for packet in XModemFileAdapter::new(source) {
            const MAX_ATTEMPTS : usize = 10;
            let mut exceeded_max_attempts = true;
            'repeat_attempts: for _ in 0..MAX_ATTEMPTS {
                self.write(&packet)?;
                let acknowledged = self.wait_for_response()?;
                if acknowledged {
                    exceeded_max_attempts = false;
                    break 'repeat_attempts;
                }
            }
            if exceeded_max_attempts { return Err(Error::BrokenLimb); }
        }

        Ok(())
    }

    fn get(&mut self) -> Result<String, Error> {
        match self.last_status.take() {
            Some(true) => Ok("Success".to_owned()),
            Some(false) => Ok("Failure".to_owned()),
            None => Err(Error::InvalidOperation),
        }
    }

    fn type_name(&self) -> &'static str { "xmodem" }
}