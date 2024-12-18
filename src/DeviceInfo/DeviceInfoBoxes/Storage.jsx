import { useDeviceContext } from "../DeviceProvider";
import "./DeviceAll.css";

import { Box } from "@mui/material";

export default function Storage() {
  let { storage } = useDeviceContext();

  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>Storage</legend>
        <p>Total Storage: {storage?.total_storage}GB</p>
        <p>Used Storage: {storage?.used_storage}GB</p>
        <p>Available Storage: {storage?.available_storage}GB </p>
      </Box>
    </div>
  );
}
