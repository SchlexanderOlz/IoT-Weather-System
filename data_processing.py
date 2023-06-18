from cassandra.cluster import Cluster, ResultSet
from cassandra.policies import DCAwareRoundRobinPolicy
from cassandra import ConsistencyLevel
from typing import List, Tuple, Dict


class DataProcesser:
    def __init__(self) -> None:
        cluster = Cluster(
            contact_points=[('localhost', 9042), ('localhost', 9043)],
            load_balancing_policy=DCAwareRoundRobinPolicy(local_dc='datacenter1'),
            protocol_version=4
        )
        self.__session = cluster.connect()
        self.__session.default_consistency_level = ConsistencyLevel.ONE
    
    def create(self) -> None:
        self.__session.execute("""CREATE KEYSPACE IF NOT EXISTS IoT_Example WITH REPLICATION = {
                                    'class': 'SimpleStrategy',
                                    'replication_factor': 3
                                }""")

        self.__session.execute("""CREATE TABLE IF NOT EXISTS IoT_Example.SensorData (
                                sensor_id TEXT,
                                timestamp TIMESTAMP,
                                temperature FLOAT,
                                humidity FLOAT,
                                light_level FLOAT,
                                occupancy BOOLEAN,
                                PRIMARY KEY (sensor_id, timestamp)
                            )""")
    
    def insert(self, data: Dict[str, any]) -> bool:
        try:
            self.__session.execute(f"""
                                INSERT INTO IoT_Example.SensorData ({', '.join(data.keys())})
                                VALUES ({', '.join(["%s" for _ in data])})
                                """, data.values())
        except Exception as e:
            print(e)
            return False
        return True
    
    def get_all(self) -> ResultSet:
        return self.__session.execute("""SELECT * FROM IoT_Example.SensorData""").all()

    def get_average_humidity(self, start: str, end: str):
        query = """
                SELECT AVG(humidity) as average_humidity
                FROM IoT_Example.SensorData
                WHERE timestamp >= %s
                 AND timestamp <= %s
                ALLOW FILTERING
                """
        try:
            result = self.__session.execute(query, (start, end)).one()
        except Exception as e:
            print(e)
        return result.average_humidity


    def get_average_temperature(self, start: str, end: str):
        query = """
                SELECT AVG(temperature) AS average_temperature
                FROM IoT_Example.SensorData
                WHERE timestamp >= %s
                 AND timestamp <= %s
                ALLOW FILTERING
                """
        try:
            result = self.__session.execute(query, (start, end)).one()
        except Exception as e:
            print(e)
        return result.average_temperature

