export interface SensorData {
    sensor_id: string;
    temperature: number;
    humidity: number;
    pressure: number;
    timestamp: string;
}

export interface Device {
    name: string;
    timestamp: string;
}