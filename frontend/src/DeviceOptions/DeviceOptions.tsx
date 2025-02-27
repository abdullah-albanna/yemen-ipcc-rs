import { ThemeProvider } from "@emotion/react";
import "./DeviceOptions.css";
import Bundles from "./Options/Bundles";
import Containers from "./Options/Containers";
import theme from "./Options/Theme";

import React from "react";
import StartButton from "./Button/StartButton";

const DeviceOptions: React.FC = () => {
  return (
    <div className="device-options-container">
      <div className="device-title">
        <p className="device-title-text"> Device Options </p>
      </div>
      <ThemeProvider theme={theme}>
        <Containers />
        <Bundles />
      </ThemeProvider>
      <div className="device-options-button-elements-container">
        <StartButton />
      </div>
    </div>
  );
};

export default DeviceOptions;
