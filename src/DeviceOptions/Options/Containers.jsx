import * as React from "react";
import Box from "@mui/material/Box";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";

export default function Containers() {
  const [container, setContainer] = React.useState("");

  const handleChange = (event) => {
    setContainer(event.target.value);
  };

  return (
    <Box className="bundles-container">
      <FormControl fullWidth>
        <InputLabel id="containers-dropdown-label">Containers</InputLabel>
        <Select
          labelId="containers-dropdown-label"
          id="containers-dropdown-select"
          value={container}
          label="Containers"
          onChange={handleChange}
          className="bundles-select"
        >
          <MenuItem value={"default"}>Default</MenuItem>
          <MenuItem value={"unknown"}>Unknown</MenuItem>
        </Select>
      </FormControl>
    </Box>
  );
}
