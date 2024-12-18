import { useDeviceContext } from "../DeviceProvider";
import "./DeviceAll.css";
import React from "react";
import { Box } from "@mui/material";

export default function Info() {
  let { hardware } = useDeviceContext();

  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>Info</legend>
        <p>Model: {hardware?.model}</p>
        <p>Model Number: {hardware?.model_number}</p>
        <p>Region: {hardware?.region}</p>
      </Box>
    </div>
  );
}
