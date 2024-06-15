import { SensorData } from "@/types";
import React, { useEffect, useState } from "react";

const LiveData = (device_name: string) => {
  const [data, setData] = useState<SensorData[]>([]);

  useEffect(() => {
    const ws = new WebSocket(
      (process.env.NEXT_PUBLIC_WEATHER_SOCKET_URL as string) + device_name
    );

    ws.onmessage = (event) => {
      const receivedData = JSON.parse(event.data);
      setData((prevData) => [...prevData, receivedData]);
    };

    ws.onclose = () => {
      console.log("Connection closed");
    };

    return () => {
      ws.close();
    };
  }, [device_name]);

  return (
    <div className="flex flex-col items-center justify-center">
      {data.map((sensorData, index) => (
        <div
          key={index}
          className="bg-gray-100 rounded-lg p-4 m-2 shadow-md w-full max-w-md"
        >
          <p className="text-lg font-semibold">Sensor Data</p>
          <div className="grid grid-cols-2 gap-4">
            <p className="text-sm font-medium">Temperature:</p>
            <p className="text-sm">{sensorData.temperature}°C</p>
            <p className="text-sm font-medium">Humidity:</p>
            <p className="text-sm">{sensorData.humidity}%</p>
            {/* Add more sensor data fields here */}
          </div>
        </div>
      ))}
    </div>
  );
};

export default LiveData;
