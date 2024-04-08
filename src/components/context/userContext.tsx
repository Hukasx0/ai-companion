import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { UserData } from '../interfaces/UserData';
import { toast } from "sonner";

interface UserDataProviderProps {
  children: ReactNode;
}

interface UserDataContextType {
  userData: UserData | null;
  refreshUserData: () => void;
}

const UserDataContext = createContext<UserDataContextType | null>(null);

export const UserDataProvider: React.FC<UserDataProviderProps> = ({ children }) => {
  const [userData, setUserData] = useState<UserData | null>(null);
  const [refreshData, setRefreshData] = useState<boolean>(false);

  useEffect(() => {
    fetchUserData().then((data) => {
      setUserData(data);
    });
  }, [refreshData]);

  const fetchUserData = async () => {
    try {
      const response = await fetch('/api/user');
      if (!response.ok) {
        throw new Error('');
      }
      const data = await response.json();
      return data;
    } catch (error) {
      console.error(error);
      toast.error(`Error while fetching user data: ${error}`);
      return null;
    }
  };

  const refreshUserData = () => {
    setRefreshData(!refreshData);
  };


  return (
    <UserDataContext.Provider value={{userData, refreshUserData}}>
      {children}
    </UserDataContext.Provider>
  );
};

export const useUserData = () => {
  return useContext(UserDataContext);
};

export const updateUserData = async (userData: UserData) => {
  try {
    const response = await fetch('/api/user', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(userData),
    });
    if (!response.ok) {
      throw new Error('');
    }
    const response_text = await response.text();
    toast.info(response_text);
  } catch (error) {
    console.error(error);
    toast.error(`Error while sending user data to backend: ${error}`);
  }
};
