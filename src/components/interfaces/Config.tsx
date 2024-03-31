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

export interface ConfigInterface {
    device: Device;
    llm_model_path: string;
    prompt_template: PromptTemplate;
}
