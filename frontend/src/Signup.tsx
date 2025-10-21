import { useState } from "react";
import "./Login.css";
import axios from "axios";
import { useNavigate } from "react-router-dom";

function Signup() {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [email, setEmail] = useState('');
    const navigate = useNavigate();

    const handleSubmit = (event: any) => {
    event.preventDefault();
    // Handle signup logic here (e.g., send data to a backend)
    console.log('Username:', username, 'Password:', password, 'Email:', email);
    axios.post("http://localhost:5150/api/auth/register", {
        name: username,
        password: password,
        email: email
    }).then(resp => {
        navigate("/login");
    })
    // Clear form fields after submission
    setUsername('');
    setPassword('');
    setEmail('');
    };

    return (
    <div>
        <h2 className="login-title">Signup</h2>
        <form onSubmit={handleSubmit} id='login-form'>
        <div className="group">
            <label htmlFor="email" className="label">Email:</label>
            <input
                type="email"
                id="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                className="input"
                />
        </div>
        <div className='group'>
        <label htmlFor="username" className='label'>Username:</label>
        <input
            type="text"
            id="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
        />
        </div>
        <div className="group">
        <label htmlFor="password">Password:</label>
        <input
            type="password"
            id="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
        />
        </div>
        <button id="login" type="submit">Signup</button>
    </form>
    </div>
    
    );
}

export default Signup;