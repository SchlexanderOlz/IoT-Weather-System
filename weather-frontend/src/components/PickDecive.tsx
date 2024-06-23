import { Device } from "@/types";
import { useState } from "react";

const DevicePicker = ({
  devices,
  setDevice,
}: {
  devices: Device[];
  setDevice: any;
}) => {
  const [selectedDevice, setSelectedDevice] = useState<string>(
    devices[0]?.name || ""
  );

  const handleChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    console.log("Called");
    setSelectedDevice(event.target.value);
    setDevice(devices.find((x) => x.name == event.target.value)!);
  };

  return (
    <div className="p-4 rounded shadow bg-blue-400 text-white">
      <label
        htmlFor="device-picker"
        className="block text-sm font-medium text-gray-700"
      >
        Select a device
      </label>
      <select
        id="device-picker"
        value={selectedDevice}
        onChange={(event) => handleChange(event)}
        className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
      >
        {devices.map((device) => (
          <option key={device.name} value={device.name}>
            {device.name}
          </option>
        ))}
      </select>
    </div>
  );
};

export default DevicePicker;
