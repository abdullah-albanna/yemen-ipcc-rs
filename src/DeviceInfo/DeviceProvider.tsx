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
  available_storage: number;
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
  hardware: Hardware;
  battery: Battery;
  os: OS;
  storage: Storage;
  connected: boolean;
}

// Define default values for each interface
const defaultHardware: Hardware = {
  model: "",
  model_number: "",
  region: "",
};

const defaultBattery: Battery = {
  battery_level: 0,
  battery_health: 0,
  cycle_counts: 0,
};

const defaultOS: OS = {
  ios_ver: "",
  build_num: "",
};

const defaultStorage: Storage = {
  total_storage: 0,
  used_storage: 0,
  available_storage: 0,
};
const DeviceContext = createContext<DeviceContextType>({
  hardware: defaultHardware,
  battery: defaultBattery,
  os: defaultOS,
  storage: defaultStorage,
  connected: false,
});

export const useDeviceContext = () => useContext(DeviceContext);

export const DeviceProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [hardware, setHardware] = useState<Hardware>(defaultHardware);
  const [battery, setBattery] = useState<Battery>(defaultBattery);
  const [os, setOS] = useState<OS>(defaultOS);
  const [storage, setStorage] = useState<Storage>(defaultStorage);
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
    invoke("check_device");

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
