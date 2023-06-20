from devices import Device
from typing import Dict, Any
import random
import time


class Thermometer(Device):
    def __init__(self, device_name: str) -> None:
        super().__init__(device_name)

    def _gather_data(self) -> Dict[str, Any]:
        factor: float = ((time.time() % 1_000) / 80) + time.time() / 1_000_000_000
        data: int = random.uniform(factor + 10.0, 12 + factor)

        joined_data: dict[str, any] = super()._gather_data()
        joined_data.update({ "temperature" : data })
        return joined_data


if __name__ == "__main__":
    therm = Thermometer('127.0.0.1', 3000, 'my_thermometer')
    therm.run()