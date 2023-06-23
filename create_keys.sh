cd iot-server
mkdir keys 2>/dev/null
cd keys

echo "[*] Creating server keys..."
openssl genrsa -out key.pem 2048
openssl req -new -key key.pem -out csr.pem
openssl x509 -req -days 10000 -in csr.pem -signkey key.pem -out cert.pem

cd ../../devices
mkdir keys 2>/dev/null
cd keys

echo "[*] Creating client keys..."
openssl genrsa -out client_key.pem 2048
openssl req -new -key client_key.pem -out csr.pem
openssl x509 -req -days 10000 -in csr.pem -signkey client_key.pem -out client_cert.pem

cd ../../
cp devices/keys/client_cert.pem iot-server/keys