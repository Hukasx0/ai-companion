import React, { createContext, useState, useContext, useEffect } from 'react';
import { MessageInterface } from '../interfaces/Message';


interface MessagesContextType {
  messages: MessageInterface[];
  addMessage: (message: MessageInterface) => void;
}

const MessagesContext = createContext<MessagesContextType | undefined>(undefined);

export const useMessages = () => {
  const context = useContext(MessagesContext);
  if (!context) {
    throw new Error('useMessages must be used within a MessagesProvider');
  }
  return context;
};

export const MessagesProvider: React.FC = ({ children }) => {
  const [messages, setMessages] = useState<MessageInterface[]>([]);

  const addMessage = (message: MessageInterface) => {
    setMessages(prevMessages => [...prevMessages, message]);
  };

  useEffect(() => {
    fetchMessages().then((data) => {
      setMessages(data);
    });
  }, []);

  const fetchMessages = async () => {
    try {
      const response = await fetch('/api/messages');
      if (!response.ok) {
        throw new Error('');
      }
      const data: MessageInterface[] = await response.json();
      return data;
    } catch (error) {
      console.error(error);
      return [];
    }
  };

  return (
    <MessagesContext.Provider value={{ messages, addMessage }}>
      {children}
    </MessagesContext.Provider>
  );
};
