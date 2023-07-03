import { useState } from "react";
import "./interfaces/MessagesInterface";

const MessagesList = ({ messages }: {messages: Messages}) => {
    return (
        <>
            {messages.map((message: Message) => message.ai ? 
                <>
                    <div className="chat chat-start">
                        <div className="chat-image avatar">
                            <div className="w-10 rounded-full">
                                <img src="" />
                            </div>
                        </div>
                        <div className="chat-header">
                            companion name
                            <time className="text-xs opacity-50">{message.date}</time>
                        </div>
                        <div className="chat-bubble">{message.text}</div>
                        </div>
                </> :

                <>
                    <div className="chat chat-end">
                        <div className="chat-image avatar">
                            <div className="w-10 rounded-full">
                                <img src="" />
                            </div>
                        </div>
                        <div className="chat-header">
                            you
                            <time className="text-xs opacity-50">{message.date}</time>
                        </div>
                        <div className="chat-bubble">{message.text}</div>
                        </div>
                </>
                )}
        </>
    )
}

const ChatWindow = () => {

    const [msgs, setMsgs] = useState([
            {
                id: 1,
                ai: true,
                text: "hello user, how can i help you?",
                date: "xyz",
            },
    ]);

    const [inputText, setInputText] = useState('');


    const sendMessage = () => {
        if (inputText !== '') {
            setInputText('');
            const requestOptions =  {
                method: "POST",
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ prompt: inputText })
            }   
            fetch('http://localhost:3000/api/testPrompt', requestOptions)
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
        <div className="mockup-window border bg-base-300 h-3/4">
            <MessagesList messages={msgs}/>
            <div className="flex justify-center items-center">
                <input type="text" placeholder="Send a message" value={inputText} onKeyDown={enterPress} onChange={(v) => handleSentMessage(v.target.value)} className="input input-bordered w-1/3 absolute bottom-0" />
            </div>
        </div>
    );
}

export default ChatWindow;
