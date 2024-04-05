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
import { Checkbox } from "@/components/ui/checkbox"
import { Info } from "lucide-react"

import companionAvatar from "../../assets/companion_avatar.jpg";

import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { useCompanionData } from "../context/companionContext"

import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { useUserData } from "../context/userContext"
import { useConfigData } from "../context/configContext"
import { showDevice } from "../interfaces/Config"

export function EditData() {
  const companionData = useCompanionData();
  const userData = useUserData();
  const configData = useConfigData();

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
              <Avatar className="w-24 h-24">
                <AvatarImage src={companionData?.avatar_path || companionAvatar} alt="Companion Avatar" />
                <AvatarFallback>H</AvatarFallback>
              </Avatar>
            </div>
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionName">Your companion name</Label>
              <Input id="companionName" defaultValue={companionData?.name || "Assistant"} />
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
              <Textarea className="min-h-[100px]" id="companionPersona" defaultValue={companionData?.persona || "I am an artificial intelligence chatbot created in ai-companion backend"} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionDialogue">Example dialogue</Label>
              <Textarea className="min-h-[100px]" id="companionDialogue" defaultValue={companionData?.example_dialogue || ""} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionFirstMessage">First message with which the AI will start a conversation</Label>
              <Textarea className="min-h-[100px]" id="companionFirstMessage" defaultValue={companionData?.first_message || "Hello {{user}}, how can i help you?"} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionLongTermMemory" className="flex flex-row gap-2">long term memory entries <Info /></Label>
              <Input id="companionLongTermMemory" type="number" defaultValue={companionData?.long_term_mem || 2} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionShortTermMemory" className="flex flex-row gap-2">short term memory entries <Info /></Label>
              <Input id="companionShortTermMemory" type="number" defaultValue={companionData?.short_term_mem || 5} />
            </div>
            <div className="flex items-center space-x-2">
                <Checkbox id="roleplay" checked={companionData?.roleplay} />
                <label
                    htmlFor="roleplay"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 flex flex-row gap-2"
                >
                    Roleplay <Info/>
                </label>
            </div>
            <div className="flex items-center space-x-2">
                <Checkbox id="dialogueTuning" checked={companionData?.dialogue_tuning} />
                <label
                    htmlFor="dialogueTuning"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 flex flex-row gap-2"
                >
                    Dialogue tuning <Info />
                </label>
            </div>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Button>Save changes</Button>
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
              <Input id="username" defaultValue={userData?.name || "User"} />
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
              <Textarea className="min-h-[100px]" id="userPersona" defaultValue={userData?.persona || "{{user}} is chatting with {{char}} using ai-companion web user interface"} />
            </div>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Button>Save changes</Button>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="config">
        <Card className="bg-background border-none" defaultValue={"cpu"}>
          <CardHeader>
            <CardTitle>Config</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <div className="space-y-1">
              <Label htmlFor="username">Device</Label>
              <Select>
                <SelectTrigger className="w-[180px]">
                  <SelectValue defaultValue={showDevice(configData?.device)} />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="cpu">CPU</SelectItem>
                  <SelectItem value="gpu">GPU</SelectItem>
                  <SelectItem value="metal">Metal</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="space-y-1">
              <Label htmlFor="userPersona" className="flex flex-row gap-2">Path to your Large Language Model (LLM) <Info/></Label>
              <Input id="llmModelPath" defaultValue={configData?.llm_model_path || "models/llama2-7b"} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="promptTemplate">Prompt template</Label>
              <Select>
                <SelectTrigger className="w-[180px]">
                  <SelectValue placeholder="default" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="default">Default</SelectItem>
                  <SelectItem value="llama2">Llama2</SelectItem>
                  <SelectItem value="mistral">Mistral</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </CardContent>
          <CardFooter className="flex justify-center">
            <Button>Save changes</Button>
          </CardFooter>
        </Card>
      </TabsContent>
    </Tabs>
  )
}
