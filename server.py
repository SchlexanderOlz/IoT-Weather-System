from typing import Any
from cassandra.cluster import Cluster
from cassandra.policies import DCAwareRoundRobinPolicy
from colorama import Fore, Style
import socket
import threading
import ssl
import os


CLUSTER = Cluster(
    ['localhost'],
    port=9042,
    load_balancing_policy=DCAwareRoundRobinPolicy(local_dc='datacenter1'),
    protocol_version=4
)
HOST = '127.0.0.1'
PORT = 3000
ADDRESS = (HOST, PORT)


class Server:
    
    def __init__(self) -> None:
        self._server: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._server.bind(ADDRESS)
        self._server.listen()

        self._context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        self._context.load_cert_chain('cert.pem', 'key.pem')

        self._session = CLUSTER.connect()


    def _handle_client(self, client_socket: socket.socket, client_address: tuple[str, int]):
        client_socket = self._context.wrap_socket(client_socket, server_side=True)
        while True:
            buff: bytes = client_socket.recv(1024)
            if not buff:
                break
            print(buff.decode())
        client_socket.close()
        print(f"""[*] Connection closed from {Fore.BLUE}IP: {client_address[0]} | Port: {client_address[1]}{Style.RESET_ALL}""")

    def start(self):
        self._server.settimeout(0.5)
        try:
            while True:
                try:
                    client_socket, client_address = self._server.accept()
                    print(f"""[*] New connection from {Fore.BLUE}IP: {client_address[0]} | Port: {client_address[1]}{Style.RESET_ALL}""")

                    client_thread = threading.Thread(target=self._handle_client, args=(client_socket, client_address))
                    client_thread.start()
                except socket.timeout:
                    continue
        except KeyboardInterrupt:
            self._server.close()
            os._exit(0)


if __name__ == "__main__":
    serv = Server()
    serv.start()
