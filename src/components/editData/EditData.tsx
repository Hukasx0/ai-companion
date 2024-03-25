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

export function EditData() {
  return (
    <Tabs defaultValue="companion">
      <TabsList className="grid w-full grid-cols-2">
        <TabsTrigger value="companion">Companion</TabsTrigger>
        <TabsTrigger value="user">User</TabsTrigger>
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
                <AvatarImage src="https://avatars.githubusercontent.com/u/82332291?v=4" alt="@Hukasx0" />
                <AvatarFallback>H</AvatarFallback>
              </Avatar>
            </div>
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionName">Your companion name</Label>
              <Input id="companionName" defaultValue="Assistant" />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionPersona" className="flex flex-row gap-2">Your companion's persona <Info /> (personality, look, backstory etc)</Label>
              <Textarea id="companionPersona" defaultValue="{{char}} is an artificial intelligence chatbot designed to help {{user}}. {{char}} is an artificial intelligence created in ai-companion backend" />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionDialogue">Example dialogue</Label>
              <Textarea id="companionDialogue" defaultValue="{{char}} is an artificial intelligence chatbot designed to help {{user}}. {{char}} is an artificial intelligence created in ai-companion backend" />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionFirstMessage">First message with which the AI will start a conversation</Label>
              <Textarea id="companionFirstMessage" defaultValue="Hello {{user}}, how can i help you?" />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionLongTermMemory" className="flex flex-row gap-2">long term memory entries <Info /></Label>
              <Input id="companionLongTermMemory" type="number" defaultValue={2} />
            </div>
            <div className="space-y-1">
              <Label htmlFor="companionShortTermMemory" className="flex flex-row gap-2">short term memory entries <Info /></Label>
              <Input id="companionShortTermMemory" type="number" defaultValue={5} />
            </div>
            <div className="flex items-center space-x-2">
                <Checkbox id="roleplay" />
                <label
                    htmlFor="roleplay"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 flex flex-row gap-2"
                >
                    Roleplay <Info/>
                </label>
            </div>
            <div className="flex items-center space-x-2">
                <Checkbox id="dialogueTuning" />
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
              <Input id="username" defaultValue="user" />
            </div>
            <div className="space-y-1">
              <Label htmlFor="userPersona" className="flex flex-row gap-2">Your persona <Info /> (personality, look, backstory etc)</Label>
              <Textarea id="userPersona" defaultValue="{{user}} is chatting with {{char}} using ai-companion web user interface" />
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
