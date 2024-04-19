import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { MessageInterface } from '../interfaces/Message';
import { toast } from "sonner";

interface MessagesProviderProps {
  children: ReactNode;
}

interface MessagesContextType {
  messages: MessageInterface[];
  refreshMessages: () => void;
  pushMessage: (message: MessageInterface) => void;
  loadMoreMessages: () => void;
  resetStart : () => void;
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
  const [start, setStart] = useState<number>(0);

  useEffect(() => {
    fetchMessages().then((data) => {
      setMessages(data);
    });
  }, [refreshData]);

  const fetchMessages = async () => {
    try {
      const response = await fetch(`/api/message?limit=50&start=${start}`);
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

  const pushMessage = (message: MessageInterface) => {
    setMessages(prevMessages => [...prevMessages, message]);
  };

  const loadMoreMessages = () => {
    setStart(start + 50);
    refreshMessages();
  };

  const resetStart = () => {
    setStart(0);
  }

  return (
    <MessagesContext.Provider value={{ messages, refreshMessages, pushMessage, loadMoreMessages, resetStart }}>
      {children}
    </MessagesContext.Provider>
  );
};
