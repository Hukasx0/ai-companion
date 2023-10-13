import { useEffect, useState } from "react";
import "./interfaces/MessagesInterface";
import CompanionAvatar from "../assets/companion_avatar.jpg";

const safe_eval = (text: string) => text.replace(/</g, '&lt;').replace(/>/g, "&gt;")
                                    .replace(/\*\*\*([^*]+)\*\*\*/g, "<strong><em>$1</em></strong>")
                                    .replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>")
                                    .replace(/\*([^*]+)\*/g, "<em>$1</em>");


const MessagesList = (companionData: CompanionData | undefined, messages: Messages) => {

    const remove_msg = (id: number) => {
        const send_json = {
            id: id
        }

        fetch('/api/removeMessage', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(send_json)
        })
        .then((response) => {
            console.log(response);
            window.location.reload();
        })
        .catch(e => {
            console.error('Cannot remove message because of: ', e);
        });
    }

    const [editMode, setEditMode] = useState<number | null>(null);
    const [editedMessageText, setEditedMessageText] = useState<string>('');

    const start_edit = (messageId: number, initialText: string) => {
        setEditMode(messageId);
        setEditedMessageText(initialText);
    };
  
    const end_edit = () => {
      setEditMode(null);
    };

    const send_updated_message = (messageId: number) => {
        const dataToSend = {
          new_text: editedMessageText,
          id: messageId,
        };
    
        fetch('/api/editMessage', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(dataToSend),
        })
          .then((response) => {
            if (response.ok) {
              console.log('Message updated successfully');
              window.location.reload();
            } else {
              console.error('Error while updating a message');
            }
          })
          .catch((error) => {
            console.error('Błąd: ', error);
          });
      };

    return (
        <>
            {messages.map((message: Message, index: number) => message.ai ? 
                <>
                    <div className="chat chat-start pl-2">
                        <div className="chat-image avatar">
                            <div className="w-10 rounded-full">
                                <img src={companionData && companionData.avatar_path ? companionData.avatar_path : CompanionAvatar} />
                            </div>
                        </div>
                        <div className="chat-header">
                            {companionData && companionData.name ? companionData.name : "AI companion"}
                            <time className="text-xs opacity-50"> {message.date}</time>
                        </div>
                        {editMode === message.id ? (
                            <input type="text" className="chat-bubble" style={{ whiteSpace: "pre-line" }} value={editedMessageText} onChange={(e) => {setEditedMessageText(e.target.value)}} onKeyDown={(e) => {
                                if (e.key === "Escape") {
                                    end_edit();
                                  } else if (e.key === "Enter") {
                                      send_updated_message(message.id);
                                  }
                                }
                            }/>
                        ) : (
                            <div className="chat-bubble" style={{ whiteSpace: "pre-line" }} dangerouslySetInnerHTML={{ __html: safe_eval(message.text) }}></div>
                        )}
                        </div>
                        <div className="flex space-x-2 pl-14">
                            {index === messages.length - 1 && index != 0 && (
                                <div className="chat-footer tiny-text opacity-50 cursor-pointer" onClick={() => {
                                    fetch('/api/regenerate_message', {
                                        method: 'POST',
                                    })
                                .then((response) => {
                                    if (response.ok) {
                                        window.location.reload();
                                      } else {
                                        console.error('Error while regenerating a message');
                                      }
                                })
                                .catch((error) => {
                                    console.error('Error while regenerating a message:', error);
                                });
                                }}>
                                  <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="#555A63"><path d="M16.242 17.242a6.04 6.04 0 0 1-1.37 1.027l.961 1.754a8.068 8.068 0 0 0 2.569-2.225l-1.6-1.201a5.938 5.938 0 0 1-.56.645zm1.743-4.671a5.975 5.975 0 0 1-.362 2.528l1.873.701a7.977 7.977 0 0 0 .483-3.371l-1.994.142zm1.512-2.368a8.048 8.048 0 0 0-1.841-2.859l-1.414 1.414a6.071 6.071 0 0 1 1.382 2.146l1.873-.701zm-8.128 8.763c-.047-.005-.094-.015-.141-.021a6.701 6.701 0 0 1-.468-.075 5.923 5.923 0 0 1-2.421-1.122 5.954 5.954 0 0 1-.583-.506 6.138 6.138 0 0 1-.516-.597 5.91 5.91 0 0 1-.891-1.634 6.086 6.086 0 0 1-.247-.902c-.008-.043-.012-.088-.019-.131A6.332 6.332 0 0 1 6 13.002V13c0-1.603.624-3.109 1.758-4.242A5.944 5.944 0 0 1 11 7.089V10l5-4-5-4v3.069a7.917 7.917 0 0 0-4.656 2.275A7.936 7.936 0 0 0 4 12.999v.009c0 .253.014.504.037.753.007.076.021.15.03.227.021.172.044.345.076.516.019.1.044.196.066.295.032.142.065.283.105.423.032.112.07.223.107.333.026.079.047.159.076.237l.008-.003A7.948 7.948 0 0 0 5.6 17.785l-.007.005c.021.028.049.053.07.081.211.272.433.538.681.785a8.236 8.236 0 0 0 .966.816c.265.192.537.372.821.529l.028.019.001-.001a7.877 7.877 0 0 0 2.136.795l-.001.005.053.009c.201.042.405.071.61.098.069.009.138.023.207.03a8.038 8.038 0 0 0 2.532-.137l-.424-1.955a6.11 6.11 0 0 1-1.904.102z"></path></svg>
                                </div>  
                            )}
                            <div className="chat-footer tiny-text opacity-50 cursor-pointer" onClick={() => start_edit(message.id, message.text)}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="#555A63"><path d="M19.045 7.401c.378-.378.586-.88.586-1.414s-.208-1.036-.586-1.414l-1.586-1.586c-.378-.378-.88-.586-1.414-.586s-1.036.208-1.413.585L4 13.585V18h4.413L19.045 7.401zm-3-3 1.587 1.585-1.59 1.584-1.586-1.585 1.589-1.584zM6 16v-1.585l7.04-7.018 1.586 1.586L7.587 16H6zm-2 4h16v2H4z"></path></svg>
                            </div>
                            <div className="chat-footer tiny-text opacity-50 cursor-pointer" onClick={() => remove_msg(message.id)}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="#555A63"><path d="M9.172 16.242 12 13.414l2.828 2.828 1.414-1.414L13.414 12l2.828-2.828-1.414-1.414L12 10.586 9.172 7.758 7.758 9.172 10.586 12l-2.828 2.828z"></path><path d="M12 22c5.514 0 10-4.486 10-10S17.514 2 12 2 2 6.486 2 12s4.486 10 10 10zm0-18c4.411 0 8 3.589 8 8s-3.589 8-8 8-8-3.589-8-8 3.589-8 8-8z"></path></svg>
                            </div>
                        </div>
                </> :

                <>
                    <div className="chat chat-end pr-2">
                        <div className="chat-image avatar">
                            <div className="w-10 rounded-full">
                                
                            </div>
                        </div>
                        <div className="chat-header">
                            you
                            <time className="text-xs opacity-50"> {message.date}</time>
                        </div>
                        {editMode === message.id ? (
                            <input type="text" className="chat-bubble" style={{ whiteSpace: "pre-line" }} value={editedMessageText} onChange={(e) => {setEditedMessageText(e.target.value)}} onKeyDown={(e) => {
                                if (e.key === "Escape") {
                                    end_edit();
                                  } else if (e.key === "Enter") {
                                      send_updated_message(message.id);
                                  }
                                }
                            }/>
                        ) : (
                            <div className="chat-bubble" style={{ whiteSpace: "pre-line" }} dangerouslySetInnerHTML={{ __html: safe_eval(message.text) }}></div>
                        )}
                        </div>
                        <div className="flex justify-end space-x-2 pr-14">
                            <div className="chat-footer tiny-text opacity-50 cursor-pointer" onClick={() => start_edit(message.id, message.text)}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="#555A63"><path d="M19.045 7.401c.378-.378.586-.88.586-1.414s-.208-1.036-.586-1.414l-1.586-1.586c-.378-.378-.88-.586-1.414-.586s-1.036.208-1.413.585L4 13.585V18h4.413L19.045 7.401zm-3-3 1.587 1.585-1.59 1.584-1.586-1.585 1.589-1.584zM6 16v-1.585l7.04-7.018 1.586 1.586L7.587 16H6zm-2 4h16v2H4z"></path></svg>
                            </div>
                            <div className="chat-footer tiny-text opacity-50 cursor-pointer" onClick={() => remove_msg(message.id)}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="#555A63"><path d="M9.172 16.242 12 13.414l2.828 2.828 1.414-1.414L13.414 12l2.828-2.828-1.414-1.414L12 10.586 9.172 7.758 7.758 9.172 10.586 12l-2.828 2.828z"></path><path d="M12 22c5.514 0 10-4.486 10-10S17.514 2 12 2 2 6.486 2 12s4.486 10 10 10zm0-18c4.411 0 8 3.589 8 8s-3.589 8-8 8-8-3.589-8-8 3.589-8 8-8z"></path></svg>
                            </div>
                        </div>
                </>
                )}
        </>
    )
}

const ChatWindow = (companionData: CompanionData | undefined) => {

    const [msgs, setMsgs] = useState<Messages>([]);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const get_data = await fetch(`${window.location.href}api/messages`);
                const data = await get_data.text();
                const parsedData = JSON.parse(data, (key, value) => {
                    if (key === 'ai') {
                        if (value === 'true') {
                            return true;
                        } else if (value === 'false') {
                            return false;
                        }
                    }
                    return value;
                });
                setMsgs(parsedData);
            } catch (error) {
                console.log('Error while fetching chat messages: ', error);
            }
        };
    
        fetchData();
    }, []);

    const [inputText, setInputText] = useState('');


    const sendMessage = () => {
        if (inputText !== '') {
            setInputText('');
            const requestOptions =  {
                method: "POST",
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ prompt: inputText })
            }   
            fetch(`${window.location.href}api/prompt`, requestOptions)
            .then(response => response.json())
            .then(jdata => setMsgs((prevMsgs) => [
                ...prevMsgs,
                jdata
            ]));
        }
    }

    const enterPress = (event: any) => {
        if (event.key === 'Enter') {
            setInputText('');
            setMsgs((prevMsgs) => [
                ...prevMsgs,
                {
                id: 0,
                ai: false,
                text: event.target.value,
                date: "now",
                },
            ]);
            sendMessage();
          }
    }

    function handleSentMessage(value: string) {
        setInputText(value);
    }

    return (
        <div className="chat-container mockup-window border bg-base-300 h-3/5 overflow-y-scroll flex flex-col">
        <div className="flex-grow">
          {MessagesList(companionData, msgs)}
        </div>
        <div className="flex-shrink-0 p-4">
          <input
            type="text"
            placeholder="Send a message"
            value={inputText}
            onKeyDown={enterPress}
            onChange={(v) => handleSentMessage(v.target.value)}
            className="input input-bordered w-full"
          />
        </div>
      </div>
    );
}

export default ChatWindow;
