import "./DeviceInfo.css";

import React from "react";
import Box from "@mui/material/Box";

import Battery from "./DeviceInfoBoxes/Battery";
import OS from "./DeviceInfoBoxes/OS";
import Info from "./DeviceInfoBoxes/Info";
import Storage from "./DeviceInfoBoxes/Storage";

export default function DeviceInfo() {
  return (
    <div className="device-info-container">
      <div className="device-title">
        <p className="device-title-text"> Device Info </p>
      </div>
      <div
        style={{
          display: "grid",
          gridTemplateRows: "repeat(4, 1fr)",
          rowGap: "10px",
          height: "70vh",
        }}
      >
        <Info />
        <OS />
        <Battery />
        <Storage />
      </div>
    </div>
  );
}
