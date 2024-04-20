import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"

import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "../ui/textarea"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { Info } from "lucide-react"

import companionAvatar from "../../assets/companion_avatar.jpg";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { updateCompanionData,  useCompanionData } from "../context/companionContext"

import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { updateUserData, useUserData } from "../context/userContext"
import { updateConfigData, useConfigData } from "../context/configContext"
import { ConfigInterface, Device, PromptTemplate } from "../interfaces/Config"
import { useEffect, useState } from "react"
import { CompanionData } from "../interfaces/CompanionData"
import { UserData } from "../interfaces/UserData"
import { toast } from "sonner"
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "../ui/dialog"
import { useMessages } from "../context/messageContext"

export function EditData() {
  const companionDataContext = useCompanionData();
  const companionData: CompanionData = companionDataContext?.companionData ?? {} as CompanionData;
  const [companionFormData, setCompanionFormData] = useState<CompanionData>(companionData);
  const [avatarFile, setAvatarFile] = useState<File | null>(null);
  const [avatarPreview, setAvatarPreview] = useState(companionData.avatar_path || companionAvatar);

  const userDataContext = useUserData();
  const userData: UserData = userDataContext?.userData ?? {} as UserData;
  const [userFormData, setUserFormData] = useState<UserData>(userData);

  const configContext = useConfigData();
  const configData: ConfigInterface = configContext?.config ?? {} as ConfigInterface;
  const [configFormData, setConfigFormData] = useState<ConfigInterface>(configData);

  const { refreshMessages, resetStart } = useMessages();

  const handleCompanionSave = async () => {
    if (companionFormData) {
      await updateCompanionData(companionFormData);
    }
  };

  const handleUserSave = async () => {
    if (userFormData) {
      await updateUserData(userFormData);
    }
  };

  const handleConfigSave = async () => {
    if (configFormData) {
      await updateConfigData(configFormData);
    }
  };

  const handleAvatarChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files && files.length > 0) {
      const selectedFile = files[0];
      setAvatarFile(selectedFile);
      setAvatarPreview(URL.createObjectURL(selectedFile));
    }
  };

  useEffect(() => {
    if (companionDataContext) {
      setCompanionFormData(companionDataContext.companionData as CompanionData);
    }
  }, [companionDataContext?.companionData]);

  const handleAvatarUpload = async () => {
    if (avatarFile) {
      try {
        const formData = new FormData();
        formData.append("avatar", avatarFile);
        const response = await fetch("/api/companion/avatar", {
          method: "POST",
          headers: {
            'Content-Type': 'image/png',
        },
          body: avatarFile,
        });
        if (response.ok) {
          toast.success("Companion avatar changed successfully!");
          companionDataContext?.refreshCompanionData();
        } else {
          toast.error("Failed to change companion avatar");
          console.error("Failed to change companion avatar");
        }
      } catch (error) {
        console.error("Error uploading avatar:", error);
        toast.error(`Error uploading avatar: ${error}`);
      }
    } else {
      toast.warning("Please select an avatar file to upload");
      console.warn("Please select an avatar file to upload");
    }
  };

  const [characterCardFile, setCharacterCardFile] = useState<File | null>(null);

  const handleCharacterCardChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files && files.length > 0) {
      const selectedFile = files[0];
      setCharacterCardFile(selectedFile);
    }
  };

  const handleCharacterCardUpload = async () => {
    if (characterCardFile) {
      try {
        const formData = new FormData();
        formData.append("character_card", characterCardFile);
        const response = await fetch("/api/companion/card", {
          method: "POST",
          headers: {
            'Content-Type': 'image/png',
        },
          body: characterCardFile,
        });
        if (response.ok) {
          toast.success("Companion card uploaded successfully!");
          await companionDataContext?.refreshCompanionData();
        } else {
          toast.error("Failed to upload character card");
          console.error("Failed to upload character card");
        }
      } catch (error) {
        console.error("Error uploading character card:", error);
        toast.error(`Error uploading character card: ${error}`);
      }
    } else {
      toast.warning("Please select an character card (.png) file to upload");
      console.warn("Please select an character card (.png) file to upload");
    }
  };

  const [characterJsonFile, setCharacterJsonFile] = useState<File | null>(null);

  const handleCharacterJsonChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = event.target.files;
    if (files && files.length > 0) {
      const selectedFile = files[0];
      setCharacterJsonFile(selectedFile);
    }
  };

  const handleCharacterJsonUpload = async () => {
    if (characterJsonFile) {
      try {
        const response = await fetch("/api/companion/characterJson", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: characterJsonFile,
        });
        if (response.ok) {
          toast.success("Character JSON uploaded successfully!");
          await companionDataContext?.refreshCompanionData();
        } else {
          toast.error("Failed to upload character JSON");
        }
      } catch (error) {
        console.error("Error uploading character JSON:", error);
        toast.error(`Error uploading character JSON: ${error}`);
      }
    } else {
      toast.warning("Please select a character JSON file to upload");
    }
  };

  const handleEraseDialogueTuning = async () => {
    try {
      const response = await fetch("/api/memory/dialogueTuning", {
        method: "DELETE",
      });

      if (response.ok) {
        toast.success("Character dialogue tuning cleared successfully!");
      } else {
        toast.error("Failed to erase dialogue tuning");
        console.error("Failed to erase dialogue tuning");
      }
    } catch (error) {
      toast.error(`Error while erasing dialogue tuning: ${error}`);
      console.error("Error while erasing dialogue tuning:", error);
    }
  };

  const handleEraseLongTerm = async () => {
    try {
      const response = await fetch("/api/memory/longTerm", {
        method: "DELETE",
      });

      if (response.ok) {
        toast.success("Long term memory cleared successfully!");
      } else {
        toast.error("Failed to erase long term memory");
        console.error("Failed to erase long term memory");
      }
    } catch (error) {
      toast.error(`Error while erasing long term memory: ${error}`);
      console.error("Error while erasing long term memory:", error);
    }
  };

  const handleClearMessages = async () => {
    try {
      const response = await fetch("/api/message", {
        method: "DELETE",
      });

      if (response.ok) {
        toast.success("Chat log cleared successfully!");
        resetStart();
        refreshMessages();
      } else {
        toast.error("Failed to clear chat log");
        console.error("Failed to clear chat log");
      }
    } catch (error) {
      toast.error(`Error while clearing chat log: ${error}`);
      console.error("Error while clearing chat log:", error);
    }
  };

  const handleExportCharacterJson = async () => {
    try {
      const response = await fetch("/api/companion/characterJson");
      if (response.ok) {
        const json = await response.json();
        const jsonString = JSON.stringify(json);
        const blob = new Blob([jsonString], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "companion.json";
        a.click();
        URL.revokeObjectURL(url);
      } else {
        toast.error("Failed to export companion as JSON");
        console.error("Failed to export companion as JSON");
      }
    } catch (error) {
      toast.error(`Error exporting companion as JSON: ${error}`);
      console.error("Error exporting companion as JSON:", error);
    }
  };

  return (
    <Tabs defaultValue="companion" className="h-[65vh] overflow-y-auto">
      <TabsList className="grid w-full grid-cols-3">
        <TabsTrigger value="companion">Companion</TabsTrigger>
        <TabsTrigger value="user">User</TabsTrigger>
        <TabsTrigger value="config">Config</TabsTrigger>
      </TabsList>
      <TabsContent value="companion">
        <Card className="bg-background border-none">
          <CardHeader>
            <CardTitle>Companion</CardTitle>
            <CardDescription>
                Change your companion data
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
          <div className="flex justify-center">
            <div className="space-y-1 self-center">
              <label htmlFor="avatar" className="cursor-pointer">
                <TooltipProvider delayDuration={350}>
                <Tooltip>
                    <TooltipTrigger asChild>
                    <Avatar className="w-24 h-24">
                      <AvatarImage id="change-avatar" src={avatarPreview} alt="Companion Avatar" />
                      <AvatarFallback>AI</AvatarFallback>
                    </Avatar>
                    </TooltipTrigger>
                    <TooltipContent side="top">
                      <p>Select an image from disk</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
                <input
                  id="avatar"
                  type="file"
                  className="hidden"
                  onChange={handleAvatarChange}
                  accept="image/*"
                />
              </label>
            </div>
          </div>
          <div className="flex justify-center">
            <Button variant={"outline"} onClick={handleAvatarUpload}>Upload avatar</Button>
          </div>
            <div className="space-y-1">
              <Label htmlFor="companionName">Your companion name</Label>
              <Input id="companionName" value={companionFormData.name} onChange={(e) => setCompanionFormData({ ...companionFormData, name: e.target.value })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionPersona" className="flex flex-row gap-2">Your companion's persona
                <TooltipProvider delayDuration={0}>
                  <Tooltip>
                    <TooltipTrigger className="cursor-default"> <Info /></TooltipTrigger>
                    <TooltipContent>
                      <p>personality, look, backstory etc</p>
                    </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </Label>
              <Textarea className="min-h-[100px]" id="companionPersona" value={companionFormData.persona} onChange={(e) => setCompanionFormData({ ...companionFormData, persona: e.target.value })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionDialogue">Example dialogue</Label>
              <Textarea className="min-h-[100px]" id="companionDialogue" value={companionFormData.example_dialogue} onChange={(e) => setCompanionFormData({ ...companionFormData, example_dialogue: e.target.value })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionFirstMessage">First message with which the AI will start a conversation</Label>
              <Textarea className="min-h-[100px]" id="companionFirstMessage" value={companionFormData.first_message} onChange={(e) => setCompanionFormData({ ...companionFormData, first_message: e.target.value })} />
            </div>
            <div className="flex flex-row items-center justify-center">
              <button onClick={handleExportCharacterJson}>Export companion data as JSON</button>
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionLongTermMemory" className="flex flex-row gap-2">long term memory entries
              <TooltipProvider delayDuration={0}>
                  <Tooltip>
                    <TooltipTrigger className="cursor-default"> <Info /></TooltipTrigger>
                    <TooltipContent>
                      <p>how much the ai has to recall things from long-term memory at a time</p>
                    </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              </Label>
              <Input id="companionLongTermMemory" type="number" value={companionFormData.long_term_mem} onChange={(e) => setCompanionFormData({ ...companionFormData, long_term_mem: parseInt(e.target.value) })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionShortTermMemory" className="flex flex-row gap-2">short term memory entries
              <TooltipProvider delayDuration={0}>
                  <Tooltip>
                    <TooltipTrigger className="cursor-default"> <Info /></TooltipTrigger>
                    <TooltipContent>
                      <p>how many recent messages to remind ai at once</p>
                    </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              </Label>
              <Input id="companionShortTermMemory" type="number" value={companionFormData.short_term_mem} onChange={(e) => setCompanionFormData({ ...companionFormData, short_term_mem: parseInt(e.target.value) })} />
            </div>
            <div className="flex items-center space-x-2">
            <input
              type="checkbox"
              className="peer h-4 w-4 shrink-0 rounded-sm border border-primary shadow focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground"
              checked={companionFormData.roleplay}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setCompanionFormData({ ...companionFormData, roleplay: e.target.checked })}
            />
            <label
              htmlFor="roleplay"
              className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 flex flex-row gap-2"
            >
              Roleplay
              <TooltipProvider delayDuration={0}>
                    <Tooltip>
                      <TooltipTrigger className="cursor-default"> <Info /></TooltipTrigger>
                      <TooltipContent>
                        <p>if checked, messages may contain gestures and other non-verbal actions written between asterisks (for example *waves hello* or *moves closer*)</p>
                      </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
            </label>
          </div>
          <div className="flex items-center space-x-2">
            <input
              type="checkbox"
              className="peer h-4 w-4 shrink-0 rounded-sm border border-primary shadow focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground"
              id="dialogueTuning"
              checked={companionFormData.dialogue_tuning}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setCompanionFormData({ ...companionFormData, dialogue_tuning: e.target.checked })}
            />
            <label
              htmlFor="dialogueTuning"
              className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 flex flex-row gap-2"
            >
              Dialogue tuning
              <TooltipProvider delayDuration={0}>
                    <Tooltip>
                      <TooltipTrigger className="cursor-default"> <Info /></TooltipTrigger>
                      <TooltipContent>
                        <p>if checked, the generated messages will resemble those for which you clicked the "good response" button below them</p>
                      </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
            </label>
          </div>
          <div className="flex flex-row gap-2">
            <Input className="text-primary" type="file" accept="image/png" onChange={handleCharacterCardChange} />
            <Button variant={"outline"} onClick={handleCharacterCardUpload}>Upload Character Card</Button>
          </div>
          <div className="flex flex-row gap-2">
            <Input className="text-primary" type="file" accept=".json" onChange={handleCharacterJsonChange} />
            <Button variant={"outline"} onClick={handleCharacterJsonUpload}>Upload Character JSON</Button>
          </div>
          <div className="flex flex-col justify-center items-center gap-4 flex-wrap my-2">
          <Dialog>
              <DialogTrigger><Button variant={"outline"} className="grow">Erase dialogue tuning</Button></DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Are you absolutely sure?</DialogTitle>
                  <DialogDescription>
                  All entries added to dialogue tuning will be erased (this action cannot be undone).
                  </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <DialogClose>
                    <Button onClick={handleEraseDialogueTuning}>Erase dialogue tuning</Button>
                  </DialogClose>
                </DialogFooter>
              </DialogContent>
            </Dialog>
            <Dialog>
              <DialogTrigger><Button variant={"outline"} className="grow">Erase long term memory</Button></DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Are you absolutely sure?</DialogTitle>
                  <DialogDescription>
                  All entries added to long term memory will be erased (this action cannot be undone).
                  </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <DialogClose>
                    <Button onClick={handleEraseLongTerm}>Erase long term memory</Button>
                  </DialogClose>
                </DialogFooter>
              </DialogContent>
            </Dialog>
            <Dialog>
              <DialogTrigger><Button variant={"outline"} className="grow">Clear chat log</Button></DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Are you absolutely sure?</DialogTitle>
                  <DialogDescription>
                  The entire chat log and short-term memory will be erased and the character's first message will be loaded (this action cannot be undone).
                  </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <DialogClose>
                    <Button onClick={handleClearMessages}>Clear chat log</Button>
                  </DialogClose>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </div>
          </CardContent>
          <CardFooter className="flex justify-center mt-3">
            <Button onClick={() => {
                handleCompanionSave();
                companionDataContext?.refreshCompanionData();
              }}>Save changes</Button>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="user">
        <Card className="bg-background border-none">
          <CardHeader>
            <CardTitle>User</CardTitle>
            <CardDescription>
                Change your data
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-1">
              <Label htmlFor="username">Your name</Label>
              <Input id="username" value={userFormData.name} onChange={(e) => setUserFormData({ ...userFormData, name: e.target.value })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="userPersona" className="flex flex-row gap-2">Your persona
                <TooltipProvider delayDuration={0}>
                    <Tooltip>
                      <TooltipTrigger className="cursor-default"> <Info /></TooltipTrigger>
                      <TooltipContent>
                        <p>personality, look, backstory etc</p>
                      </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </Label>
              <Textarea className="min-h-[100px]" id="userPersona" value={userFormData.persona} onChange={(e) => setUserFormData({ ...userFormData, persona: e.target.value })} />
            </div>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Button onClick={() => {
                handleUserSave();
                userDataContext?.refreshUserData();
              }}>Save changes</Button>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="config">
        <Card className="bg-background border-none">
          <CardHeader>
            <CardTitle>Config</CardTitle>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-1">
              <Label htmlFor="username">Device</Label>
              <Select onValueChange={(e) => setConfigFormData({ ...configFormData, device: e as Device })} defaultValue={configFormData?.device}>
              <SelectTrigger className="w-[180px]">
                <SelectValue placeholder="Select a device to run the LLM model" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="CPU">CPU</SelectItem>
                <SelectItem value="GPU">GPU</SelectItem>
                <SelectItem value="Metal">Metal</SelectItem>
              </SelectContent>
            </Select>
            </div>
            <div className="space-y-1">
              <Label htmlFor="gpuLayers" className="flex flex-row gap-2">GPU Layers
              </Label>
              <Input id="gpuLayers" type="number" value={configFormData.gpu_layers} onChange={(e) => setConfigFormData({ ...configFormData, gpu_layers: parseInt(e.target.value) })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="userPersona" className="flex flex-row gap-2">Path to your Large Language Model (LLM)
              <TooltipProvider delayDuration={0}>
                    <Tooltip>
                      <TooltipTrigger className="cursor-default"> <Info /></TooltipTrigger>
                      <TooltipContent>
                        <p>path on the server to the llm model file with the .gguf extension</p>
                      </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
                </Label>
              <Input id="llmModelPath" value={configFormData.llm_model_path} onChange={(e) => setConfigFormData({ ...configData, llm_model_path: e.target.value })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="promptTemplate">Prompt template</Label>
              <Select onValueChange={(e) => setConfigFormData({ ...configFormData, prompt_template: e  as PromptTemplate })} defaultValue={configFormData?.prompt_template}>
                <SelectTrigger className="w-[180px]">
                  <SelectValue placeholder="default" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="Default">Default</SelectItem>
                  <SelectItem value="Llama2">Llama2</SelectItem>
                  <SelectItem value="Mistral">Mistral</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Button onClick={() => {
                handleConfigSave();
                configContext?.refreshConfigData();
              }}>Save changes</Button>
          </CardFooter>
        </Card>
      </TabsContent>
    </Tabs>
  )
}
