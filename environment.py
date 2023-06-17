from devices import Thermometer
from devices import Device
import threading
import time
import os


PORT = 3000
HOST = '127.0.0.1'


def run():
    therm_1 = Thermometer(HOST, PORT, 'therm_1')
    therm_2 = Thermometer(HOST, PORT, 'therm_2')

    devices: list[Device] = [therm_1, therm_2]

    threads: list[threading.Thread] = []
    for target in devices:
        thread = threading.Thread(target=target.run)
        threads.append(thread)

    for thread in threads:
        thread.start()
