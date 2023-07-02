import React, {Component} from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import { w3cwebsocket as W3CWebSocket } from 'websocket';

const root = ReactDOM.createRoot(document.getElementById('root'));

const weather_client = new W3CWebSocket("ws://localhost:8050")

weather_client.onmessage = (message) => {
  const data = JSON.parse(message.data);
  console.log(data);
};

root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

reportWebVitals();
