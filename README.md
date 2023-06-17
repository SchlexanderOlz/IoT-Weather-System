# IoT-Cassandra
An example usage for the Cassandra database


docker run --name cassandra-node1 -p 9042:9042 -d cassandra
docker run --name cassandra-node2 -p 9043:9042 --link cassandra-node1:cassandra -d cassandra
