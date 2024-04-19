import { useState, useEffect, useRef } from 'react';
import { ScrollArea } from "@/components/ui/scroll-area";
import { Message } from "./Message";
import { useMessages } from "../context/messageContext";

export function MessageScroll() {
  const scrollRef = useRef<HTMLDivElement>(null);
  const { messages } = useMessages();
  const [loadMoreVisible] = useState(true);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight - scrollRef.current.clientHeight;
    }
  }, [messages]);

  return (
    <ScrollArea
      ref={scrollRef}
      className="h-[70vh] md:h-[82vh] w-full rounded-md border"
    >
      <div className="p-4 h-full">
        {loadMoreVisible && (
          <h4
            className="mb-4 text-sm font-medium leading-none text-center text-primary cursor-pointer"
           
          >
            Load previous messages
          </h4>
        )}
        <div className="flex flex-col gap-5">
          {messages.map((message, index) => (
            <Message key={index} received={message.ai} id={message.id} regenerate={index === messages.length - 1 && index !== 0} content={message.content} created_at={message.created_at} />
          ))}
        </div>
      </div>
    </ScrollArea>
  );
}
