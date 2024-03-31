import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { Pencil, RotateCw, Star, Trash2 } from "lucide-react";
import { useUserData } from "../context/userContext";
import { useCompanionData } from "../context/companionContext";

interface MessageProps {
  sent: boolean;
  regenerate: boolean;
  content: string;
  created_at: Date;
}

const UserMessage = ({ content, created_at }: MessageProps) => {
  const userData = useUserData();

  return (
    <div className='chat chat-end'>
      <div className="chat-header">
        <time className="text-xs mr-3 opacity-50">{created_at.toLocaleTimeString()}</time>
        {userData?.name || "User"}
      </div>
      <div className="chat-bubble">{content}</div> 
      <div className="chat-footer opacity-50 flex flex-row gap-2 mt-1">
        <Pencil />
        <Trash2 />
      </div>
    </div>
  );
};

const AiMessage = ({ content, created_at, regenerate }: MessageProps) => {
  const companionData = useCompanionData();

  return (
    <div className='chat chat-start'>
      <div className="chat-image avatar">
        <div className="w-10 rounded-full">
          <Avatar>
            <AvatarImage src={companionData?.avatar_path || "https://avatars.githubusercontent.com/u/82332291?v=4"} alt="@Hukasx0" />
            <AvatarFallback>H</AvatarFallback>
          </Avatar>
        </div>
      </div>
      <div className="chat-header">
        {companionData?.name || "Assistant"}
        <time className="text-xs ml-3 opacity-50">{created_at.toLocaleTimeString()}</time>
      </div>
      {regenerate ? 
        <div className="flex flex-row gap-2 items-center">
          <div className="chat-bubble">{content}</div> 
          <RotateCw />
        </div>
        :
        <div className="chat-bubble">{content}</div> 
      }
      <div className="chat-footer opacity-50 flex flex-row gap-2 mt-1">
        <Pencil />
        <Star />
        <Trash2 />
      </div>
    </div>
  );
};

export function Message({ sent, regenerate, content, created_at }: MessageProps) {
  return (
    <>
      {sent ? <UserMessage sent={sent} content={content} created_at={created_at} regenerate={false} /> : <AiMessage sent={sent} content={content} created_at={created_at} regenerate={regenerate} />}
    </>
  );
}
