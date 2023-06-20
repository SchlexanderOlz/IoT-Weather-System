from typing import Dict, Any
from devices import Device
import random
import time


class B7490(Device):
    def __init__(self, device_name: str) -> None:
        super().__init__(device_name)
    
    def _gather_data(self) -> Dict[str, Any]:
        base_data: dict[str, any] = super()._gather_data()

        factor: float = ((time.time() % 1_000) / 80) + time.time() / 1_000_000_000
        temperature: float = random.uniform(factor + 10.0, 12 + factor)
        humidity: float = random.uniform(0 + factor * 3, (100 - temperature / 5) - factor * 3)
        light_level: float = random.uniform(100 + factor * 50, 80_000 + factor * 30 + temperature * 100)
        
        base_data.update({"humidity" : humidity, "temperature" : temperature, "light_level" : light_level})
        return base_data