import { useEffect, useState } from 'react';
import "./Login.css";
import axios from 'axios';
import Cookies from 'js-cookie';
import { useNavigate } from 'react-router-dom';

function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  useEffect(() => {
    // validate token
    if(Cookies.get("token") != null) {
      navigate('/profile', {replace: true})
    }
  })

  

  const handleSubmit = (event: any) => {
    event.preventDefault();
    // Handle login logic here (e.g., send data to a backend)
    console.log('Username:', email, 'Password:', password);
    axios.post("http://localhost:5150/api/auth/login", {
      email: email,
      password: password
    }).then(resp => {
      Cookies.set('token', resp.data.token)
      navigate("/profile")
    })
    // Clear form fields after submission
    setEmail('');
    setPassword('');
  };

  return (
    <div>
      <h2 className="login-title">Login</h2>
      <form onSubmit={handleSubmit} id='login-form'>
      <div className='group'>
        <label htmlFor="email-field">Email:</label>
        <input
          type="email"
          id="email-field"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
          className='input'
        />
      </div>
      <div>
        <label htmlFor="password-field">Password:</label>
        <input
          type="password"
          id="password-field"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
          className='input'
        />
      </div>
      <button id="login" type="submit">Login</button>
    </form>
    </div>
    
  );
}

export default Login;