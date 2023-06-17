import socket
import ssl
from abc import ABC
from typing import Dict
import time
import json
import hashlib


class Device(ABC):
    def __init__(self, host: str, port: int, device_name: str) -> None:
        super().__init__()
        self.__server_address = (host, port)

        self.__context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
        self.__context.check_hostname = False
        self.__context.verify_mode = ssl.CERT_NONE

        self.__client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.__client_socket = self.__context.wrap_socket(self.__client_socket, server_hostname=host)

        self.uuid = hashlib.md5(device_name.encode()).hexdigest()


    def __send_data(self, data: Dict[str, str]) -> None:
        data_str = json.dumps(data)
        data_bytes = data_str.encode()
        self.__client_socket.send(data_bytes)

    def _gather_data(self) -> Dict[str, str]:
        pass

    def run(self) -> None:
        self.__client_socket.connect(self.__server_address)
        while True:
            time.sleep(5)
            data: Dict[str, str] = self._gather_data()
            self.__send_data(data)

