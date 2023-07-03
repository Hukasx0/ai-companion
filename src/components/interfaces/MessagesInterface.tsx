interface Message {
    id: number;
    ai: boolean;
    text: string;
    date: string;
}

interface Messages extends Array<Message> {}

