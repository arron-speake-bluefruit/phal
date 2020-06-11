"""
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2020 Callum David O'Brien
"""

import pytest
import requests
import sys

IP_A = None
IP_B = None

def addr(ip):
    return "http://" + ip + ":8000"

def set_config(ip, config):
    response = requests.post(addr(ip) + "/config", data = config)
    if response.status_code != 200:
        exit(1)

def get_pin(ip, pin_name):
    response = requests.get(addr(ip) + "/limb/" + pin_name)
    if response.status_code != 200:
        exit(1)
    return response.text == "High"

def set_pin(ip, pin_name, state):
    response = requests.post(
        addr(ip) + "/limb/" + pin_name,
        data = "High" if state else "Low")
    if response.status_code != 200:
        exit(1)

def get_serial(ip, serial_name):
    response = requests.get(addr(ip) + "/limb/" + serial_name)
    if response.status_code != 200:
        exit(1)
    return response.text

def send_serial(ip, serial_name, message):
    response = requests.post(
        addr(ip) + "/limb/" + serial_name,
        data = message)
    if response.status_code != 200:
        exit(1)

class TestPin:
    def setup_method(self, test_method):
        config = open("pin-config.json").read()
        set_config(IP_A, config)
        set_config(IP_B, config)

    def test_setting_pin_high_then_low_then_high(self):
        set_pin(IP_A, "output-pin", True)
        assert get_pin(IP_B, "input-pin")
        set_pin(IP_A, "output-pin", False)
        assert not get_pin(IP_B, "input-pin")
        set_pin(IP_A, "output-pin", True)
        assert get_pin(IP_B, "input-pin")

class TestSerial:
    def setup_method(self, test_method):
        config = open("serial-config.json").read()
        set_config(IP_A, config)
        set_config(IP_B, config)

    def test_sending_and_receiving_message(self):
        msg = "foo"
        send_serial(IP_A, "uart", msg)
        assert get_serial(IP_B, "uart") == msg
