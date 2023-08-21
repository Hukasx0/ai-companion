# AI Companion API documentation
AI companion can be used as a backend or API for projects that require a personalised AI chatbot/character.

## Endpoint `/api/prompt`

Method: POST

Description: This endpoint sends a request to the AI with the provided prompt. Both the prompt and the prompt reply are stored in the AI's short-term and long-term memory.

Input parameters:
JSON object containing the prompt
- `prompt` (type `String`)
```
{
    "prompt": <prompt text>           // string
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the following JSON body:
```
{
    "id": 0,                            // int
    "ai": true,                         // bool
    "text": <ai generated reply>,       // string
    "date": "now"                       // string
}
```

## Endpoint `/api/messages`

Method: GET

Description: This endpoint retrieves all messages from the database.

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with a JSON array of `All messages` stored in the database.
```
[
    {
        "id": <message_id>,             // int
        "ai": <"true" or "false">,      // string
        "text": <message_text>,         // string
        "date": <date>                  // string
    },
]
```

## Endpoint `/api/clearMessages`

Method: GET

Description: This endpoint clears the chat log and ai short-term memory by deleting all messages from the database.

Response (if everything was successful):
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

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Removed message".

## Endpoint `/api/companionData`

Method: GET

Description: This endpoint retrieves all of companion data from the database.

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with a JSON representation of the companion data stored in the database.
```
{
    "id": 1,                                                                                        // int
    "name": "<companion name>",                                                                     // string
    "persona": "<companion persona>",                                                               // string
    "example_dialogue": "<example dialogue>",                                                       // string
    "first_message": "<first message that ai sends>"                                                // string
    "long_term_mem": "<number of things to recall from long-term memory>",                          // int
    "short_term_mem": "<number of things to recall from short-term memory>",                        // int
    "roleplay": "<1 or 0>",                                                                         // int (1 means enabled, 0 means disabled)
    "avatar_path": "<path to companion avatar on a server>",                                        // string
}
```

## Endpoint `/api/change/firstMessage`

Method: POST

Description: This endpoint updates the first message sent by ai in the database and ai memory.

You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.

Input parameters:
JSON object containing the new first message
- `first_message` (type: `String`)
```
{
    "first_message": "<first message the AI ​​will send>"        // string
}
```

Response (if everything was successful):
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

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed companion name".

## Endpoint `/api/change/companionPersona`

Method: POST

Description: This endpoint updates the persona of the companion in the database and ai memory.

You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.

Input parameters:
JSON object containing the new companion persona
- `companion_persona` (type: `String`)
```
{
    "companion_persona": "<new companion persona>"            // string
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed companion persona".

## Endpoint `/api/change/companionExampleDialogue`

Method: POST

Description: This endpoint updates the example dialogue of the companion in the database and ai memory.

You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.

Input parameters:
JSON object containing the new example dialogue
- `example_dialogue` (type: `String`)
```
{
    "example_dialogue": "<new example dialogue>"            // string
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed companion example dialogue".

## Endpoint `/api/change/longTermMemory`

Method: POST

Description: This endpoint changes the amount of things to recall from long-term memory.

Input parameters:
JSON object containing the limit of long-term memory
- `limit` (type: `Int`)
```
{
    "limit": "<number of things to recall from long-term memory>"            // int
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed long term memory limit for ai".

## Endpoint `/api/change/shortTermMemory`

Method: POST

Description: This endpoint changes the amount of things to recall from short-term memory - previously sent (generated by ai) or received (sent by user) messages.

Input parameters:
JSON object containing the limit of short-term memory
- `limit` (type: `Int`)
```
{
    "limit": "<number of things to recall from short-term memory>"           // int
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed short term memory limit for ai".

## Endpoint `/api/change/roleplay`

Method: POST

Description: This endpoint changes ai ability to do roleplay actions (e.g. *moves closer*, *walks away*).

Input parameters:
JSON object containing enabling/disabling roleplay
- `enable` (type: `Boolean`)
```
{
    "enable": <true | false>                                              // boolean
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed short term memory limit for ai".

## Endpoint `/api/change/companionData`

Method: POST

Description: This endpoint updates the data of the companion in the database and ai memory.

Input parameters:
JSON object containing the new companion data.
- `id` (type: `Int`) - always set to 0
- `name` (type `String`) - new companion name
- `persona` (type `String`) - new companion persona (You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.)
- `example_dialogue` (type `String`) - new example dialogue (ai will use it to mimic writing style) (You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.)
- `first_message` (type `String`) - new first message sent by ai (You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.)
- `long_term_mem` (type: `Int`) - how many things ai needs to recall from long-term memory
- `short_term_mem` (type: `Int`) - how many things ai needs to recall from short-term memory (how much previous messages sent or received ai will remember)
- `roleplay` (type: `Boolean`) - do you want the ai to do roleplay actions (e.g. *moves closer*, *walks away*)
```
{
    "id": 0,                                                    // int
    "name": "<new companion name>",                             // string
    "persona": "<new companion persona>",                       // string
    "example_dialogue": "<new example dialogue>",               // string
    "first_message": "<first message the AI ​​will send>"         // string
    "long_term_mem": <number of things to recall>,              // int
    "short_term_mem": <number of things to recall>,             // int
    "roleplay": <true | false>                                  // boolean
}
```

Response (if everything was successful):
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

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed user name".

## Endpoint `/api/change/userPersona`

Method: POST

Description: This endpoint updates the persona of user in the database and ai memory.

You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.

Input parameters:
JSON object containing the new user persona
- `user_persona` (type: `String`)
```
{
    "user_persona": "<new user persona>"        // string
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed user persona".

## Endpoint `/api/change/userData`

Method: POST

Description: This endpoint updates the data of user in the database.

Input parameters:
- `id` (type: `Int`) - always set to 0
- `name` (type `String`) - new username
- `persona` (type `String`) - new user persona (You can use {{char}} here - if you are referring to a character, or {{user}} - if you are referring to a user.)
```
{
    "id": 0,                                // int
    "name": "<new username>",               // string
    "persona": "<new user persona>"         // string
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the message "Data of user has been changed".

## Endpoint `/api/addData`

Method: POST

Description: This endpoint allows you to save data to the AI's long-term memory.

Input parameters:
JSON object containing text that will be saved in AI's long term memory
- `text` (type: `String`)
```
{
    "text": <some text>                   // string
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Added custom data to AI long term memory".

## Endpoint `/api/erase/longTermMemory`

Method: GET

Description: This endpoint removes everything from the long-term memory of the AI.

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Erased AI's long term memory".

## Endpoint `/api/change/companionAvatar`

Method: POST

Description: This endpoint allows you to change avatar of your ai companion.

request (in any programming language/tool) must be equivalent to this:
```
curl -X POST -H "Content-Type: image/png" -T avatar.png http://localhost:3000/api/change/companionAvatar
```

where:

`avatar.png` - is the name of your avatar file

`http://localhost:3000/api/change/companionAvatar` - is the server url + port + endpoint

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Changed avatar of your ai companion".

## Endpoint `/api/import/characterJson`

Method: POST
Description: This endpoint allows you to import your companion data via json (you can create character json here https://zoltanai.github.io/character-editor/ or search on some website that contains character json files).

Input parameters:
JSON object containing text that will be saved in AI's long term memory
- `name` (type: `String`)
- `description` (type: `String`)
- `first_mes` (type: `String`)
- `mes_example` (type: `String`)
```
{
    "name": "<character name>",                                                                   // string
    "description": "<character description>",                                                     // string
    "first_mes": "<first message that will be sent by ai>",                                       // string
    "mes_example": "<example dialogue (ai will use it to mimic writing style)>",                  // string
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Data of your ai companion has been changed".

## Endpoint `/api/import/characterCard`

Method: POST

Description: This endpoint allows you to import your companion data via character card (mostly png file) (you can create character json here https://zoltanai.github.io/character-editor/ or search on some website that contains character cards).

request (in any programming language/tool) must be equivalent to this:
```
curl -X POST -H "Content-Type: image/png" -T card.png http://localhost:3000/api/import/characterCard
```

where:

`card.png` - is the name of your character card file

`http://localhost:3000/api/import/characterCard` - is the server url + port + endpoint

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Data of your ai companion has been changed".

## Endpoint `/api/import/messagesJson`

Method: POST
Description: This endpoint allows you to import messages (sent by user and/or ai) to short-term and long-term memory.

Input parameters:
JSON object containing messages, that will be saved in ai's long-term and short-term memory.
- `messages` (type: `[{"ai": boolean, "text": string}]`)
```
{
  "messages": [                     // array
    {
      "ai": <true | false>,         // boolean
      "text": <text of a message>   // string
    },
    // ... can be one or more messages
  ]
}
```

Response (if everything was successful):
- Returns HTTP status code 200 (OK) with the plain text message "Imported messages to your ai companion memory".

## Endpoint `/api/messagesJson`

Method: GET

Description: This endpoint allows you to export messages from ai's short-term memory to json format (you can use this later to import messages in `/api/import/messagesJson`)

Response (if everything was successful):
JSON object containing messages from ai's short-term memory.
```
{
  "messages": [                     // array
    {
      "ai": <true | false>,         // boolean
      "text": <text of a message>   // string
    },
    // ... can be one or more messages
  ]
}
```
