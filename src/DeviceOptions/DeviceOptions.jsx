import { ThemeProvider } from "@emotion/react";
import "./DeviceOptions.css";
import Bundles from "./Options/Bundles";
import Containers from "./Options/Containers";
import theme from "./Options/Theme";

export default function DeviceOptions() {
  return (
    <div className="device-options-container">
      <div className="device-title">
        <p className="device-title-text"> Device Options </p>
      </div>
      <ThemeProvider theme={theme}>
        <Containers />
        <Bundles />
      </ThemeProvider>
    </div>
  );
}
