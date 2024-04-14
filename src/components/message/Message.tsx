import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { Pencil, RotateCw, Star, Trash2 } from "lucide-react";
import { useUserData } from "../context/userContext";
import { useCompanionData } from "../context/companionContext";

import companionAvatar from "../../assets/companion_avatar.jpg";
import { CompanionData } from "../interfaces/CompanionData";
import { UserData } from "../interfaces/UserData";
import { useState } from "react";
import { useMessages } from "../context/messageContext";
import { Textarea } from "../ui/textarea";
import { TooltipProvider, Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";

interface MessageScrollProps {
  received: boolean;
}

interface MessageScrollProps extends MessageProps {
  regenerate: boolean;
  content: string;
  created_at: string;
}


interface MessageProps {
  id: number;
  regenerate: boolean;
  content: string;
  created_at: string;
}

const UserMessage = ({ id, content, created_at }: MessageProps) => {
  const userDataContext = useUserData();
  const userData: UserData = userDataContext?.userData ?? {} as UserData;

  const { refreshMessages } = useMessages();

  const [editing, setEditing] = useState(false);
  const [editedContent, setEditedContent] = useState(content);
  const [originalContent, setOriginalContent] = useState(content);

  const handleEdit = () => {
    setOriginalContent(content);
    setEditing(true);
  };

  const handleSave = async () => {
    try {
      const response = await fetch(`/api/message/${id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ ai: false, content: editedContent }),
      });

      if (response.ok) {
        setEditing(false);
        refreshMessages();
      } else {
        console.error('Failed to update message');
      }
    } catch (error) {
      console.error('Error updating message:', error);
    }
  };

  const handleCancel = () => {
    setEditedContent(originalContent);
    setEditing(false);
  };

  const handleDelete = async () => {
    try {
      const response = await fetch(`/api/message/${id}`, {
        method: 'DELETE',
      });

      if (response.ok) {
        refreshMessages();
      } else {
        console.error('Failed to delete message');
      }
    } catch (error) {
      console.error('Error deleting message:', error);
    }
  };

  return (
    <div className='chat chat-end'>
      <div className="chat-header">
        <time className="text-xs mr-3 opacity-50">{created_at}</time>
        {userData.name || "User"}
      </div>
      <div className="chat-bubble">
          {editing ? (
            <Textarea value={editedContent} onChange={(e) => setEditedContent(e.target.value)} />
          ) : (
            content
          )}
          </div> 
      <div className="chat-footer opacity-50 flex flex-row gap-2 mt-1">
        {editing ? (
          <>
            <button onClick={handleSave}>Save</button>
            <button onClick={handleCancel}>Cancel</button>
          </>
        ) : (
          <>
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                    <button onClick={handleEdit}><Pencil /></button>
                    </TooltipTrigger>
                    <TooltipContent side="bottom">
                      <p>Edit message</p>
                    </TooltipContent>
                  </Tooltip>
              </TooltipProvider>
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                    <button onClick={handleDelete}><Trash2 /></button>
                    </TooltipTrigger>
                    <TooltipContent side="bottom">
                      <p>Delete message</p>
                    </TooltipContent>
                  </Tooltip>
              </TooltipProvider>
          </>
        )}
      </div>
    </div>
  );
};


const AiMessage = ({ id, content, created_at, regenerate }: MessageProps) => {
  const companionDataContext = useCompanionData();
  const companionData: CompanionData = companionDataContext?.companionData ?? {} as CompanionData;

  const { refreshMessages } = useMessages();

  const [editing, setEditing] = useState(false);
  const [editedContent, setEditedContent] = useState(content);
  const [originalContent, setOriginalContent] = useState(content);

  const handleEdit = () => {
    setOriginalContent(content);
    setEditing(true);
  };

  const handleSave = async () => {
    try {
      const response = await fetch(`/api/message/${id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ ai: true, content: editedContent }),
      });

      if (response.ok) {
        setEditing(false);
        refreshMessages();
      } else {
        console.error('Failed to update message');
      }
    } catch (error) {
      console.error('Error updating message:', error);
    }
  };

  const handleCancel = () => {
    setEditedContent(originalContent);
    setEditing(false);
  };

  const handleDelete = async () => {
    try {
      const response = await fetch(`/api/message/${id}`, {
        method: 'DELETE',
      });

      if (response.ok) {
        refreshMessages();
      } else {
        console.error('Failed to delete message');
      }
    } catch (error) {
      console.error('Error deleting message:', error);
    }
  };

  const handleTuning = async () => {
    try {
      const response = await fetch('/api/memory/dialogueTuning', {
        method: 'POST',
      });

      if (response.ok) {
        refreshMessages();
      } else {
        console.error('Failed to add tuning message');
      }
    } catch (error) {
      console.error('Error adding tuning message:', error);
    }
  };

  const handleRegenerate = async () => {
    try {
      const response = await fetch('/api/prompt/regenerate', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        refreshMessages();
      } else {
        console.error('Failed to regenerate prompt');
      }
    } catch (error) {
      console.error('Error regenerating prompt:', error);
    }
  };


  return (
    <div className='chat chat-start'>
      <div className="chat-image avatar">
        <div className="w-10 rounded-full">
          <Avatar>
            <AvatarImage src={companionData.avatar_path || companionAvatar} alt="Companion Avatar" />
            <AvatarFallback>AI</AvatarFallback>
          </Avatar>
        </div>
      </div>
      <div className="chat-header">
        {companionData.name || "Assistant"}
        <time className="text-xs ml-3 opacity-50">{created_at}</time>
      </div>
      {regenerate ? 
        <div className="flex flex-row gap-2 items-center">
          <div className="chat-bubble">
            {editing ? (
              <Textarea value={editedContent} onChange={(e) => setEditedContent(e.target.value)} />
            ) : (
              content
            )}
            </div> 
            {!editing && 
              <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                      <button onClick={handleRegenerate}><RotateCw /></button>
                    </TooltipTrigger>
                    <TooltipContent side="right">
                      <p>Regenerate message</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              }
        </div>
        :
        <div className="chat-bubble">
          {editing ? (
            <Textarea value={editedContent} onChange={(e) => setEditedContent(e.target.value)} />
          ) : (
            content
          )}
        </div> 
      }
      <div className="chat-footer opacity-50 flex flex-row gap-2 mt-1">
        {editing ? (
          <>
            <button onClick={handleSave}>Save</button>
            <button onClick={handleCancel}>Cancel</button>
          </>
        ) : (
          <>
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                    <button onClick={handleEdit}><Pencil /></button>
                    </TooltipTrigger>
                    <TooltipContent side="bottom">
                      <p>Edit message</p>
                    </TooltipContent>
                  </Tooltip>
              </TooltipProvider>
            {regenerate && 
              <TooltipProvider>
              <Tooltip>
                  <TooltipTrigger asChild>
                  <button onClick={handleTuning}><Star /></button>
                  </TooltipTrigger>
                  <TooltipContent side="bottom">
                    <p>Add dialogue to dialogue tuning</p>
                  </TooltipContent>
                </Tooltip>
            </TooltipProvider>
            }
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild>
                    <button onClick={handleDelete}><Trash2 /></button>
                    </TooltipTrigger>
                    <TooltipContent side="bottom">
                      <p>Delete message</p>
                    </TooltipContent>
                  </Tooltip>
              </TooltipProvider>
          </>
        )}
      </div>
    </div>
  );
};

export function Message({ received, regenerate, id, content, created_at }: MessageScrollProps) {
  return (
    <>
      {received ? <AiMessage content={content} id={id} created_at={created_at} regenerate={regenerate} />: <UserMessage content={content} id={id} created_at={created_at} regenerate={false} /> }
    </>
  );
}
