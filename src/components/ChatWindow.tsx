import { useEffect, useState } from "react";
import "./interfaces/MessagesInterface";
import CompanionAvatar from "../assets/companion_avatar.jpg";

const MessagesList = ({ messages }: {messages: Messages}) => {
    return (
        <>
            {messages.map((message: Message) => message.ai ? 
                <>
                    <div className="chat chat-start">
                        <div className="chat-image avatar">
                            <div className="w-10 rounded-full">
                                <img src={CompanionAvatar} />
                            </div>
                        </div>
                        <div className="chat-header">
                            AI companion
                            <time className="text-xs opacity-50"> {message.date}</time>
                        </div>
                        <div className="chat-bubble">{message.text}</div>
                        </div>
                </> :

                <>
                    <div className="chat chat-end">
                        <div className="chat-image avatar">
                            <div className="w-10 rounded-full">
                                
                            </div>
                        </div>
                        <div className="chat-header">
                            you
                            <time className="text-xs opacity-50"> {message.date}</time>
                        </div>
                        <div className="chat-bubble">{message.text}</div>
                        </div>
                </>
                )}
        </>
    )
}

const ChatWindow = () => {

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
        <div className="mockup-window border bg-base-300 h-3/5 overflow-y-scroll">
            <MessagesList messages={msgs}/>
            <div className="flex justify-center items-center">
                <input type="text" placeholder="Send a message" value={inputText} onKeyDown={enterPress} onChange={(v) => handleSentMessage(v.target.value)} className="input input-bordered w-1/3 min-w-max fixed bottom-14" />
            </div>
        </div>
    );
}

export default ChatWindow;
