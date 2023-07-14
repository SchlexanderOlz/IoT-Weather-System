import React, { useState, useEffect } from "react";
import {
  Box,
  CircularProgress,
  Divider,
  Typography,
  createTheme,
  ThemeProvider,
} from "@mui/material";

const boxElementTheme = createTheme({
  typography: {
    fontSize: 20,
  },
});

const TemperatureBox = () => {
  const [sensorData, setSensorData] = useState(null);

  useEffect(() => {
    const ws = new WebSocket("ws://172.21.214.122:3030/ws/");
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setSensorData(data);
    };
    return () => ws.close();
  }, []);
  return (
    <Box
      sx={{
        p: 7,
        position: "absolute",
        top: 20,
        left: "3%",
        borderRadius: 5,
        boxShadow: 3,
        bgcolor: "background.paper",
        minWidth: "250px",
      }}
    >
      <Typography variant="h6" gutterBottom sx={{ fontSize: "2em" }}>
        {sensorData ? (
          `Temperature: ${sensorData.temperature}°C`
        ) : (
          <CircularProgress />
        )}
      </Typography>
      <Divider />

      <ThemeProvider theme={boxElementTheme}>
        <Typography variant="subtitle1" gutterBottom>
          {sensorData ? (
            `Humidity: ${sensorData.humidity}%`
          ) : (
            <CircularProgress />
          )}
        </Typography>
        <Divider />
        <Typography variant="subtitle1" gutterBottom>
          {sensorData ? (
            `Pressure: ${sensorData.pressure} bar`
          ) : (
            <CircularProgress />
          )}
        </Typography>
        <Divider />
        <Typography variant="subtitle1" gutterBottom>
          {sensorData ? (
            `Light-Level: ${sensorData.light_level} lux`
          ) : (
            <CircularProgress />
          )}
        </Typography>
      </ThemeProvider>
    </Box>
  );
};

export default TemperatureBox;
