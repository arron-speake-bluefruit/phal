use serde_json as json;
use std::convert::TryInto;

/// Shared JSON settings import for XModem and Serial
pub fn port_settings_from_json(config: &json::Value) -> Option<serial::PortSettings> {
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
        baud_rate,
        char_size,
        parity,
        stop_bits,
        flow_control,
    })
}
