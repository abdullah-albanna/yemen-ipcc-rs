import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
} from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface OS {
  ios_ver: string;
  build_num: string;
}

interface Storage {
  total_storage: number;
  used_storage: number;
  available_storage: number; // Fixed typo: availabel_storage -> available_storage
}

interface Battery {
  battery_level: number;
  battery_health: number;
  cycle_counts: number;
}

interface Hardware {
  model: string;
  model_number: string;
  region: string;
}

interface DeviceContextType {
  hardware: Hardware | null;
  battery: Battery | null;
  os: OS | null;
  storage: Storage | null;
  connected: boolean;
}

const DeviceContext = createContext<DeviceContextType>({
  hardware: null,
  battery: null,
  os: null,
  storage: null,
  connected: false,
});

export const useDeviceContext = () => useContext(DeviceContext);

export const DeviceProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [hardware, setHardware] = useState<Hardware | null>(null);
  const [battery, setBattery] = useState<Battery | null>(null);
  const [os, setOS] = useState<OS | null>(null);
  const [storage, setStorage] = useState<Storage | null>(null);
  const [connected, setConnected] = useState(false);

  // Memoizing the event listeners with useCallback
  const handleHardwareUpdate = useCallback((event: { payload: Hardware }) => {
    setHardware(event.payload);
  }, []);

  const handleBatteryUpdate = useCallback((event: { payload: Battery }) => {
    setBattery(event.payload);
  }, []);

  const handleOSUpdate = useCallback((event: { payload: OS }) => {
    setOS(event.payload);
  }, []);

  const handleStorageUpdate = useCallback((event: { payload: Storage }) => {
    setStorage(event.payload);
  }, []);

  const handleConnectionUpdate = useCallback((event: { payload: boolean }) => {
    setConnected(event.payload);
  }, []);

  useEffect(() => {
    const listeners = [
      listen<Hardware>("device_hardware", handleHardwareUpdate),
      listen<Battery>("device_battery", handleBatteryUpdate),
      listen<OS>("device_os", handleOSUpdate),
      listen<Storage>("device_storage", handleStorageUpdate),
      listen<boolean>("device_status", handleConnectionUpdate),
    ];

    // Start the device monitoring process
    invoke("check_device")
      .then(() => console.log("Started device monitoring"))
      .catch((err) => console.error("Error starting device monitoring:", err));

    // Cleanup listeners on unmount using Promise.all for better readability
    return () => {
      Promise.all(listeners).then((unlisteners) => {
        unlisteners.forEach((unlisten) => unlisten());
      });
    };
  }, [
    handleHardwareUpdate,
    handleBatteryUpdate,
    handleOSUpdate,
    handleStorageUpdate,
    handleConnectionUpdate,
  ]);

  return (
    <DeviceContext.Provider
      value={{ hardware, battery, os, storage, connected }}
    >
      {children}
    </DeviceContext.Provider>
  );
};
