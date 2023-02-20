import { useEffect, useRef, useState } from "react";
import { DebugMessage, debugMessageStore } from "../misc/DebugMessages";
import { WebsocketSession } from "../misc/WebsocketController";
import "./DevConsole.css";

export function DevConsole({ guildId }: { guildId?: string }) {
    const [messages, setMessages] = useState<DebugMessage[]>([])
    const bottom = useRef<HTMLLIElement>(null);


    useEffect(() => {
        if (guildId) {
            setMessages(debugMessageStore.getGuildMessages(guildId));
            WebsocketSession.subscribeGuild(guildId);
        }
    }, [guildId])

    useEffect(() => {
        const key = guildId ?? "global";
        const listenerId = debugMessageStore.addListener(key, onNewMessage);
        return () => {
            debugMessageStore.removeListener(key, listenerId);
        }
    }, [guildId])

    useEffect(() => {
        if (bottom.current) {
            bottom.current.scrollIntoView({ behavior: 'auto' })
        }
    })

    function onNewMessage(message: DebugMessage) {
        setMessages((current) => {
            let newMessages = [
                ...current,
                message
            ]

            return newMessages
        });
    }

    return <ul className="dev-console">
        {messages.map(v => <ConsoleMessage key={v.id} message={v}></ConsoleMessage>)}
        <li ref={bottom}></li>
    </ul>
}

function ConsoleMessage(props: { message: DebugMessage }) {
    return <li className={`dev-console-message dev-console-message-level-${props.message.level.toLowerCase()}`}>
        <pre><span className="dev-console-message-source">[{props.message.level}{props.message.context}]:</span>{props.message.message}</pre>
    </li>
}