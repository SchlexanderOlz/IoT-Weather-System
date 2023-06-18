from colorama import Fore, Style
from typing import Tuple
from data_processing import DataProcesser
import socket
import threading
import ssl
import os
import json


HOST = '127.0.0.1'
PORT = 3000
ADDRESS = (HOST, PORT)


class Server:
    def __init__(self) -> None:
        self.__server: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.__server.bind(ADDRESS)
        self.__server.listen()

        self.__context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        self.__context.load_cert_chain('cert.pem', 'key.pem')
        
        self.__processer = DataProcesser()


    def __handle_client(self, client_socket: socket.socket, client_address: Tuple[str, int]):
        client_socket = self.__context.wrap_socket(client_socket, server_side=True)
        while True:
            buff: bytes = client_socket.recv(1024)
            if not buff:
                break
            if not self.__processer.insert(json.loads(buff.decode())):
                print(f"""[-] Insert of Data failed for set {Fore.CYAN}{buff.decode()}{Style.RESET_ALL}""")
            print(buff.decode())
        client_socket.close()
        print(f"""[*] Connection closed from {Fore.BLUE}IP: {client_address[0]} | Port: {client_address[1]}{Style.RESET_ALL}""")


    def start(self):
        self.__processer.create()
        self.__server.settimeout(0.5)
        try:
            while True:
                try:
                    client_socket, client_address = self.__server.accept()
                    print(f"""[*] New connection from {Fore.BLUE}IP: {client_address[0]} | Port: {client_address[1]}{Style.RESET_ALL}""")

                    client_thread = threading.Thread(target=self.__handle_client, args=(client_socket, client_address))
                    client_thread.start()
                except socket.timeout:
                    continue
        except KeyboardInterrupt:
            self.__server.close()
            os._exit(0)


if __name__ == "__main__":
    serv = Server()
    serv.start()
