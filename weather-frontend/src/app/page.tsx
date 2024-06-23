"use client"
import LiveData from "@/components/LiveData";
import DevicePicker from "@/components/PickDecive";
import { useEffect, useState } from "react";
import { Device } from "@/types";

export default function Home() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [selectedDevice, setSelectedDevice] = useState<Device>({name: "MockDevice"} as Device );

  const fetchDevices = async () => {
    const response = await fetch(
      (process.env.NEXT_PUBLIC_SERVER_URL as string) + "/devices"
    );
    const data = await response.json();
    setDevices(data);
  };

  useEffect(() => {
    fetchDevices();
  }, []);

  return (
    <main>
      <div className="flex flex-col items-center min-h-screen mt-5">
        {selectedDevice ? <LiveData device_name={selectedDevice.name} /> : null}
        <DevicePicker devices={devices} setDevice={setSelectedDevice} />
      </div>
    </main>
  );
}
