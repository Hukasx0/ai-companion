import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { Pencil, RotateCw, Star, Trash2 } from "lucide-react";
import { useUserData } from "../context/userContext";
import { useCompanionData } from "../context/companionContext";

import companionAvatar from "../../assets/companion_avatar.jpg";

interface MessageScrollProps {
  received: boolean;
}

interface MessageScrollProps extends MessageProps {
  regenerate: boolean;
  content: string;
  created_at: Date;
}


interface MessageProps {
  regenerate: boolean;
  content: string;
  created_at: Date;
}

const UserMessage = ({ content, created_at }: MessageProps) => {
  const userData = useUserData();

  return (
    <div className='chat chat-end'>
      <div className="chat-header">
        <time className="text-xs mr-3 opacity-50">{new Date(created_at).toLocaleDateString()}</time>
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
            <AvatarImage src={companionData?.avatar_path || companionAvatar} alt="Companion Avatar" />
            <AvatarFallback>H</AvatarFallback>
          </Avatar>
        </div>
      </div>
      <div className="chat-header">
        {companionData?.name || "Assistant"}
        <time className="text-xs ml-3 opacity-50">{new Date(created_at).toLocaleDateString()}</time>
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

export function Message({ received, regenerate, content, created_at }: MessageScrollProps) {
  return (
    <>
      {received ? <AiMessage content={content} created_at={created_at} regenerate={regenerate} />: <UserMessage content={content} created_at={created_at} regenerate={false} /> }
    </>
  );
}
