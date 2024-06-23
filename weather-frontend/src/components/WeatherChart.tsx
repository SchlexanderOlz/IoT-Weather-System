import { SensorData } from "@/types";
import { useEffect, useState } from "react";
import { CartesianGrid, Line, LineChart, Tooltip, XAxis, YAxis } from "recharts";

const WeatherChart = () => {
    const [data, setData] = useState<SensorData[]>([]);

    const fetchWeatherData = async () => {
        const endTime = new Date(Date.now());
        const beginTime = new Date(endTime.getTime() - 60 * 60 * 1000);

        const response = await fetch(
            (process.env.NEXT_PUBLIC_SERVER_URL as string) + 
            `/weather?device=MockDevice&begin=${beginTime.toISOString()}&end=${endTime.toISOString()}`
        );
        return await response.json();
    }

    
    useEffect(() => {
        fetchWeatherData().then((data) => setData(data.map((x: any) => ({ ...x, timestamp_formated: 
    Intl.DateTimeFormat('en-US', { dateStyle: 'full', timeStyle: 'long' }).format(new Date(
  x.timestamp)) }))));
    }, []);
    
    return (
        <div>
        <h2>Weather over the past hour</h2>
        <LineChart
          width={600}
          height={300}
          data={data}
          margin={{ top: 5, right: 20, bottom: 5, left: 0 }}
        >
          <Line type="monotone" dataKey="temperature" stroke="#8884d8" />
          <CartesianGrid stroke="#ccc" strokeDasharray="5 5" />
          <XAxis dataKey="timestamp_formated" />
          <YAxis dataKey="temperature" />
          <Tooltip />
        </LineChart>
        </div>
    );
    }

export default WeatherChart;