# IoT Weather App

An Iot-System which is used to display live data

docker pull cassandra
docker run --name IoT-Node0 -p 9042:9042 -d cassandra
docker run --name IoT-Node1 -p 9043:9042 --link IoT-Node0:cassandra -d cassandra
