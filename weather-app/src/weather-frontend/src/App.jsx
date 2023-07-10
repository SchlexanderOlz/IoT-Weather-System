import React, { useState, useEffect } from 'react';
import "./App.css"

const App = () => {
  const [sensorData, setSensorData] = useState(null);

  useEffect(() => {
    const ws = new WebSocket('ws://192.168.8.181:3030/ws/');
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setSensorData(data);
    };
    return () => ws.close();
  }, []);

  return (
    <div className="App">
      <div id="current_container">
        <div id="temperature">
          {sensorData ? (
              <p>Temperature: {sensorData.temperature}°C</p>
          ) : (
              <p>Loading...</p>
          )}
        </div>
        <hr></hr>
        <div class="less_important">
          {sensorData ? (
            <div class="value_container">
              <p class="describer_text">Humidity: </p>
              <p>{sensorData.humidity}%</p>
            </div>
          ) : (
            <p>Loading...</p>
          )}
        </div>
        <hr></hr>
        <div class="less_important">
          {sensorData ? (
            <div class="value_container">
              <p class="describer_text">Pressure: </p>
              <p>{sensorData.pressure} bar</p>
            </div>
          ) : (
            <p>Loading...</p>
          )}
        </div>
        <hr></hr>
        <div class="less_important">
          {sensorData ? (
            <div class="value_container">
              <p class="describer_text">Light-Level: </p>
              <p>{sensorData.light_level}</p>
            </div>
          ) : (
            <p>Loading...</p>
          )}
        </div>

      </div>
    </div>
  );
  
};

export default App;