interface Message {
    id: number;
    sent: boolean;
    text: string;
    date: string;
}

interface Messages extends Array<Message> {}

