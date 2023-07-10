# AI Companion API documentation
AI companion can be used as a backend or API for projects that require a personalised AI chatbot.

## Endpoint `/api/prompt`

Method: POST

Description: This endpoint sends a request to the AI with the provided prompt. Both the prompt and the prompt reply are stored in the AI's short-term and long-term memory.

Input parameters:
JSON object containing the prompt
- `prompt` (type `String`)
```
{
    "prompt": "<prompt text>"           // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the following JSON body:
```
{
    "id": 0,                            // int
    "ai": true,                         // bool
    "text": "<ai_generated_reply>",     // string
    "date": "now"                       // string
}
```

## Endpoint `/api/messages`

Method: GET

Description: This endpoint retrieves all messages from the database.

Response:
- Returns HTTP status code 200 (OK) with a JSON array of `All messages` stored in the database.
```
[
    {
        "id": <message_id>,             // int
        "ai": <"true" or "false">,      // string
        "text": "<message_text>",       // string
        "date": "<date>"                // string
    },
]
```

## Endpoint `/api/clearMessages`

Method: GET

Description: This endpoint clears the chat log and ai short-term memory by deleting all messages from the database.

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Chat log cleared".

## Endpoint `/api/removeMessage`

Method: POST

Description: This endpoint removes a specific message from the database and ai short-term memory based on the provided message ID.

Input parameters:
JSON object containing the message ID to be removed
- `id` (type: `Int`)
```
{
    "id": <id number>           // int
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Removed message".

## Endpoint `/api/companionData`

Method: GET

Description: This endpoint retrieves all of companion data from the database.

Response:
- Returns HTTP status code 200 (OK) with a JSON representation of the companion data stored in the database.
```
{
    "id": 1,                                                // int
    "name": "<companion name>",                             // string
    "persona": "<companion persona>",                       // string
    "first_message": "<first message that ai sends>"        // string
}
```

## Endpoint `/api/change/firstMessage`

Method: POST

Description: This endpoint updates the first message sent by ai in the database and ai memory.

Input parameters:
JSON object containing the new first message
- `first_message` (type: `String`)
```
{
    "first_message": "<first message the AI ​​will send>"        // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Changed first message".

## Endpoint `/api/change/companionName`

Method: POST

Description: This endpoint updates the name of the companion in the database and ai memory.

Input parameters:
JSON object containing the new companion name
- `companion_name` (type: `String`)
```
{
    "companion_name": "<new companion name>"                    // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Changed companion name".

## Endpoint `/api/change/companionPersona`

Method: POST

Description: This endpoint updates the persona of the companion in the database and ai memory.

Input parameters:
JSON object containing the new companion persona
- `companion_persona` (type: `String`)
```
{
    "companion_persona": "<new companion persona>"            // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Changed companion persona".

## Endpoint `/api/change/companionData`

Method: POST

Description: This endpoint updates the data of the companion in the database and ai memory.

Input parameters:
JSON object containing the new companion data.
- `id` (type: `Int`) - always set to 0
- `name` (type `String`) - new companion name
- `persona` (type `String`) - new companion persona
- `first_message` (type `String`) - new first message sent by ai
```
{
    "id": 0,                                                    // int
    "name": "<new companion name>",                             // string
    "persona": "<new companion persona>",                       // string
    "first_message": "<first message the AI ​​will send>"         // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Data of your AI companion has been changed".

## Endpoint `/api/change/userName`

Method: POST

Description: This endpoint updates the name of user in the database and ai memory.

Input parameters:
JSON object containing new username
- `user_name` (type: `String`)
```
{
    "user_name": "<new username>"        // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Changed user name".

## Endpoint `/api/change/userPersona`

Method: POST

Description: This endpoint updates the persona of user in the database and ai memory.

Input parameters:
JSON object containing the new user persona
- `user_persona` (type: `String`)
```
{
    "user_persona": "<new user persona>"        // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Changed user persona".

## Endpoint `/api/change/userData`

Method: POST

Description: This endpoint updates the data of user in the database.

Input parameters:
- `id` (type: `Int`) - always set to 0
- `name` (type `String`) - new username
- `persona` (type `String`) - new user persona
```
{
    "id": 0,                                // int
    "name": "<new username>",               // string
    "persona": "<new user persona>"         // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the message "Data of user has been changed".

## Endpoint `/api/addData`

Method: POST

Description: This endpoint allows you to save data to the AI's long-term memory.

Input parameters:
JSON object containing text that will be saved in AI's long term memory
- `text` (type: `String`)
```
{
    "text": "<some text>"                   // string
}
```

Response:
- Returns HTTP status code 200 (OK) with the plain text message "Added custom data to AI long term memory".
