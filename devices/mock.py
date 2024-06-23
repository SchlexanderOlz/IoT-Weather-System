import struct
import socket
import random


DEVICE_NAME = "MockDevice"
HOST_ADDRESS = "10.10.0.26"
HOST_PORT = 7000

def get_temperature():
    return (10 + random.randint(0, 20), 75.0)

def get_light():
    return random.randint(0, 1000)

def get_pressure():
    return random.randint(9500, 10500) 

def gather_data():
    bytes_list = bytearray([0x1, 0x1])
    bytes_list += DEVICE_NAME.encode('utf-8')
    
    temperature = get_temperature()
    bytes_list += bytearray([0x2])
    bytes_list += struct.pack('f', temperature[0])  # 'f' is the format for a float in struct
    bytes_list += bytearray([0x3])
    bytes_list += bytearray([int(temperature[1])])  # Assuming second part of temperature is an integer
    
    light_level = get_light()
    bytes_list += bytearray([0x4])
    bytes_list += struct.pack('f', light_level)
    
    pressure = get_pressure()
    bytes_list += bytearray([0x5])
    bytes_list += struct.pack('I', pressure)  # 'I' is the format for an unsigned int in struct
    
    return bytes_list


if __name__ == "__main__":
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((HOST_ADDRESS, HOST_PORT))
        while True:
            sock.sendall(gather_data())