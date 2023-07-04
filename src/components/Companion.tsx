import CompanionAvatar from "../assets/companion_avatar.jpg";
import { useEffect, useState } from "react";
import "./interfaces/CompanionData";

const Modal = () => {
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

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setCompanionData((prevState) => ({
          ...prevState,
          [name]: value
        }));
      };

      const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        fetch('/api/change/companionData', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(companionData)
        })
          .then((response) => {
            console.log(response);
            window.location.reload();
          })
          .catch((error) => {
            console.log("Error while updating companion data", error);
          });
      };
    
    return (
        <>
        <input type="checkbox" id="modal" className="modal-toggle" />
        <div className="modal">
            <div className="modal-box">
                <h3 className="text-lg font-bold">Change your companion data</h3>
                <p className="py-4">Your AI companion's name</p>
                <input onChange={handleChange} type="text" name="name" id="name" value={companionData && companionData.name} />
                <p className="py-4">Your AI companion's persona (personality, look, backstory etc)</p>
                <input onChange={handleChange} type="text" name="persona" id="persona" value={companionData && companionData.persona} />
                <p className="py-4">First message with which the AI will start conversation</p>
                <input onChange={handleChange} type="text" name="first_message" id="first_message" value={companionData && companionData.first_message} /> <br />
                <button className='btn btn-primary center' onClick={handleSubmit}>Update</button>
            </div>
            <label className="modal-backdrop" htmlFor="modal">Close</label>
            </div>
        </>
    )
}

const Companion_element = () => {
    const clearMessages = async () => {
        try {
            await fetch(`${window.location.href}api/clearMessages`);
        } catch (error) {
            console.log(`Error while clearing chat log: ${error}`);
        }
    }

    const clearButtonPress = async () => {
        await clearMessages();
        window.location.reload();
    }

    return (
        <>
        <Modal />
        <div className="flex justify-center items-center">
        <div className="avatar card w-52 bg-base-100">
            <div className="w-24 rounded-full self-center">
                <img src={CompanionAvatar} />
            </div>
            <h2 className="text-center">AI companion</h2>
            <label htmlFor="modal" className="btn btn-outline btn-primary">Change data</label>
            <button className='btn btn-outline btn-primary' onClick={clearButtonPress}><a>Clear chat</a></button>
        </div>
        </div>
        </>
    )
}

export default Companion_element
