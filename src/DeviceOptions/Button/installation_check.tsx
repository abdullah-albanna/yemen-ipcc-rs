import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const startChecking = async (setButtonColor, setButtonText, setLoading) => {
  try {
    setButtonText("Checking...");

    // Start checking
    await invoke("check_installing_succeed");

    let isInstallationDone = false; // Track whether the process has completed

    // Set up the event listener for 'installation_succeed_status'
    const unlisten = await listen<boolean>(
      "installation_succeed_status",
      (event) => {
        const { payload } = event;
        if (payload) {
          setButtonText("Success!");
          setButtonColor("green");
        } else {
          setButtonText("Failed");
          setButtonColor("red");
        }
        setLoading(false);
        isInstallationDone = true; // Mark as done
      },
    );

    // Poll for updates up to 41 times (with 1 second delay per attempt)
    for (let i = 1; i <= 41; i++) {
      if (isInstallationDone) break; // Stop if the installation is complete

      await new Promise((resolve) => setTimeout(resolve, 1000)); // Wait 1 second
    }

    // Clean up the event listener after finishing
    unlisten();

    if (!isInstallationDone) {
      setButtonColor("red");
      setButtonText("Timeout");
    }
  } catch (e) {
    // Handle errors
    setLoading(false);
    setButtonText("Error");
    setButtonColor("red");
  }
};

export default startChecking;
