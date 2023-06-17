from device import Device


class Thermometer(Device):
    def __init__(self, host: str, port: int) -> None:
        super().__init__(host, port)

    def _send_data(self):
        self._client_socket.send(input().encode())


if __name__ == "__main__":
    therm = Thermometer('127.0.0.1', 3000)
    therm._send_data()