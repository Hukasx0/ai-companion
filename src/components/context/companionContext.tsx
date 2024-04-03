import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { CompanionData } from '../interfaces/CompanionData';

interface CompanionDataProviderProps {
  children: ReactNode;
}

const CompanionDataContext = createContext<CompanionData | null>(null);

export const CompanionDataProvider: React.FC<CompanionDataProviderProps> = ({ children }) => {
  const [companionData, setCompanionData] = useState<CompanionData | null>(null);

  useEffect(() => {
    fetchCompanionData().then((data) => {
      setCompanionData(data);
    });
  }, []);

  const fetchCompanionData = async () => {
    try {
      const response = await fetch('/api/companion');
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
    <CompanionDataContext.Provider value={companionData}>
      {children}
    </CompanionDataContext.Provider>
  );
};

export const useCompanionData = () => {
  return useContext(CompanionDataContext);
};
