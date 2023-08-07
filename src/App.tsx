import './App.scss'
import { useEffect, useState } from "react";
import Companion_element from './components/Companion';
import Footer from './components/Footer';
import ChatWindow from './components/ChatWindow';

function App() {
  const [companionData, setCompanionData] = useState<CompanionData>();

  useEffect(() => {
      const fetchData = async () => {
          try {
              const get_data = await fetch(`${window.location.href}api/companionData`);
              const data = await get_data.text();
              const json_data = JSON.parse(data);
              setCompanionData(json_data);
          } catch (error) {
              console.log('Error while fetching chat messages: ', error);
          }
      };
  
      fetchData();
  }, []);
  return (
    <>
      <main className='h-screen'>
        {Companion_element(companionData, setCompanionData)}
        {ChatWindow(companionData)}
      </main>
      <Footer />
    </>
  )
}

export default App
