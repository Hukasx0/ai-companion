import { useEffect, useState, useRef } from "react";
import "./interfaces/CompanionData";
import "./interfaces/UserData";
import CompanionAvatar from "../assets/companion_avatar.jpg";

const Modal = (companionData: CompanionData | undefined, setCompanionData: React.Dispatch<React.SetStateAction<CompanionData | undefined>> ) => {

    const handleChange = (e: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => {

        const checked = e.target instanceof HTMLInputElement ? e.target.checked : undefined;
        const { name, value, type } = e.target;

        const v2 = type === 'checkbox' ? checked : value;

        const v3 = type === 'number' ? parseInt(value, 10) : v2;

        setCompanionData((prevState) => ({
          ...prevState,
          [name]: v3
        }));
      };

      const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (companionData) {
          const updatedCompanionData = {
            ...companionData,
            roleplay: Boolean(companionData.roleplay),
            dialogue_tuning: Boolean(companionData.dialogue_tuning)
          };
      
          fetch('/api/change/companionData', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify(updatedCompanionData)
          })
          .then((response) => {
            console.log(response);
            window.location.reload();
          })
          .catch((error) => {
            console.log("Error while updating companion data", error);
          });
        }
      };

      const [chrJson, setChrJson] = useState<File | null>(null);

      const handleJsonChange = (event: React.ChangeEvent<HTMLInputElement>) =>  {
        if (event.target.files && event.target.files.length > 0) {
          setChrJson(event.target.files[0]);
        }
      }

      const [chrCard, setChrCard] = useState<File | null>(null);

      const handleCardChange = (event: React.ChangeEvent<HTMLInputElement>) =>  {
        if (event.target.files && event.target.files.length > 0) {
          setChrCard(event.target.files[0]);
        }
      }

      const [msgsJson, setMsgsJson] = useState<File | null>(null);

      const handleMsgsJsonChange = (event: React.ChangeEvent<HTMLInputElement>) =>  {
        if (event.target.files && event.target.files.length > 0) {
          setMsgsJson(event.target.files[0]);
        }
      }

      const handleJsonSubmit = (type: number) => {
        if (chrJson && type === 0) {
            fetch('/api/import/characterJson', {
                method: 'POST',
                headers: {
                  'Content-Type': 'application/json',
              },
                body: chrJson
            })
            .then((response) => {
              console.log(response);
              window.location.reload();
            })
            .catch((error) => {
              console.log("Error while updating companion data", error);
            });
        }
        else if (msgsJson && type === 1) {
          fetch('/api/import/messagesJson', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
          },
            body: msgsJson
          })
          .then((response) => {
            console.log(response);
            window.location.reload();
          })
          .catch((error) => {
            console.log("Error while importing messages via json file", error);
          });
        }
    };

    const handleCardSubmit = () => {
      if (chrCard) {
        const fileReader = new FileReader();
        fileReader.onload = async () => {
            const fileData = fileReader.result as ArrayBuffer;
            try {
                const response = await fetch('/api/import/characterCard', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'image/png',
                    },
                    body: fileData,
                });

                if (response.ok) {
                  console.log(response);
                  window.location.reload();
                } else {
                  console.log("Error while sending character card");
                }
            } catch (error) {
              console.log("Error while sending character card", error);
            }
        };
        fileReader.readAsArrayBuffer(chrCard);
    }
  };

    const [selectedImage, setSelectedImage] = useState<string>(CompanionAvatar);
    useEffect(() => {
      setSelectedImage(companionData && companionData.avatar_path ? companionData.avatar_path : CompanionAvatar);
    }, [companionData]);
    const [imageBlob, setImageBlob] = useState<Blob| null>(null);
    const inputRef = useRef<HTMLInputElement | null>(null);

    const handleImageClick = () => {
      if (inputRef.current) {
        inputRef.current.click();
      }
    };

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      if (file) {
        const reader = new FileReader();
        setImageBlob(file);
  
        reader.onload = (e) => {
          const imageDataUrl = e.target?.result as string;
          setSelectedImage(imageDataUrl);
        };
  
        reader.readAsDataURL(file);
      }
    };

    const handleAvatarSubmit = () => {
      if (imageBlob) {
        const fileReader = new FileReader();
        fileReader.onload = async () => {
            const fileData = fileReader.result as ArrayBuffer;
            try {
                const response = await fetch('/api/change/companionAvatar', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'image/png',
                    },
                    body: fileData,
                });

                if (response.ok) {
                  console.log(response);
                  window.location.reload();
                } else {
                  console.log("Error while changing companion avatar");
                }
            } catch (error) {
              console.log("Error while changing companion avatar: ", error);
            }
        };
        fileReader.readAsArrayBuffer(imageBlob);
    }
  };

  const eraseLongTermMem = async () => {
    try {
        await fetch('/api/erase/longTermMemory');
    } catch (error) {
        console.log(`Error while erasing long term memory: ${error}`);
    }
}

const eraseButtonPressed = async () => {
    await eraseLongTermMem();
    window.location.reload();
}

const eraseDialogueTuningMsgs = async () => {
  try {
    await fetch('/api/clearTuningMessages');
    window.location.reload();
  } catch (error) {
    console.log(`Error while erasing tuning messages: ${error}`);
  }
}
    
    return (
        <>
        <input type="checkbox" id="companionModal" className="modal-toggle" />
        <div className="modal">
            <div className="modal-box">
                <h3 className="text-lg font-bold">Change your companion data</h3>
                <br />
                <div className="flex justify-center avatar-container">
                  <div className="avatar avatar-change tooltip" data-tip="Change avatar">
                    <div className="w-24 rounded-xl">
                      <img onClick={handleImageClick} src={selectedImage}/>
                      <input
                        ref={inputRef}
                        type="file"
                        accept="image/*"
                        onChange={handleFileChange}
                        style={{ display: 'none' }}
                      />
                  </div>
                </div>
              </div>
              <br />
              <div className="flex justify-center">
                  <button className='btn btn-primary' onClick={handleAvatarSubmit}>Change avatar</button>
                </div>
                <p className="py-4">Your AI companion's name</p>
                <input onChange={handleChange} type="text" name="name" id="name" value={companionData && companionData.name} />
                <p className="py-4">Your AI companion's persona (personality, look, backstory etc)</p>
                <textarea style={{ width: '100%', maxWidth: '500px' }} onChange={handleChange} name="persona" id="persona" value={companionData && companionData.persona} />
                <p className="py-4">Example dialogue</p>
                <textarea style={{ width: '100%', maxWidth: '500px' }} onChange={handleChange} name="example_dialogue" id="example_dialogue" value={companionData && companionData.example_dialogue} />
                <p className="py-4">First message with which the AI will start a conversation</p>
                <textarea style={{ width: '100%', maxWidth: '500px' }} onChange={handleChange} name="first_message" id="first_message" value={companionData && companionData.first_message} />
                <div className="flex justify-center">
                  <a className="text-primary" href="/api/characterJson">Export character as JSON</a>
                </div>
                <p className="py-4">long term memory entries (how much the ai has to recall things from long-term memory at a time)</p>
                <input onChange={handleChange} type="number" name="long_term_mem" id="long_term_mem" value={companionData && companionData.long_term_mem} />
                <p className="py-4">short term memory entries (how many recent messages to remind ai at once)</p>
                <input onChange={handleChange} type="number" name="short_term_mem" id="short_term_mem" value={companionData && companionData.short_term_mem} /> <br /> <br />
                <input onChange={handleChange} type="checkbox" name="roleplay" id="roleplay" checked={companionData && companionData.roleplay} />
                <label htmlFor="rp" className="py-4"> roleplay</label> <br />
                <input onChange={handleChange} type="checkbox" name="dialogue_tuning" id="dialogue_tuning" checked={companionData && companionData.dialogue_tuning} />
                <label htmlFor="rp" className="py-4"> dialogue tuning</label> <br />
                <a className="text-primary" style={{ cursor: "pointer" }} onClick={eraseDialogueTuningMsgs}>Erase liked responses</a>
                 <br /> <br />
                <div className="flex justify-center">
                    <button className='btn btn-primary' onClick={handleSubmit}>Update</button>
                </div>
                <br />
                <input type="file" className="file-input w-full max-w-xs" onChange={handleJsonChange} />
                <div className="flex justify-center">
                    <button className='btn btn-primary' onClick={() => handleJsonSubmit(0)}>Upload character JSON</button>
                </div>
                <br />
                <input type="file" className="file-input w-full max-w-xs" onChange={handleCardChange} />
                <div className="flex justify-center">
                    <button className='btn btn-primary' onClick={handleCardSubmit}>Upload character Card</button>
                </div>
                <br />
                <input type="file" className="file-input w-full max-w-xs" onChange={handleMsgsJsonChange} />
                <div className="flex justify-center">
                    <button className='btn btn-primary' onClick={() => handleJsonSubmit(1)}>Import messages JSON</button>
                </div>
                <br />
                <div className="flex justify-center">
                  <a className="text-primary" href="/api/messagesJson">Export messages as JSON</a>
                </div>
                <br />
                <div className="flex justify-center">
                  <button className='btn btn-outline btn-primary' onClick={eraseButtonPressed}>Erase AI's long term memory</button>
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

    const handleChange = (e: React.ChangeEvent<HTMLInputElement> | React.ChangeEvent<HTMLTextAreaElement>) => {
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
                <textarea style={{ width: '100%', maxWidth: '500px' }} onChange={handleChange} name="persona" id="persona" value={userData && userData.persona} /> <br /> <br />
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
