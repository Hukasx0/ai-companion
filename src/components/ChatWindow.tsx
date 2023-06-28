import "./interfaces/MessagesInterface";

const msgs: Messages = [
    {
        id: 1,
        sent: false,
        text: "Hello user!",
        date: "16:10",
    },
    {
        id: 2,
        sent: true,
        text: "Hello AI!",
        date: "16:20",
    },
    {
        id: 3,
        sent: false,
        text: "Welcome back!",
        date: "16:22",
    }
];

const MessagesList = ({ messages }: {messages: Messages}) => {
    return (
        <>
            {messages.map((message: Message) => message.sent ? 
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
                </> :

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
                </>
                )}
        </>
    )
}

const ChatWindow = () => {
    return (
        <div className="mockup-window border bg-base-300 h-3/4">
            <MessagesList messages={msgs}/>
            <div className="flex justify-center items-center">
                <input type="text" placeholder="Send a message" className="input input-bordered w-1/3 absolute bottom-0" />
            </div>
        </div>
    );
}

export default ChatWindow;
