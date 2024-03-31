interface Config {
    id: number;
    device: Device;
    llm_model_path: string;
    prompt_template: PromptTemplate;
}

interface Device {
    CPU: number;
    GPU: number;
    Metal: number;
}

enum PromptTemplate {
    Default = "default",
    Llama2 = "llama2",
    Mistral = "mistral"
}

interface ConfigView {
    device: Device;
    llm_model_path: string;
    prompt_template: PromptTemplate;
}
