import socket
import ssl
from abc import ABC


class Device(ABC):
    def __init__(self, host: str, port: int) -> None:
        super().__init__()
        self._context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
        self._context.check_hostname = False
        self._context.verify_mode = ssl.CERT_NONE

        self._client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._client_socket = self._context.wrap_socket(self._client_socket, server_hostname=host)

        self._server_address = (host, port)
        self._client_socket.connect(self._server_address)

    def _send_data(self):
        pass


if __name__ == "__main__":
    device = Device()