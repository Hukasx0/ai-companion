# AI companion v1

![webui screenshot](https://raw.githubusercontent.com/Hukasx0/ai-companion/main/public/webui_screenshot.png)

![webui screenshot](https://raw.githubusercontent.com/Hukasx0/ai-companion/main/public/webui_screenshot2.png)

![License](https://img.shields.io/github/license/Hukasx0/ai-companion)
![Downloads](https://img.shields.io/github/downloads/Hukasx0/ai-companion/total)

## A single little binary that has all the features you need!

AI Companion is a project that aims to provide a quick, simple, light and convenient way to create AI chatbots on your local computer, it does not require any external API, installation of any libraries, you simply install the binary file corresponding to your operating system and device, download the selected model and you can start using the tool as: WebUI for chatting with LLM, WebUI for roleplaying with an AI character, or use as an API for your other projects that require an AI chatbot.

The project includes many unique features, such as short-term memory, CUDA, OpenCL and Metal support, long-term memory, dialogue tuning, time recognition, learning by chatting, ability to work as a REST API, reading character cards, easy-to-use WebUI allowing you to edit data, edit configuration, send, edit and delete messages.

## Features
- works locally - does not require API keys for other services, which makes it completely free to use (well, apart from electricity costs - your computer must work somehow), also does not require the Internet to work
- privacy - all conversations are kept locally in the SQLite database, which means that your conversations or the characteristics of your AI stay only on your computer
- API - you can use it as a backend for your other projects that requires LLMs, custom ai chatbots or custom ai characters
- speed - wrote in Rust shows good efficiency when it comes to CPU, GPU (nothing slows your generation) and RAM (you don't need to use weaker ai models)
- ease of use - everything can be changed in web user interface, and everything is compiled into a single binary file that can be launched on your machine (no need for playing with hundreds of confusing
files, and no need to fight with wrong library/interpreter/framework versions)
- customization - you can change the AI's name, personality, appearance and the first message sent. Also short term and long term memory of ai can be modified
- short-term memory - artificial intelligence remembers recently received/sent messages
- long-term memory - AI can remember conversations even thousands of prompts later using long-term memory - associating things with different words, phrases, sentences or even dates
- real-time learning - when chatting with the AI, it is able to create "memories" as well as learn about the people it chats with (what their profession is, what they like to eat, drink and so on)
- feeding ai with custom data - using the API, it is possible to save to the AI's long-term memory, e.g. fragments of documents, articles, song lyrics, poems
- roleplay - ai chatbot can (if enabled) perform actions between asterisks (*) e.g. *moves closer*, *waves hello*
- you can load character files in .json or .png (character cards) format. For example, you can create your own using [this tool](https://github.com/Hukasx0/character-factory)
- import and export messages via json
- you can use {{char}} and {{user}} in companion's persona, example dialogue, first message and user persona (if you change username or companion name, you don't need to change these, it will automatically change)
- time - AI Chatbot can obtain information about the current time from the computer, and its long-term memory can remember what days certain conversations took place

## Installation
Download the binary appropriate for your operating system and device from [here](https://github.com/Hukasx0/ai-companion/releases/tag/1.0.0) (for example **ai-companion-windows-cuda.exe**), and then install llm model with **.gguf** extension (for example [this one](https://huggingface.co/TheBloke/zephyr-7B-beta-GGUF/resolve/main/zephyr-7b-beta.Q4_K_M.gguf?download=true)), next launch ai-companion binary file, open your web browser at **http://localhost:3000** and you will see AI Companion WebUI, next click the **gear icon** on the website, go to **config**, and replace **Path to your Large Language Model (LLM)** with path to your **.gguf** model on your drive, after doing it, you can start chatting with your companion!

## Supported AI models
small list of tested and working AI models:
- [Mistral 7B](https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.1-GGUF)
- [Zephyr 7B Beta](https://huggingface.co/TheBloke/zephyr-7B-beta-GGUF)
- [Llama 3 8B Instruct](https://huggingface.co/MaziyarPanahi/Meta-Llama-3-8B-Instruct-GGUF) 

And many many other LLM models in .gguf format

## API documentation
API documentation can be found here

## Projects based on ai-companion Backend/API/Library
- [local assistant](https://github.com/Hukasx0/local-assistant) - llm powered ai virtual assistant
- [matrix companion bot](https://github.com/Hukasx0/matrix-companion-bot) - AI-based chat bot running on the Matrix protocol 

## ~~Use as python library~~ (Deprecated)
~~If you are looking for a Python library that allows you to use the ai-companion backend in your projects, it is available here [ai-companion-py](https://github.com/Hukasx0/ai-companion-py)~~

## Compilation from source code:
To build an executable file you need: [Node.js and npm](https://nodejs.org/), [Rust and cargo](https://www.rust-lang.org/)

To make the software work with CUDA, OpenCL and Metal you must also follow similar steps to those [in this documentation](https://github.com/rustformers/llm/blob/main/doc/acceleration-support.md)

make a git clone of the repository:
```
git clone https://github.com/Hukasx0/ai-companion
```
go to the folder
```
cd ai-companion/
```
install node modules
```
npm i
```
compile everything into one binary
```
npm run build-full
```
or

compile everything into one binary with CUDA support
```
npm run build-full-cuda
```
or

compile everything into one binary with OpenCL support
```
npm run build-full-opencl
```
or

compile everything into one binary with Metal support
```
npm run build-full-metal
```
(after compilation the binary should be in ai-companion/backend/target/release)

and then proceed the same as for [installation](#installation)
