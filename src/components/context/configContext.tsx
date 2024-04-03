import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { ConfigInterface } from '../interfaces/Config';

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
