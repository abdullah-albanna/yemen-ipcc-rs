import React, { useEffect, useState } from "react";
import "./DisconnectedScreen.css";
import { useDeviceContext } from "../DeviceInfo/DeviceProvider";

const DisconnectedScreen = ({ children }) => {
  let { connected } = useDeviceContext();

  const handleRetry = () => {
    window.location.reload();
  };

  return (
    <div className="disconnected-screen">
      {children}
      {connected || (
        <div className="overlay">
          <div className="message">
            <h1>Device Not Detected</h1>
            <p>Ensure your iPhone is connected</p>
          </div>
          <button className="retry-button" onClick={handleRetry}>
            Retry
          </button>
        </div>
      )}
    </div>
  );
};

export default DisconnectedScreen;
