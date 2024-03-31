import React, { createContext, useState, useContext, useEffect } from 'react';
import { UserData } from '../interfaces/UserData';

const UserDataContext = createContext<UserData | null>(null);

export const UserDataProvider: React.FC = ({ children }) => {
  const [userData, setUserData] = useState<UserData | null>(null);

  useEffect(() => {
    fetchUserData().then((data) => {
      setUserData(data);
    });
  }, []);

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
      return null;
    }
  };

  return (
    <UserDataContext.Provider value={userData}>
      {children}
    </UserDataContext.Provider>
  );
};

export const useUserData = () => {
  return useContext(UserDataContext);
};
