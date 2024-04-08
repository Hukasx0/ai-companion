import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { MessageInterface } from '../interfaces/Message';
import { toast } from "sonner";

interface MessagesProviderProps {
  children: ReactNode;
}

interface MessagesContextType {
  messages: MessageInterface[];
  refreshMessages: () => void;
}

const MessagesContext = createContext<MessagesContextType | undefined>(undefined);

export const useMessages = () => {
  const context = useContext(MessagesContext);
  if (!context) {
    throw new Error('useMessages must be used within a MessagesProvider');
  }
  return context;
};

export const MessagesProvider: React.FC<MessagesProviderProps> = ({ children }) => {
  const [messages, setMessages] = useState<MessageInterface[]>([]);
  const [refreshData, setRefreshData] = useState<boolean>(false);

  useEffect(() => {
    fetchMessages().then((data) => {
      setMessages(data);
    });
  }, [refreshData]);

  const fetchMessages = async () => {
    try {
      const response = await fetch('/api/message?limit=50&start=0');
      if (!response.ok) {
        throw new Error('');
      }
      const data: MessageInterface[] = await response.json();
      return data;
    } catch (error) {
      console.error(error);
      toast.error(`Error while fetching messages: ${error}`);
      return [];
    }
  };

  const refreshMessages = () => {
    setRefreshData(!refreshData);
  };


  return (
    <MessagesContext.Provider value={{ messages, refreshMessages }}>
      {children}
    </MessagesContext.Provider>
  );
};
