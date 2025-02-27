import { useDeviceContext } from "../DeviceProvider";
import "./DeviceAll.css";

import React from "react";
import { Box } from "@mui/material";

const Battery: React.FC = () => {
  let { battery } = useDeviceContext();

  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>Battery</legend>
        <p>Battery Level: {battery?.battery_level}%</p>
        <p>Battery Health: {battery?.battery_health}%</p>
        <p>Cycle Count: {battery?.cycle_counts}</p>
      </Box>
    </div>
  );
};

export default Battery;
