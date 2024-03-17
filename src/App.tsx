import { Button } from './components/ui/button'
import './App.css'

import { ThemeProvider } from "@/components/theme-provider"
import { ModeToggle } from './components/mode-toggle'
import { MessageScroll } from './components/MessageScroll'
import { Input } from './components/ui/input'
import { Avatar, AvatarFallback, AvatarImage } from './components/ui/avatar'

function App() {
  return (
    <>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <div className='max-container'>
          <Avatar>
            <AvatarImage src="https://avatars.githubusercontent.com/u/82332291?v=4" alt="@Hukasx0" />
            <AvatarFallback>H</AvatarFallback>
          </Avatar>
          <Button>Button</Button>
          <ModeToggle />
          <MessageScroll />
          <Input type="text" placeholder="type your message here" className='w-full' />
        </div>
      </ThemeProvider>
    </>
  )
}

export default App
