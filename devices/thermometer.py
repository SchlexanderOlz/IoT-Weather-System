from devices import Device
from typing import Dict
import random
import time
from datetime import datetime


class Thermometer(Device):
    def __init__(self, host: str, port: int, device_name: str) -> None:
        super().__init__(host, port, device_name)

    def _gather_data(self) -> Dict[str, float]:
        factor: float = ((time.time() % 1_000) / 80) + time.time() / 1_000_000_000
        data: int = random.uniform(factor + 10.0, 12 + factor)
        return {"sensor_id" : self.uuid,
                "temperature" : data,
                "timestamp" : datetime.now().strftime('%Y-%m-%d %H:%M:%S.%f')[:-3]
                }


if __name__ == "__main__":
    therm = Thermometer('127.0.0.1', 3000, 'my_thermometer')
    therm.run()