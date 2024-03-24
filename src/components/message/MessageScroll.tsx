
import { ScrollArea } from "@/components/ui/scroll-area"
import { Message } from "./Message"
import { useLayoutEffect, useRef } from "react";

const tags = Array.from({ length: 50 }).map(
  (_, i, a) => `${a.length - i}`
)

export function MessageScroll() {
  const scrollRef = useRef<HTMLDivElement>(null);

  useLayoutEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight - scrollRef.current.clientHeight;
    }
  }, [tags]);

  return (
    <ScrollArea
    ref={scrollRef}
      className="h-[70vh] md:h-[82vh] w-full rounded-md border"
    >
      <div className="p-4 h-full">
        <h4 className="mb-4 text-sm font-medium leading-none text-center text-primary">Load previous messages</h4>
        <div className="flex flex-col gap-5">
        {tags.map((tag, index) => (
          <>
            <Message key={tag} sent={index % 2 === 0} regenerate={index === tags.length - 1} />
          </>
        ))}
        </div>
      </div>
    </ScrollArea>
  )
}

