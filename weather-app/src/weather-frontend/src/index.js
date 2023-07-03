import React, {Component} from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import { w3cwebsocket as W3CWebSocket } from 'websocket';

const root = ReactDOM.createRoot(document.getElementById('root'));


const socket = new WebSocket('ws://192.168.8.181:3030/ws/');
let pingInterval;

socket.onopen = function(event) {
    console.log('WebSocket connection opened');
    pingInterval = setInterval(() => {
        socket.send('ping');
    }, 5000);
};

socket.onmessage = function(event) {
    console.log('WebSocket message received:', event.data);
};

socket.onclose = function(event) {
    console.log('WebSocket connection closed');
};

window.addEventListener('beforeunload', () => {
    socket.send('close');
    clearInterval(pingInterval);
});


root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

reportWebVitals();
