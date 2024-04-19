import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { Pencil, RotateCw, ThumbsUp, Trash2 } from "lucide-react";
import { useUserData } from "../context/userContext";
import { useCompanionData } from "../context/companionContext";

import companionAvatar from "../../assets/companion_avatar.jpg";
import { CompanionData } from "../interfaces/CompanionData";
import { UserData } from "../interfaces/UserData";
import { useEffect, useState, lazy } from "react";
import { useMessages } from "../context/messageContext";
import { Textarea } from "../ui/textarea";
import { TooltipProvider, Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";
import { toast } from "sonner";

const Markdown = lazy(() => import('react-markdown'));

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
        toast.success('Message updated successfully');
      } else {
        toast.error('Failed to update message');
        console.error('Failed to update message');
      }
    } catch (error) {
      toast.error(`Error updating message: ${error}`);
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
        toast.success('Message deleted successfully');
      } else {
        toast.error('Failed to delete message');
        console.error('Failed to delete message');
      }
    } catch (error) {
      toast.error(`Error deleting message: ${error}`);
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
            <Markdown>{content}</Markdown>
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
            <TooltipProvider delayDuration={250}>
                <Tooltip>
                    <TooltipTrigger asChild>
                    <button onClick={handleEdit}><Pencil /></button>
                    </TooltipTrigger>
                    <TooltipContent side="bottom">
                      <p>Edit message</p>
                    </TooltipContent>
                  </Tooltip>
              </TooltipProvider>
            <TooltipProvider delayDuration={250}>
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

  const [displayedContent, setDisplayedContent] = useState(content);

  const [editing, setEditing] = useState(false);
  const [editedContent, setEditedContent] = useState(content);
  const [originalContent, setOriginalContent] = useState(content);

  useEffect(() => {
    setDisplayedContent(content);
  }, [content]);

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
        toast.success('Message updated successfully');
      } else {
        toast.error('Failed to update message');
        console.error('Failed to update message');
      }
    } catch (error) {
      toast.error(`Error updating message: ${error}`);
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
        toast.success('Message deleted successfully');
      } else {
        toast.error('Failed to delete message');
        console.error('Failed to delete message');
      }
    } catch (error) {
      toast.error(`Error deleting message: ${error}`);
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
        toast.success('Successfully added two previous messages as dialogue tuning');
      } else {
        toast.error('Failed to add tuning message');
        console.error('Failed to add tuning message');
      }
    } catch (error) {
      toast.error(`Error adding tuning message: ${error}`);
      console.error('Error adding tuning message:', error);
    }
  };

  const handleRegenerate = async () => {
    const oc = displayedContent;
    try {
      setDisplayedContent("Regenerating a message...");
      const response = await fetch('/api/prompt/regenerate', {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        refreshMessages();
        setDisplayedContent(await response.text());
      } else {
        toast.error('Failed to regenerate prompt');
        console.error('Failed to regenerate prompt');
        setDisplayedContent(oc);
      }
    } catch (error) {
      toast.error(`Error regenerating prompt: ${error}`);
      console.error('Error regenerating prompt:', error);
      setDisplayedContent(oc);
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
              <Markdown>{displayedContent}</Markdown>
            )}
            </div> 
            {!editing && 
              <TooltipProvider delayDuration={350}>
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
            <Markdown>{displayedContent}</Markdown>
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
            <TooltipProvider delayDuration={250}>
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
              <TooltipProvider delayDuration={250}>
              <Tooltip>
                  <TooltipTrigger asChild>
                  <button onClick={handleTuning}><ThumbsUp /></button>
                  </TooltipTrigger>
                  <TooltipContent side="bottom">
                    <p>Good response</p>
                  </TooltipContent>
                </Tooltip>
            </TooltipProvider>
            }
            <TooltipProvider delayDuration={250}>
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
      {received ? <AiMessage key={id} content={content} id={id} created_at={created_at} regenerate={regenerate} />: <UserMessage key={id} content={content} id={id} created_at={created_at} regenerate={false} /> }
    </>
  );
}
