import React, { useState } from 'react';
import "./Login.css";

function Login() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = (event: any) => {
    event.preventDefault();
    // Handle login logic here (e.g., send data to a backend)
    console.log('Username:', username, 'Password:', password);
    // Clear form fields after submission
    setUsername('');
    setPassword('');
  };

  return (
    <div>
      <h2 className="login-title">Login</h2>
      <form onSubmit={handleSubmit} id='login-form'>
      <div className='group'>
        <label htmlFor="username" className='label'>Username:</label>
        <input
          type="text"
          id="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          required
          className='input'
        />
      </div>
      <div>
        <label htmlFor="password">Password:</label>
        <input
          type="password"
          id="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
        />
      </div>
      <button id="login" type="submit">Login</button>
    </form>
    </div>
    
  );
}

export default Login;