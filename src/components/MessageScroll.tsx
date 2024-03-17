
import { ScrollArea } from "@/components/ui/scroll-area"

const tags = Array.from({ length: 50 }).map(
  (_, i, a) => `${a.length - i}`
)

export function MessageScroll() {
  return (
    <ScrollArea
      className="h-[70vh] md:h-[82vh] w-full rounded-md border"
    >
      <div className="p-4 h-full">
        <h4 className="mb-4 text-sm font-medium leading-none">Messages</h4>
        {tags.map((tag) => (
          <>
            <div key={tag} className="text-sm">
              {tag}
            </div>
          </>
        ))}
      </div>
    </ScrollArea>
  )
}

