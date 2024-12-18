import "./DeviceInfo.css";

import React, { useEffect, useState } from "react";
import Box from "@mui/material/Box";

import Battery from "./DeviceInfoBoxes/Battery";
import OS from "./DeviceInfoBoxes/OS";
import Info from "./DeviceInfoBoxes/Info";
import Storage from "./DeviceInfoBoxes/Storage";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useDeviceContext } from "./DeviceProvider";

const DeviceInfo: React.FC = () => {
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
};

export default DeviceInfo;
