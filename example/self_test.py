"""
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2020 Callum David O'Brien
"""

import pytest
import requests

MASTER = "http://192.168.3.95:8000"
SLAVE = "http://192.168.3.110:8000"

def get_pin(ip, pin_name):
    response = requests.get(ip + "/limb/" + pin_name)
    if response.status_code != 200:
        exit(1)
    return response.text == "High"

def set_pin(ip, pin_name, state):
    response = requests.post(
        ip + "/limb/" + pin_name,
        data = "High" if state else "Low")
    if response.status_code != 200:
        exit(1)

def get_serial(ip, serial_name):
    response = requests.get(ip + "/limb/" + serial_name)
    if response.status_code != 200:
        exit(1)
    return response.text

def send_serial(ip, serial_name, message):
    response = requests.post(
        ip + "/limb/" + serial_name,
        data = message)
    if response.status_code != 200:
        exit(1)

class TestPin:
    def test_setting_pin(self):
        set_pin(SLAVE, "pin1", True)
        assert get_pin(MASTER, "pin1")
        
        set_pin(SLAVE, "pin1", False)
        assert not get_pin(MASTER, "pin1")

    def test_getting_pin(self):
        set_pin(MASTER, "pin2", True)
        assert get_pin(SLAVE, "pin2")
        
        set_pin(MASTER, "pin2", False)
        assert not get_pin(SLAVE, "pin2")

class TestSerial:
    def test_sending_message(self):
        msg = "ping"
        send_serial(SLAVE, "uart", msg)
        assert get_serial(MASTER, "uart") == msg

    def test_receive_message(self):
        msg = "pong"
        send_serial(MASTER, "uart", msg)
        assert get_serial(SLAVE, "uart") == msg
