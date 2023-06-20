from cassandra.cluster import Cluster, ResultSet, Session
from cassandra.policies import DCAwareRoundRobinPolicy
from cassandra import ConsistencyLevel
from typing import List, Any, Dict


class DataProcesser:
    def __init__(self) -> None:
        cluster = Cluster(
            contact_points=[('localhost', 9042), ('localhost', 9043)],
            load_balancing_policy=DCAwareRoundRobinPolicy(local_dc='datacenter1'),
            protocol_version=4
        )
        self.__session: Session = cluster.connect()
        self.__session.default_consistency_level = ConsistencyLevel.ONE
        self.create()
    
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

    def get_average_humidity(self, start: str, end: str) -> float:
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
            return -1.0
        return result.average_humidity


    def get_average_temperature(self, start: str, end: str) -> float:
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
            return -256
        return result.average_temperature


    def get_average_light_level(self, start: str, end: str) -> float:
        query = """
                SELECT AVG(light_level) AS average_light_level
                FROM IoT_Example.SensorData
                WHERE timestamp >= %s
                AND timestamp <= %s
                ALLOW FILTERING
                """
        try:
            result = self.__session.execute(query, (start, end)).one()
        except Exception as e:
            print(e)
            return -1.0
        return result.average_light_level


    def get_average_data_per_day(self, start: str, end: str) -> List[Dict[str, Any]]:
        query = """
                SELECT sensor_id, toDate(timestamp) as date,
                    AVG(temperature) AS average_temperature,
                    AVG(humidity) AS average_humidity,
                    AVG(light_level) AS average_light_level
                FROM IoT_Example.SensorData
                WHERE timestamp >= %s
                 AND timestamp <= %s
                GROUP BY sensor_id, toDate(timestamp)
                ALLOW FILTERING
                """
        try:
            result = self.__session.execute(query, (start, end)).all()
        except Exception as e:
            print(e)
            return []
        return result

    def get_most_recent_data(self, sensor_id: str) -> Dict[str, Any]:
        query = """
                SELECT *
                FROM IoT_Example.SensorData
                WHERE sensor_id = %s
                ORDER BY timestamp DESC
                LIMIT 1
                """
        try:
            result = self.__session.execute(query, (sensor_id,)).one()
        except Exception as e:
            print(e)
            return None
        return result
