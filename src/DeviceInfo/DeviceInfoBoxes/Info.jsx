import "./DeviceAll.css";
import { Box } from "@mui/material";

export default function Info() {
  return (
    <div className="info-container">
      <Box component="fieldset">
        <legend>Info</legend>
        <p>Model: iPhone 13 Pro Max</p>
        <p>Serial Number: 12JnAs</p>
      </Box>
    </div>
  );
}
