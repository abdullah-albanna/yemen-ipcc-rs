import "./DeviceAll.css";

import { Box } from "@mui/material";

export default function Battery() {
  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>Battery</legend>
        <p>Battery Level: 80%</p>
        <p>Battery Health: 90%</p>
        <p>Cycle Count: 32</p>
      </Box>
    </div>
  );
}
