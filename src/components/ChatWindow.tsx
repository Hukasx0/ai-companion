import { Avatar, AvatarFallback, AvatarImage } from "./ui/avatar";
import { ModeToggle } from "./mode-toggle";
import { EditDataPopup } from "./editData/EditDataPopup";
import { MessageScroll } from "./message/MessageScroll";
import { Textarea } from "./ui/textarea";
import { Menu, SendHorizontal } from "lucide-react";
import { Button } from "./ui/button";

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
  } from "@/components/ui/dropdown-menu"
import { DropdownMenuSeparator } from "@/components/ui/dropdown-menu";

const ChatWindow = () => {
    return (
        <>
        <div className='w-full flex justify-end'>
            <ModeToggle />
          </div>
          <div className='flex flex-row items-center gap-5'>
          <Avatar>
            <AvatarImage src="https://avatars.githubusercontent.com/u/82332291?v=4" alt="@Hukasx0" />
            <AvatarFallback>H</AvatarFallback>
          </Avatar>
          <EditDataPopup />
          </div>
          <MessageScroll />
          <div className="flex flex-row w-full items-center gap-2">
          <DropdownMenu>
            <DropdownMenuTrigger>
            <Button variant="outline" size={"sm"}>
                <Menu />
            </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem>Regenerate</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>Continue</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>Impersonate</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
          <Textarea  cols={1} placeholder="Type your message" />
          <Button size={"sm"}><SendHorizontal /></Button>
          </div>
        </>
    )
}

export default ChatWindow;