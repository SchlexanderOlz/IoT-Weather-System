import React, { useState, useEffect } from "react";
import {
  Table,
  TableHead,
  TableRow,
  TableCell,
  TableBody,
  TableContainer,
  Paper,
  CircularProgress,
} from "@mui/material";
import { useMediaQuery } from "react-responsive";

const DeviceList = () => {
  const [devices, setDevices] = useState(null);
  const isSmallScreen = useMediaQuery({ query: "(max-width: 900px)" });
  const isMediumScreen = useMediaQuery({ query: "(max-width: 1400px)" });

  useEffect(() => {
    fetch("/getDevices")
      .then((response) => response.json())
      .then((data) => setDevices(data));
  }, []);

  return (
    <TableContainer
      component={Paper}
      sx={{
        minWidth: isSmallScreen ? 50 : isMediumScreen ? 75 : 100,
        maxWidth: isSmallScreen ? 300 : isMediumScreen ? 500 : 750,
        maxHeight: "15em",
        marginLeft: "250px",
        marginTop: "50px",
        overflow: "auto",
        position: "absolute",
        top: 20,
        left: "25%",
      }}
    >
      <Table
        sx={{
          minWidth: isSmallScreen ? 50 : isMediumScreen ? 75 : 100,
          maxWidth: isSmallScreen ? 250 : isMediumScreen ? 450 : 700,
        }}
        size="small"
        aria-label="device table"
      >
        <TableHead>
          <TableRow>
            <TableCell sx={{ fontSize: "16px" }}>Name</TableCell>
            <TableCell align="right" sx={{ fontSize: "16px" }}>
              LastUseDate
            </TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {devices ? (
            devices.map((row) => (
              <TableRow
                key={row.name}
                sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
              >
                <TableCell component="th" scope="row" sx={{ fontSize: "16px" }}>
                  {row.name}
                </TableCell>
                <TableCell align="right" sx={{ fontSize: "16px" }}>
                  {row.timestamp}
                </TableCell>
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell align="right">
                <CircularProgress />
              </TableCell>
              <TableCell align="right">
                <CircularProgress />
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default DeviceList;
