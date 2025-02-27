import React, { useState } from "react";
import Box from "@mui/material/Box";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";

import "./DropDown.css";

const Bundles: React.FC = () => {
  const [bundle, setBundle] = useState<string>("");

  const handleChange = (event) => {
    setBundle(event.target.value);
  };

  return (
    <Box className="bundles-container">
      <FormControl fullWidth>
        <InputLabel id="bundles-dropdown-label">Bundles</InputLabel>
        <Select
          labelId="bundles-dropdown-label"
          id="bundles-dropdown-select"
          value={bundle}
          label="Age"
          onChange={handleChange}
          className="bundles-select"
        >
          <MenuItem value={"CellularSouthLTE"}>CellularSouthLTE</MenuItem>
          <MenuItem value={"CWW"}>CWW</MenuItem>
          <MenuItem value={"China"}>China</MenuItem>
        </Select>
      </FormControl>
    </Box>
  );
};

export default Bundles;
