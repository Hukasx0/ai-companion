import './App.css'

import { ThemeProvider } from "@/components/theme-provider"
import Footer from './components/Footer'
import ChatWindow from './components/ChatWindow'

function App() {
  return (
    <>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <div className='max-container'>
          <ChatWindow />
        </div>
      </ThemeProvider>
      <Footer />
    </>
  )
}

export default App
