import { useEffect, useState } from "react";
import "./interfaces/CompanionData";
import "./interfaces/UserData";
import CompanionAvatar from "../assets/companion_avatar.jpg";

const Modal = (companionData: CompanionData | undefined, setCompanionData: React.Dispatch<React.SetStateAction<CompanionData | undefined>> ) => {

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value, type, checked } = e.target;

        const v2 = type === 'checkbox' ? checked : value;

        const v3 = type === 'number' ? parseInt(value, 10) : v2;

        setCompanionData((prevState) => ({
          ...prevState,
          [name]: v3
        }));
      };

      const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (companionData && companionData.roleplay) {
          companionData.roleplay = Boolean(companionData.roleplay)
        }
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
        <input type="checkbox" id="companionModal" className="modal-toggle" />
        <div className="modal">
            <div className="modal-box">
                <h3 className="text-lg font-bold">Change your companion data</h3>
                <p className="py-4">Your AI companion's name</p>
                <input onChange={handleChange} type="text" name="name" id="name" value={companionData && companionData.name} />
                <p className="py-4">Your AI companion's persona (personality, look, backstory etc)</p>
                <input onChange={handleChange} type="text" name="persona" id="persona" value={companionData && companionData.persona} />
                <p className="py-4">First message with which the AI will start conversation</p>
                <input onChange={handleChange} type="text" name="first_message" id="first_message" value={companionData && companionData.first_message} />
                <p className="py-4">long term memory entries (how much the ai has to recall things from long-term memory at a time)</p>
                <input onChange={handleChange} type="number" name="long_term_mem" id="long_term_mem" value={companionData && companionData.long_term_mem} />
                <p className="py-4">short term memory entries (how many recent messages to remind ai at once)</p>
                <input onChange={handleChange} type="number" name="short_term_mem" id="short_term_mem" value={companionData && companionData.short_term_mem} /> <br /> <br />
                <input onChange={handleChange} type="checkbox" name="roleplay" id="roleplay" checked={companionData && companionData.roleplay} />
                <label htmlFor="rp" className="py-4"> roleplay</label>
                 <br /> <br />
                <div className="flex justify-center">
                    <button className='btn btn-primary' onClick={handleSubmit}>Update</button>
                </div>
            </div>
            <label className="modal-backdrop" htmlFor="companionModal">Close</label>
            </div>
        </>
    )
}

const UserModal = () => {
    const [userData, setUserData] = useState<UserData>();

    useEffect(() => {
        const fetchData = async () => {
            try {
                const get_data = await fetch(`${window.location.href}api/userData`);
                const data = await get_data.text();
                const json_data = JSON.parse(data);
                setUserData(json_data);
            } catch (error) {
                console.log('Error while fetching chat messages: ', error);
            }
        };
    
        fetchData();
    }, []);

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setUserData((prevState) => ({
          ...prevState,
          [name]: value
        }));
      };

      const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        fetch('/api/change/userData', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(userData)
        })
          .then((response) => {
            console.log(response);
            window.location.reload();
          })
          .catch((error) => {
            console.log("Error while updating user data", error);
          });
      };
    
    return (
        <>
        <input type="checkbox" id="userModal" className="modal-toggle" />
        <div className="modal">
            <div className="modal-box">
                <h3 className="text-lg font-bold">Change your data</h3>
                <p className="py-4">Your name</p>
                <input onChange={handleChange} type="text" name="name" id="name" value={userData && userData.name} />
                <p className="py-4">Your persona (personality, look, backstory etc)</p>
                <input onChange={handleChange} type="text" name="persona" id="persona" value={userData && userData.persona} /> <br /> <br />
                <div className="flex justify-center">
                    <button className='btn btn-primary' onClick={handleSubmit}>Update</button>
                </div>
            </div>
            <label className="modal-backdrop" htmlFor="userModal">Close</label>
            </div>
        </>
    )
}

const Companion_element = (companionData: CompanionData | undefined, setCompanionData: React.Dispatch<React.SetStateAction<CompanionData | undefined>>) => {
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
        {Modal(companionData, setCompanionData)}
        <UserModal />
        <div className="flex justify-center items-center">
        <div className="avatar card w-52 bg-base-100">
            <div className="w-24 rounded-full self-center">
                <img src={companionData && companionData.avatar_path ? companionData.avatar_path : CompanionAvatar} />
            </div>
            <h2 className="text-center">{companionData && companionData.name ? companionData.name : "AI companion"}</h2>
            <label htmlFor="companionModal" className="btn btn-outline btn-primary">Change data</label>
            <label htmlFor="userModal" className="btn btn-outline btn-primary">Change user data</label>
            <button className='btn btn-outline btn-primary' onClick={clearButtonPress}><a>Clear chat</a></button>
        </div>
        </div>
        </>
    )
}

export default Companion_element
