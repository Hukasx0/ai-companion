import './App.scss'
import Companion_element from './components/Companion';
import Footer from './components/Footer';
import ChatWindow from './components/ChatWindow';

function App() {
  return (
    <>
      <main className='h-screen'>
        <Companion_element />
        <ChatWindow />
      </main>
      <Footer />
    </>
  )
}

export default App
