from devices import Thermometer, Device, B7490
import threading


PORT = 3000
HOST = '127.0.0.1'


def run():
    therm_1 = Thermometer('therm_1')
    therm_2 = Thermometer('therm_2')
    
    b7490_1 = B7490('new_device(wow)')
    b7490_2 = B7490('another_devie(wow)')

    devices: list[Device] = [therm_1, therm_2, b7490_1, b7490_2]

    threads: list[threading.Thread] = []
    for target in devices:
        target.connect(HOST, PORT)
        thread = threading.Thread(target=target.run)
        threads.append(thread)

    for thread in threads:
        thread.start()
