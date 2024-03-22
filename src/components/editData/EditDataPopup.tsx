import { Button } from "../../components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../../components/ui/dialog"
import { Input } from "../../components/ui/input"
import { Label } from "../../components/ui/label"

import { Tabs, TabsContent, TabsList, TabsTrigger } from "../../components/ui/tabs"

import {
    Drawer,
    DrawerClose,
    DrawerContent,
    DrawerDescription,
    DrawerFooter,
    DrawerHeader,
    DrawerTitle,
    DrawerTrigger,
  } from "../../components/ui/drawer"

export function EditDataPopup() {
    let isMobile = false;
    if (window.matchMedia('(max-width: 810px)').matches) {
        isMobile = true;
    }
  return (
    <>
    {isMobile ? 
    <Drawer>
    <DrawerTrigger>Open</DrawerTrigger>
    <DrawerContent>
      <DrawerHeader>
        <DrawerTitle>Are you absolutely sure?</DrawerTitle>
        <DrawerDescription>This action cannot be undone.</DrawerDescription>
      </DrawerHeader>
      <DrawerDescription>
      <Tabs defaultValue="companion" className="w-full">
      <DialogHeader>
<TabsList>
  <TabsTrigger value="companion">Companion</TabsTrigger>
  <TabsTrigger value="user">User</TabsTrigger>
</TabsList>
</DialogHeader>
<TabsContent value="account">Make changes to your account here.</TabsContent>
<TabsContent value="password">Change your password here.</TabsContent>
</Tabs>
      </DrawerDescription>
      <DrawerFooter>
        
        <DrawerClose>
          <Button variant="outline">Cancel</Button>
        </DrawerClose>
      </DrawerFooter>
    </DrawerContent>
  </Drawer>
    :
    <Dialog>
    <DialogTrigger asChild>
      <Button variant="outline">Edit Profile</Button>
    </DialogTrigger>
    <DialogContent className="sm:max-w-[425px]">
    <Tabs defaultValue="companion" className="w-full">
      <DialogHeader>
<TabsList>
  <TabsTrigger value="companion">Companion</TabsTrigger>
  <TabsTrigger value="user">User</TabsTrigger>
</TabsList>
</DialogHeader>
<TabsContent value="account">Make changes to your account here.</TabsContent>
<TabsContent value="password">Change your password here.</TabsContent>
</Tabs>
    </DialogContent>
  </Dialog>
    }
    </>
  )
}
