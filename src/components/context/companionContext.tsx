import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { CompanionData } from '../interfaces/CompanionData';
import { toast } from "sonner";

interface CompanionDataProviderProps {
  children: ReactNode;
}

interface CompanionDataContextType {
  companionData: CompanionData | null;
  refreshCompanionData: () => void;
}

export const CompanionDataContext = createContext<CompanionDataContextType | null>(null);

export const CompanionDataProvider: React.FC<CompanionDataProviderProps> = ({ children }) => {
  const [companionData, setCompanionData] = useState<CompanionData | null>(null);
  const [refreshData, setRefreshData] = useState<boolean>(false);

  useEffect(() => {
    fetchCompanionData().then((data) => {
      setCompanionData(data);
    });
  }, [refreshData]);

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
      toast.error(`Error while fetching companion data: ${error}`);
      return null;
    }
  };

  const refreshCompanionData = () => {
    setRefreshData(!refreshData);
  };



  return (
    <CompanionDataContext.Provider value={{companionData, refreshCompanionData}}>
      {children}
    </CompanionDataContext.Provider>
  );
};

export const useCompanionData = () => {
  return useContext(CompanionDataContext);
};

export const updateCompanionData = async (companionData: CompanionData) => {
  try {
    const response = await fetch('/api/companion', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(companionData),
    });
    if (!response.ok) {
      throw new Error('');
    }
    const response_text = await response.text();
    toast.info(response_text);
  } catch (error) {
    console.error(error);
    toast.error(`Error while sending companion data to backend: ${error}`);
  }
};

