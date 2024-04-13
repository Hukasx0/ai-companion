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
import { useState } from "react"
import { CompanionData } from "../interfaces/CompanionData"
import { UserData } from "../interfaces/UserData"
import { toast } from "sonner"

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
        }
      } catch (error) {
        console.error("Error uploading avatar:", error);
        toast.error(`Error uploading avatar: ${error}`);
      }
    } else {
      toast.warning("Please select an avatar file to upload");
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
          <CardContent className="space-y-2 ">
          <div className="flex justify-center">
            <div className="space-y-1 self-center">
              <label htmlFor="avatar" className="cursor-pointer">
                <Avatar className="w-24 h-24">
                  <AvatarImage src={avatarPreview} alt="Companion Avatar" />
                  <AvatarFallback>AI</AvatarFallback>
                </Avatar>
                <input
                  id="avatar"
                  type="file"
                  className="hidden"
                  onChange={handleAvatarChange}
                />
              </label>
            </div>
          </div>
          <div className="flex justify-center">
            <Button onClick={handleAvatarUpload}>Upload</Button>
          </div>
            <div className="space-y-1">
              <Label htmlFor="companionName">Your companion name</Label>
              <Input id="companionName" value={companionFormData.name} onChange={(e) => setCompanionFormData({ ...companionFormData, name: e.target.value })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionPersona" className="flex flex-row gap-2">Your companion's persona
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger> <Info /></TooltipTrigger>
                    <TooltipContent>
                      <p>(personality, look, backstory etc)</p>
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
            <div className="space-y-1">
              <Label htmlFor="companionLongTermMemory" className="flex flex-row gap-2">long term memory entries <Info /></Label>
              <Input id="companionLongTermMemory" type="number" value={companionFormData.long_term_mem} onChange={(e) => setCompanionFormData({ ...companionFormData, long_term_mem: parseInt(e.target.value) })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionShortTermMemory" className="flex flex-row gap-2">short term memory entries <Info /></Label>
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
              Roleplay <Info/>
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
              Dialogue tuning <Info />
            </label>
          </div>
          </CardContent>
          <CardFooter className="flex justify-center">
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
          <CardContent className="space-y-2">
            <div className="space-y-1">
              <Label htmlFor="username">Your name</Label>
              <Input id="username" value={userFormData.name} onChange={(e) => setUserFormData({ ...userFormData, name: e.target.value })} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="userPersona" className="flex flex-row gap-2">Your persona
                <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger> <Info /></TooltipTrigger>
                      <TooltipContent>
                        <p>(personality, look, backstory etc)</p>
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
          <CardContent className="space-y-2">
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
              <Label htmlFor="userPersona" className="flex flex-row gap-2">Path to your Large Language Model (LLM) <Info/></Label>
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
