import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar"
import { Pencil, RotateCw, Star, Trash2 } from "lucide-react";

const UserMessage = () => {
    return (
      <div className='chat chat-end'>
  <div className="chat-header">
    <time className="text-xs mr-3 opacity-50">12:45</time>
    Assistant
  </div>
  <div className="chat-bubble">Hello user!</div> 
  <div className="chat-footer opacity-50 flex flex-row gap-2 mt-1">
    <Pencil />
    <Trash2 />
  </div>
</div>
    )
}

const AiMessage = ({ regenerate }: { regenerate: boolean }) => {
  return (
<div className='chat chat-start'>
  <div className="chat-image avatar">
    <div className="w-10 rounded-full">
      <Avatar>
        <AvatarImage src="https://avatars.githubusercontent.com/u/82332291?v=4" alt="@Hukasx0" />
        <AvatarFallback>H</AvatarFallback>
      </Avatar>
    </div>
  </div>
  <div className="chat-header">
    Assistant
    <time className="text-xs ml-3 opacity-50">12:45</time>
  </div>
  {regenerate ? 
  <div className="flex flex-row gap-2 items-center">
  <div className="chat-bubble">Hello user!</div> 
   <RotateCw />
</div>
:
<div className="chat-bubble">Hello user!</div> 
  }
  <div className="chat-footer opacity-50 flex flex-row gap-2 mt-1">
    <Pencil />
    <Star />
    <Trash2 />
  </div>
</div>
  )
}

  export function Message({ sent, regenerate }: { sent: boolean, regenerate: boolean }) {
    return (
      <>
        {sent ? <UserMessage /> : <AiMessage regenerate={regenerate} />}
      </>
    )
  }


