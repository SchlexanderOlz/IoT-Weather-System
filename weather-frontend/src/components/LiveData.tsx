"use client";
import { SensorData } from "@/types";
import React, { useEffect, useState } from "react";

const LiveData = ({ device_name }: { device_name: string }) => {
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
    <div className="flex flex-col items-center">
      <h1 className="text-4xl font-bold mb-4">Live Data</h1>
      <div className="bg-blue-400 rounded-lg p-6 m-2 shadow-md w-full max-w-md text-white">
        {data.length > 0 ? (
          <div>
            <div className="flex justify-between items-center mb-4">
              <p className="text-xl font-bold">Sensor:</p>
              <p className="text-xl">{data.at(-1)!.sensor_id}</p>
            </div>
            <div className="flex flex-col ">
              <div className="flex justify-between items-center mb-4">
                <p className="text-sm font-bold">Temperature:</p>
                <p className="text-sm">{data.at(-1)!.temperature}°C</p>
              </div>
              <div className="flex justify-between items-center mb-4">
                <p className="text-sm font-bold">Humidity:</p>
                <p className="text-sm">{data.at(-1)!.humidity}%</p>
              </div>
            </div>
          </div>
        ) : (
          <p className="text-lg">Loading....</p>
        )}
      </div>
    </div>
  );
};

export default LiveData;
