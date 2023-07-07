import React, { useState, useEffect } from 'react';

const App = () => {
  const [sensorData, setSensorData] = useState(null);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:3030/ws/');
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setSensorData(data);
    };
    return () => ws.close();
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        {sensorData ? (
          <div>
            <p>Sensor ID: {sensorData.sensor_id}</p>
            <p>Timestamp: {sensorData.timestamp}</p>
            <p>Temperature: {sensorData.temperature}</p>
            <p>Humidity: {sensorData.humidity}</p>
            <p>Light Level: {sensorData.light_level}</p>
            <p>Pressure: {sensorData.pressure}</p>
          </div>
        ) : (
          <p>Loading...</p>
        )}
      </header>
    </div>
  );
};

export default App;
