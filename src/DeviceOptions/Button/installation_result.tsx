import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useDeviceContext } from "./../../DeviceInfo/DeviceProvider";

const startInstalling = async (setButtonColor, setButtonText, setLoading) => {
  let { hardware, os } = useDeviceContext();

  try {
    setButtonText("Installing..");

    await invoke("install_ipcc", {
      deviceModel: hardware.model,
      iosVer: os.ios_ver,
    });

    let isInstallationDone = false; // Track whether the process has completed
    let successInstallation = false;

    const unlisten = await listen<boolean>(
      "carrier_bundle_install_status",
      (event) => {
        const { payload } = event;
        if (payload) {
          setButtonText("Installed successfully!");
          setButtonColor("green");
          successInstallation = true;
        } else {
          setButtonText("Failed to install");
          setButtonColor("red");
          successInstallation = false;
        }
        isInstallationDone = true; // Mark as done
      },
    );

    for (let i = 1; i <= 41; i++) {
      if (isInstallationDone) break;

      await new Promise((resolve) => setTimeout(resolve, 1000)); // Wait 1 second
    }

    // Clean up the event listener after finishing
    unlisten();

    return successInstallation;
  } catch (e) {
    // Handle errors
    setLoading(false);
    setButtonText("Error Installing");
    setButtonColor("red");
    console.log(e);
    return false;
  }
};

export default startInstalling;
