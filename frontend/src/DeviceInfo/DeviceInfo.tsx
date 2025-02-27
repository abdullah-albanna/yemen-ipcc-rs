import "./DeviceInfo.css";

import Battery from "./DeviceInfoBoxes/Battery";
import OS from "./DeviceInfoBoxes/OS";
import Info from "./DeviceInfoBoxes/Info";
import Storage from "./DeviceInfoBoxes/Storage";
import React from "react";

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
