import { useState, useEffect, useRef } from 'react';
import { ScrollArea } from "@/components/ui/scroll-area";
import { Message } from "./Message";
import { useMessages } from "../context/messageContext";

export function MessageScroll() {
  const scrollRef = useRef<HTMLDivElement>(null);
  const { messages } = useMessages();
  const [loadMoreVisible, setLoadMoreVisible] = useState(true);
  const [startIndex, setStartIndex] = useState(0);
  const messageLimit = 50;

  useEffect(() => {
    fetchMessages();
  }, []);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight - scrollRef.current.clientHeight;
    }
  }, [messages]);

  const fetchMessages = async () => {
    try {
      const response = await fetch(`http://adres-twojego-api/messages?limit=${messageLimit}&start=${startIndex}`);
      if (!response.ok) {
        throw new Error('Wystąpił błąd podczas pobierania wiadomości');
      }
      const data = await response.json();
      data.forEach((message: any) => {
       // addMessage(message);
      });
      setLoadMoreVisible(data.length === messageLimit);
    } catch (error) {
      console.error(error);
    }
  };

  const handleLoadMore = () => {
    setStartIndex(startIndex + messageLimit);
  };

  return (
    <ScrollArea
      ref={scrollRef}
      className="h-[70vh] md:h-[82vh] w-full rounded-md border"
    >
      <div className="p-4 h-full">
        {loadMoreVisible && (
          <h4
            className="mb-4 text-sm font-medium leading-none text-center text-primary cursor-pointer"
            onClick={handleLoadMore}
          >
            Load previous messages
          </h4>
        )}
        <div className="flex flex-col gap-5">
          {messages.map((message, index) => (
            <Message key={index} sent={index % 2 === 0} regenerate={index === messages.length - 1} content={message.content} created_at={message.created_at} />
          ))}
        </div>
      </div>
    </ScrollArea>
  );
}
