import React, { useState } from "react";
import { Button } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import startChecking from "./installation_check";
import startInstalling from "./installation_result";

const StartButton: React.FC = () => {
  const [loading, setLoading] = useState(false); // To manage loading state
  const [buttonText, setButtonText] = useState<string>("Start");
  const [buttonColor, setButtonColor] = useState<string>("black");

  const resetButtonTextDelayed = () => {
    // Reset button text after a short delay
    setTimeout(() => {
      setButtonColor("black");
      setButtonText("Start");
      setLoading(false);
    }, 3000);
  };

  //  const installCarrierBundle = async () => {
  //    setButtonText("Installing..");
  //
  //    await invoke<boolean>("install_ipcc", {
  //      deviceModel: hardware.model,
  //      iosVer: os.ios_ver,
  //    });
  //
  //    const startListening = async () => {
  //      let isInstallationCheckDone = false; // Track whether the process has completed
  //
  //      // Set up the event listener for 'installation_succeed_status'
  //      const unlisten = await listen<boolean>(
  //        "installation_succeed_status",
  //        (event) => {
  //          const { payload } = event;
  //          if (payload) {
  //            setButtonText("Success!");
  //            setButtonColor("green");
  //          } else {
  //            setButtonText("Failed");
  //            setButtonColor("red");
  //          }
  //          setLoading(false);
  //          isInstallationCheckDone = true; // Mark as done
  //        },
  //      );
  //    };
  //
  //    return result;
  //  };

  const handleButtonClick = async () => {
    setLoading(true);
    setButtonColor("white");

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
