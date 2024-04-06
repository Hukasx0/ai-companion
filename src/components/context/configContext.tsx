import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { ConfigInterface } from '../interfaces/Config';
import { toast } from "sonner";

interface ConfigProviderProps {
  children: ReactNode;
}

const ConfigContext = createContext<ConfigInterface | null>(null);

export const ConfigProvider: React.FC<ConfigProviderProps> = ({ children }) => {
  const [config, setConfig] = useState<ConfigInterface | null>(null);

  useEffect(() => {
    fetchConfigData().then((data) => {
      setConfig(data);
    });
  }, []);

  const fetchConfigData = async () => {
    try {
      const response = await fetch('/api/config');
      if (!response.ok) {
        throw new Error('');
      }
      const data = await response.json();
      return data;
    } catch (error) {
      console.error(error);
      toast.error(`Error while fetching config data: ${error}`);
      return null;
    }
  };

  return (
    <ConfigContext.Provider value={config}>
      {children}
    </ConfigContext.Provider>
  );
};

export const useConfigData = () => {
  return useContext(ConfigContext);
};

export const updateConfigData = async (configData: ConfigInterface) => {
  try {
    const response = await fetch('/api/config', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(configData),
    });
    if (!response.ok) {
      throw new Error('');
    }
    const response_text = await response.text();
    toast.info(response_text);
  } catch (error) {
    console.error(error);
    toast.error(`Error while sending config data to backend: ${error}`);
  }
};
