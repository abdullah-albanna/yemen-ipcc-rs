import "./DeviceAll.css";

import { Box } from "@mui/material";

export default function Storage() {
  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>Storage</legend>
        <p>Total Storage: 256GB</p>
        <p>Used Storage: 60GB</p>
        <p>Availabel Storage: 194GB </p>
      </Box>
    </div>
  );
}
