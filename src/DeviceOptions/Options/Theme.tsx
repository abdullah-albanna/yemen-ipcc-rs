import { createTheme } from "@mui/material";

const theme = createTheme({
  components: {
    MuiSelect: {
      styleOverrides: {
        // Styles for the select box itself
        select: {
          backgroundColor: "#8AB0AB", // Background of the select box
          color: "black", // Text color
          borderRadius: "10px", // Rounded corners
          padding: "8px 12px", // Padding inside the select
          "&:hover": {
            backgroundColor: "#9AB0AB", // Hover effect for the select
          },
        },
        // Dropdown arrow styles
        icon: {
          color: "#26413C", // Color of the dropdown arrow
          fontSize: "2rem", // Arrow size
        },
      },
    },
    MuiOutlinedInput: {
      styleOverrides: {
        root: {
          "& .MuiOutlinedInput-notchedOutline": {
            borderColor: "", // Border color of the input
          },
          "&:hover .MuiOutlinedInput-notchedOutline": {
            borderColor: "lightgoldenrodyellow", // Border color on hover
            borderRadius: 10,
          },
          "&.Mui-focused .MuiOutlinedInput-notchedOutline": {
            borderColor: "pink", // Border color on focus
            borderRadius: 10,
          },
        },
      },
    },
    MuiMenu: {
      styleOverrides: {
        paper: {
          backgroundColor: "#3E505B", // Background of dropdown
          borderRadius: "8px", // Rounded corners
        },
        list: {
          padding: "5px", // Padding for dropdown menu
        },
      },
    },
    MuiMenuItem: {
      styleOverrides: {
        root: {
          fontSize: "1rem", // Font size for menu items
          color: "lightgoldenrodyellow", // Text color for items
          padding: "10px 16px", // Padding for each menu item
          backgroundColor: "#26413C",
          borderRadius: 5,
          "&:hover": {
            backgroundColor: "#8AB0AB", // Hover background for items
            color: "black", // Hover text color
          },
          "&.Mui-selected": {
            backgroundColor: "#8AB0AB", // Background when selected
            color: "black", // Text color when selected
          },
          "&.Mui-selected:hover": {
            backgroundColor: "#8AC0AB", // Background when selected and hovered
          },
        },
      },
    },
  },
});

export default theme;
