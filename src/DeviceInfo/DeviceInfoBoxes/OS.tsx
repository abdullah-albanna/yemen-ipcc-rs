import { useDeviceContext } from "../DeviceProvider";
import "./DeviceAll.css";
import React from "react";

import { Box } from "@mui/material";

const OS: React.FC = () => {
  let { os } = useDeviceContext();

  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>OS</legend>
        <p>iOS: {os?.ios_ver}</p>
        <p>Build Number: {os?.build_num}</p>
      </Box>
    </div>
  );
};

export default OS;
