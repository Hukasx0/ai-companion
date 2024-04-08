import { Avatar, AvatarFallback, AvatarImage } from "./ui/avatar";
import { ModeToggle } from "./mode-toggle";
import { EditDataPopup } from "./editData/EditDataPopup";
import { MessageScroll } from "./message/MessageScroll";
import { Textarea } from "./ui/textarea";
import { Menu, SendHorizontal } from "lucide-react";
import { Button } from "./ui/button";

import companionAvatar from "../assets/companion_avatar.jpg";

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
  } from "@/components/ui/dropdown-menu"
import { DropdownMenuSeparator } from "@/components/ui/dropdown-menu";
import { useCompanionData } from "./context/companionContext";
import { CompanionData } from "./interfaces/CompanionData";

const ChatWindow = () => {
  const companionDataContext = useCompanionData();
  const companionData: CompanionData = companionDataContext?.companionData ?? {} as CompanionData;

    return (
        <>
        <div className='w-full flex justify-end'>
            <ModeToggle />
          </div>
          <div className='flex flex-row items-center gap-5'>
          <Avatar>
            <AvatarImage src={companionData.avatar_path || companionAvatar} alt="Companion Avatar" />
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
            <DropdownMenuContent side="top">
                <DropdownMenuItem>Regenerate</DropdownMenuItem>
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