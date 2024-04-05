interface Device {
    CPU: number;
    GPU: number;
    Metal: number;
}

export function showDevice(d: Device | undefined): string {
    if (d) {
        if (d.CPU) {
            return "cpu";
        }
        if (d.GPU) {
            return "gpu";
        }
        if (d.Metal) {
            return "metal";
        }
    }
    return "cpu";
};


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
