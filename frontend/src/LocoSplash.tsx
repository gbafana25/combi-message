import Login from './Login';
import { BrowserRouter, Routes, Route, Link, Form, Router } from 'react-router-dom';
import Signup from './Signup';
import Home from './Home';
import Profile from './Profile';
import { useEffect } from 'react';
import { io } from 'socket.io-client';
import { MessageBrowserClient } from './MessageBrowserClient';



export const LocoSplash = () => {
  

  return (
    <div>
      <header className="navbar fixed-top">
        <div className="container">
          
          <ul className="navbar-nav ">
            <li className="">
              <a
                className=""
                href="https://github.com/loco-rs/loco?ref=starter"
                target="_blank"
                rel="noreferrer"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="20"
                  height="20"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  className="feather feather-github"
                >
                  <title>Loco GitHub repo</title>
                  <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" />
                </svg>
              </a>
            </li>
            <li>
              <a href="/">Home</a>
            </li>
            <li className="">
              <a href="/login">Login</a>
            </li>
            <li>
              <a href="/signup">Signup</a>
            </li>
            <li>
              <a href="/message-browser">Message Browser</a>
            </li>
          </ul>
        </div>
      </header>
      <div>
        
        <BrowserRouter>
          <Routes>
            <Route path="/login" element={<Login/>}/>
            <Route path="/signup" element={<Signup/>}/>
            <Route path="/" element={<Home/>}/>
            <Route path="/profile" element={<Profile/>}/>
            <Route path="/message-browser" element={<MessageBrowserClient/>}/>
          </Routes>
        </BrowserRouter>
          
      </div> 
    </div>
  );
};
