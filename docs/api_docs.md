# AI Companion v1 API Documentation

## Introduction

The Companion API allows users to send and receive messages, manage companion settings, and retrieve various data related to the companion, user or backend.

## Base URL

The base URL for accessing the Companion API is `http://localhost:3000/api` or `http://<your_ip_address>:3000/api`

## Endpoints

### 1. Messages

#### 1.1 Get Messages

- **URL:** `/message`
- **Method:** `GET`
- **Description:** Retrieve a list of messages exchanged with the companion.
- **Parameters:**
  - `limit` (optional): The maximum number of messages to retrieve. Max is 50.
  - `offset` (optional): The offset for paginating through messages.
- **Response:**
  - Status: 200 OK
  - Body: Array of message objects.
- **Example Request:**
  ```http
  GET /message?limit=50&offset=0
  ```
- **Example Response:**
  ```json
  [
    {
      "id": 1,
      "ai": true,
      "content": "Hello there!",
      "created_at": "Saturday 20.04.2024 17:49"
    },
    {
      "id": 2,
      "ai": false,
      "content": "Hi, can you help me with something?",
      "created_at": "Saturday 20.04.2024 19:02"
    }
  ]
  ```

#### 1.2 Erase messages
- **URL:** `/message`
- **Method:** `DELETE`
- **Description:** Delete every message saved in short-term memory and chat log
- **Response:**
  - Status: 200 OK
  - Body: Chat log cleared!
- **Example Request:**
  ```http
  DELETE /message
  ```


#### 1.3 Add Message

- **URL:** `/message`
- **Method:** `POST`
- **Description:** Add a message to the database (without prompting the AI).
- **Request Body:**
  - `ai` (boolean): Indicates whether the message is from the AI (true) or user (false).
  - `content` (string): The content of the message.
- **Response:**
  - Status: 200 OK
  - Body: Message added!
- **Example Request:**
  ```http
  POST /message
  Content-Type: application/json

  {
    "ai": true,
    "content": "Message sent by AI"
  }
  ```

#### 1.4 Get message by ID

- **URL:** `/message/{id}`
- **Method:** `GET`
- **Description:** Retrieve a message by its ID
- **Path Parameters:**
  - `id` (integer): The ID of the message
- **Response:**
  - Status: 200 OK
  - Body: message object
- **Example Request:**
  ```http
  GET /message/2
  ```
- **Example Response:**
  ```json
    {
      "id": 2,
      "ai": false,
      "content": "Hi, can you help me with something?",
      "created_at": "Saturday 20.04.2024 19:02"
    }
  ```

#### 1.5 Edit Message

- **URL:** `/message/{id}`
- **Method:** `PUT`
- **Description:** Edit a message by its ID
- **Path Parameters:**
  - `id` (integer): The ID of the message to edit
- **Request Body:**
  - `ai` (boolean): Indicates whether the message is from the AI (true) or user (false).
  - `content` (string): The content of the message.
- **Response:**
  - Status: 200 OK
  - Body: Message edited at id {id}
- **Example Request:**
  ```http
  PUT /message/{id}
  Content-Type: application/json

  {
    "ai": true,
    "content": "Message sent by AI"
  }
  ```

#### 1.6 Delete Message

- **URL:** `/message/{id}`
- **Method:** `DELETE`
- **Description:** Delete a message by its ID.
- **Path Parameters:**
  - `id` (integer): The ID of the message to delete.
- **Response:**
  - Status: 200 OK
  - Body: Message deleted at id {id}
- **Example Request:**
  ```http
  DELETE /message/1
  ```

### 2. Companion data

#### 2.1 Get Companion data

- **URL:** `/companion`
- **Method:** `GET`
- **Description:** Retrieve information about the companion.
- **Response:**
  - Status: 200 OK
  - Body: Companion object.
- **Example Request:**
  ```http
  GET /companion
  ```
- **Example Response:**
  ```json
  {
    "name": "Assistant",
    "persona": "Friendly assistant",
    "example_dialogue": "",
    "first_message": "Hello world!",
    "long_term_mem": 2,
    "short_term_mem": 5,
    "roleplay": true,
    "dialogue_tuning": false,
    "avatar_path": "/assets/companion_avatar-4rust.jpg"
  }
  ```

#### 2.2 Update Companion data

- **URL:** `/companion`
- **Method:** `PUT`
- **Description:** Update information about the companion.
- **Request Body:**
  - `name` (string): The name of the companion.
  - `persona` (string): The persona or description of the companion.
  - `example_dialogue` (string): Example dialogue for the companion.
  - `first_message` (string): First message sent by companion
  - `long_term_mem` (number): The number of entries that the AI ​​should recall from long-term memory during a conversation
  - `short_term_mem` (number): The number of entries that the AI ​​should recall from short-term memory during a conversation
  - `roleplay` (boolean): Should the AI ​​perform non-verbal actions between asterisks, e.g. *moves closer*, *waves hello*
  - `dialogue_tuning` (boolean): Should ai use message tuning
  - `avatar_path` (string): Path to the companion's avatar image.
- **Response:**
  - Status: 200 OK
  - Body: Companion data edited!
- **Example Request:**
  ```http
  PUT /companion
  Content-Type: application/json

  {
    "name": "Companion",
    "persona": "New companion",
    "example_dialogue": "",
    "first_message": "Hello friend!",
    "long_term_mem": 0,
    "short_term_mem": 2,
    "roleplay": false,
    "dialogue_tuning": true,
    "avatar_path": "/assets/companion_avatar-4rust.jpg"
  }
  ```

#### 2.3 Update Companion data via character card (.png) file

- **URL:** `/companion/card`
- **Method:** `POST`
- **Description:** Update information about the companion via character card file (you can create character files, e.g. using [this tool](https://github.com/Hukasx0/character-factory)).
- **Response:**
  - Status: 200 OK
  - Body: Updated companion data via character card!
- **Example Request:**
  ```sh
  curl -X POST -H "Content-Type: image/png" -T card.png http://localhost:3000/api/companion/card
  ```

#### 2.4 Update Companion data via character JSON data

- **URL:** `/companion/characterJson`
- **Method:** `POST`
- **Description:** Update information about the companion via character json (you can create character json, e.g. using [this tool](https://github.com/Hukasx0/character-factory)).
- **Request Body:**
  - `name` (string): The name of the companion.
  - `description` (string): The persona or description of the companion.
  - `first_mes` (string): First message sent by companion
  - `mes_example` (string): Example dialogue for the companion.
- **Response:**
  - Status: 200 OK
  - Body: Character json imported successfully!
- **Example Request:**
  ```http
  PUT /companion/characterJson
  Content-Type: application/json

  {
    "name": "Companion",
    "description": "New companion",
    "first_mes": "Hello friend!",
    "mes_example": "",
  }
  ```

#### 2.5 Update Companion avatar

- **URL:** `/companion/avatar`
- **Method:** `POST`
- **Description:** Update companion avatar image
- **Response:**
  - Status: 200 OK
  - Body: Companion avatar changed!
- **Example Request:**
  ```sh
  curl -X POST -H "Content-Type: image/png" -T avatar.png http://localhost:3000/api/companion/avatar
  ```

### 3. User data

#### 3.1 Get User data

- **URL:** `/user`
- **Method:** `GET`
- **Description:** Retrieve information about the user.
- **Response:**
  - Status: 200 OK
  - Body: User object.
- **Example Request:**
  ```http
  GET /user
  ```
- **Example Response:**
  ```json
  {
    "name": "User",
    "persona": "User description"
  }
  ```

#### 3.2 Update User data

- **URL:** `/user`
- **Method:** `PUT`
- **Description:** Update information about the user.
- **Request Body:**
  - `name` (string): The name of the user.
  - `persona` (string): The persona or description of the user.
- **Response:**
  - Status: 200 OK
  - Body: User data edited!
- **Example Request:**
  ```http
  PUT /user
  Content-Type: application/json

  {
    "name": "John Doe",
    "persona": "Programmer that wants to use ai-companion as api for his project"
  }
  ```

### 4. Configuration

#### 4.1 Get Configuration

- **URL:** `/config`
- **Method:** `GET`
- **Description:** Retrieve configuration settings for the companion backend.
- **Response:**
  - Status: 200 OK
  - Body: Configuration settings object.
- **Example Request:**
  ```http
  GET /config
  ```
- **Example Response:**
  ```json
  {
    "device": "CPU",
    "llm_model_path": "/path/to/model.gguf",
    "gpu_layers": 20,
    "prompt_template": "Default"
  }
  ```

#### 4.2 Update Configuration

- **URL:** `/config`
- **Method:** `PUT`
- **Description:** Update configuration settings for the companion backend.
- **Request Body:**
  - `device` (string) ("CPU" || "GPU" || "Metal"): The device used for processing (CPU, GPU, Metal).
  - `llm_model_path` (string): Path to the language model.
  - `gpu_layers` (integer): Number of GPU layers.
  - `prompt_template` (string) ("Default" || "Llama2" || "Mistral"): Prompt template for generating responses (Default, Llama2, Mistral).
- **Response:**
  - Status: 200 OK
  - Body: Config updated!
- **Example Request:**
  ```http
  PUT /config
  Content-Type: application/json

  {
    "device": "GPU",
    "llm_model_path": "/path/to/model.gguf",
    "gpu_layers": 30,
    "prompt_template": "Mistral"
  }
  ```

### 5. Memory

#### 5.1 Add entry to long-term memory

- **URL:** `/memory/longTerm`
- **Method:** `POST`
- **Description:** Add data to ai long-term memory
- **Request Body:**
  - `entry` (string): Information that you want to save in your companion's long-term memory, I recommend breaking large pieces of text into parts
- **Response:**
  - Status: 200 OK
  - Body: Long term memory entry added!
- **Example Request:**
  ```http
  PUT /memory/longTerm
  Content-Type: application/json

  {
    "entry": "AI Companion is a project that aims to provide a quick, simple, light and convenient way to create AI chatbots on your local computer"
  }
  ```

#### 5.2 Erase long-term memory

- **URL:** `/memory/longTerm`
- **Method:** `DELETE`
- **Description:** Clear long term memory.
- **Response:**
  - Status: 200 OK
  - Body: Long term memory cleared!
- **Example Request:**
  ```http
  DELETE /memory/longTerm
  ```

#### 5.3 Add last dialogue to dialogue tuning

- **URL:** `/memory/dialogueTuning`
- **Method:** `POST`
- **Description:** Adds the user's previous message and AI's response as dialogue tuning
- **Response:**
  - Status: 200 OK
  - Body: Saved previous dialogue as template dialogue
- **Example Request:**
  ```http
  POST /memory/dialogueTuing
  ```

#### 5.4 Erase dialogue tuning entries

  - **URL:** `/memory/dialogueTuning`
- **Method:** `DELETE`
- **Description:** Clear all dialogue tuning entries.
- **Response:**
  - Status: 200 OK
  - Body: Dialogue tuning memory cleared!
- **Example Request:**
  ```http
  DELETE /memory/dialogueTuning
  ```

### 6. Prompting

#### 6.1 Update Configuration

- **URL:** `/prompt`
- **Method:** `POST`
- **Description:** Prompt the ai
- **Request Body:**
  - `prompt` (string): Prompt to the AI
- **Response:**
  - Status: 200 OK
  - Body: None
- **Example Request:**
  ```http
  POST /prompt
  Content-Type: application/json

  {
    "prompt": "what time is it currently?"
  }
  ```

#### 6.2 Update Configuration

- **URL:** `/prompt/regenerate`
- **Method:** `GET`
- **Description:** Regenerate answer to your AI prompt
- **Response:**
  - Status: 200 OK
  - Body: None
- **Example Request:**
  ```http
  GET /prompt/regenerate
  ```

---

AI Companion v1

2024 Hubert Kasperek