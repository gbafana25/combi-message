import axios from "axios";
import Cookies from 'js-cookie';
import "./index.css"
import "./Profile.css"
import { useEffect, useState } from "react";

function Profile() {
    const [email, setEmail] = useState('');
    const [name, setName] = useState('')
    var r = {name: "", email: "", pid: "", apikey: ""};
    const [resp, setResp] = useState({name: "", email: "", apikey: ""});

    useEffect(() => {
       curr_user() 
        
    }, []);

    const curr_user = async () => {
            const response = await axios.get("http://localhost:5150/api/auth/current", {
                headers: {
                Authorization: "Bearer "+Cookies.get("token")
                }
            })
            setResp(await response.data)

        }
    
    return (
        
        <div className="profile-all">
            <h1>Account Info</h1>
            <div className="profile-box">
                <h2>Name</h2>
                    <p>{resp.name}</p>
                <h2>Email</h2>
                    <p>{resp.email}</p>
            </div>

            <div className="profile-box">
                <h1>API Key</h1>
                <p>{resp.apikey}</p>    
            </div>
        </div>    
    )
}

export default Profile;