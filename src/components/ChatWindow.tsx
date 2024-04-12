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
import { useMessages } from "./context/messageContext";
import { useState } from "react";
import { toast } from "sonner";

const ChatWindow = () => {
  const companionDataContext = useCompanionData();
  const companionData: CompanionData = companionDataContext?.companionData ?? {} as CompanionData;

  const { refreshMessages, pushMessage } = useMessages();

  const [message, setMessage] = useState('');

  const handleMessageChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setMessage(event.target.value);
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      promptMessage();
    }
  };

  const promptMessage = async () => {
    try {
      const sendPromise = await fetch('/api/prompt', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ prompt: message }),
      }).then(response => {
        if (response.ok) {
          refreshMessages();
        }
      });
  
      const clearPromise = new Promise<void>(resolve => {
        setMessage('');
        resolve();
      });

      const pushSentMessagePromise = new Promise<void>(resolve => {
        pushMessage({
          id: -1,
          ai: false,
          content: message,
          created_at: new Date()
        });
        resolve();
      });
  
      await Promise.all([sendPromise, clearPromise, pushSentMessagePromise]);

    } catch (error) {
      console.error('Error sending message:', error);
      toast.error(`Error while sending a message: ${error}`);
    }
  };

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
          <Textarea value={message} onChange={handleMessageChange} cols={1} placeholder="Type your message" onKeyDown={handleKeyDown} />
          <Button size={"sm"} onClick={promptMessage}><SendHorizontal /></Button>
          </div>
        </>
    )
}

export default ChatWindow;