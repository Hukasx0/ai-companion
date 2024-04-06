import { Button } from "../../components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogTrigger,
} from "../../components/ui/dialog"

import {
    Drawer,
    DrawerClose,
    DrawerContent,
    DrawerDescription,
    DrawerFooter,
    DrawerHeader,
    DrawerTrigger,
  } from "../../components/ui/drawer"
import { Settings } from "lucide-react"
import { EditData } from "./EditData"

export function EditDataPopup() {
    let isMobile = false;
  //  let isMobileOrTablet = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
    if (window.matchMedia('(max-width: 810px)').matches) {
        isMobile = true;
    }
  return (
    <>
    {isMobile ? 
    <Drawer>
    <DrawerTrigger>
      <Button variant="outline" size={"sm"}><Settings /></Button>
    </DrawerTrigger>
    <DrawerContent>
      <DrawerHeader>
      </DrawerHeader>
      <DrawerDescription>
        <EditData />
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
      <Button variant="outline" size={"sm"}><Settings /></Button>
    </DialogTrigger>
    <DialogContent className="sm:max-w-[425px]">
      <EditData />
    </DialogContent>
  </Dialog>
    }
    </>
  )
}
