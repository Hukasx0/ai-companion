import React, { createContext, useState, useContext, useEffect } from 'react';
import { CompanionData } from '../interfaces/CompanionData';

const CompanionDataContext = createContext<any>(null);

export const CompanionDataProvider: React.FC = ({ children }) => {
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
