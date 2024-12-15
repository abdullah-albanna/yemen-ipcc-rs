import "./DeviceAll.css";

import { Box } from "@mui/material";

export default function OS() {
  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>OS</legend>
        <p>iOS: 18.1</p>
        <p>Build Number: 23H3d</p>
      </Box>
    </div>
  );
}
