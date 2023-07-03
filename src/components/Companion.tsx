import CompanionAvatar from "../assets/companion_avatar.jpg";

const Companion_element = () => {
    return (
        <div className="flex justify-center items-center">
        <div className="avatar card w-52 bg-base-100">
            <div className="w-24 rounded-full self-center">
                <img src={CompanionAvatar} />
            </div>
            <h2 className="text-center">AI companion</h2>
            <button className='btn btn-outline btn-primary'><a>Modify data</a></button>
        </div>
        </div>
    )
}

export default Companion_element
