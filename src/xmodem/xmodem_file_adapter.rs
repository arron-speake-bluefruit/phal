// Copyright (C) 2020 Arron Speake
use super::packet::{Packet, PAYLOAD_SIZE};

use std::{
    fs::File,
    io::Read,
};

pub struct XModemFileAdapter {
    file: File,
    block: u8,
    reached_eof: bool,
}

impl XModemFileAdapter {
    pub fn new(file: File) -> Self {
        Self {
            file,
            block: 0,
            reached_eof: false,
        }
    }

    pub fn get_next_packet(&mut self) -> Option<Packet> {
        // TODO: Handle panic at (block == 255).
        self.block += 1;

        let mut buffer = [0u8; PAYLOAD_SIZE];
        let result = self.file.read(&mut buffer);
        match result {
            Ok(0) => None,
            Ok(bytes_read) => {
                Some(Packet::new(
                    self.block,
                    &buffer[0..bytes_read],
                ))
            },
            Err(_) => None,
        }
    }
}

impl Iterator for XModemFileAdapter {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_eof { return None; }

        match self.get_next_packet() {
            Some(p) => Some(p),
            None => {
                self.reached_eof = true;
                Some(Packet::Terminal)
            },
        }
    }
}