import React, { useState } from "react";
import { Button } from "@mui/material";
import startChecking from "./installation_check";
import startInstalling from "./installation_result";

const StartButton: React.FC = () => {
  const [loading, setLoading] = useState<boolean>(false); // To manage loading state
  const [buttonText, setButtonText] = useState<string>("Start");
  const [buttonColor, setButtonColor] = useState<string>("black");

  const resetButtonTextDelayed = (): void => {
    // Reset button text after a short delay
    setTimeout((): void => {
      setButtonColor("black");
      setButtonText("Start");
      setLoading(false);
    }, 3000);
  };

  const handleButtonClick = async (): Promise<void> => {
    setLoading(true);
    setButtonColor("white");

    // if the installation fails, reset the button text and stop the process
    if (!(await startInstalling(setButtonColor, setButtonText, setLoading))) {
      resetButtonTextDelayed();
      return;
    }
    await startChecking(setButtonColor, setButtonText, setLoading);

    resetButtonTextDelayed();
  };

  return (
    <div>
      <Button
        variant="contained"
        sx={{
          backgroundColor: "#8AB0AB",
          borderRadius: "12px",
          padding: "10px 20px",
          textAlign: "center",
          "&:hover": {
            backgroundColor: "#8AC0AB",
          },
        }}
        // The color of the button is kept in here so it would be dynamic
        style={{ color: buttonColor }}
        onClick={handleButtonClick}
        disabled={loading} // Disable the button while loading
      >
        {buttonText}
      </Button>
    </div>
  );
};

export default StartButton;
