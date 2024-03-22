import './App.css'

import { ThemeProvider } from "@/components/theme-provider"
import { ModeToggle } from './components/mode-toggle'
import { MessageScroll } from './components/message/MessageScroll'
import { Input } from './components/ui/input'
import { Textarea } from "@/components/ui/textarea"
import { Avatar, AvatarFallback, AvatarImage } from './components/ui/avatar'
import { EditDataPopup } from './components/editData/EditDataPopup'
import Footer from './components/Footer'

function App() {
  return (
    <>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <div className='max-container'>
          <div className='w-full flex justify-end'>
            <ModeToggle />
          </div>
          <div className='flex flex-row'>
          <Avatar>
            <AvatarImage src="https://avatars.githubusercontent.com/u/82332291?v=4" alt="@Hukasx0" />
            <AvatarFallback>H</AvatarFallback>
          </Avatar>
          <EditDataPopup />
          </div>
          <MessageScroll />
        
          <Textarea  cols={1} placeholder="Type your message here." />
        </div>
      </ThemeProvider>
      <Footer />
    </>
  )
}

export default App
