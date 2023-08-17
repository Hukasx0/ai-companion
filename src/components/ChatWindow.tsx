import { useEffect, useState } from "react";
import "./interfaces/MessagesInterface";
import CompanionAvatar from "../assets/companion_avatar.jpg";

const safe_eval = (text: string) => text.replace(/</g, '&lt;').replace(/>/g, "&gt;")
                                    .replace(/\*\*\*([^*]+)\*\*\*/g, "<strong><em>$1</em></strong>")
                                    .replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>")
                                    .replace(/\*([^*]+)\*/g, "<em>$1</em>");


const MessagesList = (companionData: CompanionData | undefined, messages: Messages) => {

    const removeMsg = (id: number) => {
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

    return (
        <>
            {messages.map((message: Message) => message.ai ? 
                <>
                    <div className="chat chat-start">
                        <div className="chat-image avatar">
                            <div className="w-10 rounded-full">
                                <img src={companionData && companionData.avatar_path ? companionData.avatar_path : CompanionAvatar} />
                            </div>
                        </div>
                        <div className="chat-header">
                            {companionData && companionData.name ? companionData.name : "AI companion"}
                            <time className="text-xs opacity-50"> {message.date}</time>
                        </div>
                        <div className="chat-bubble" dangerouslySetInnerHTML={{ __html: safe_eval(message.text) }}></div>
                        <div className="chat-footer opacity-50 cursor-pointer" onClick={() => removeMsg(message.id)}>
                            Remove message
                        </div>
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
                        <div className="chat-bubble" dangerouslySetInnerHTML={{ __html: safe_eval(message.text) }}></div>
                        <div className="chat-footer opacity-50 cursor-pointer" onClick={() => removeMsg(message.id)}>
                            Remove message
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
        <div className="mockup-window border bg-base-300 h-3/5 overflow-y-scroll">
            {MessagesList(companionData, msgs)}
            <div className="flex justify-center items-center">
                <input type="text" placeholder="Send a message" value={inputText} onKeyDown={enterPress} onChange={(v) => handleSentMessage(v.target.value)} className="input input-bordered w-1/3 min-w-max fixed bottom-14" />
            </div>
        </div>
    );
}

export default ChatWindow;
