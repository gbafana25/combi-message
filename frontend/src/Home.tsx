import Cookies from 'js-cookie';
import { BrowserRouter, Route, Routes, useNavigate } from 'react-router-dom';
import Login from './Login';
import "./Home.css";

const LoginButton = () => {
    return (
        <a className='button-3' role='button-3' href='/login'>Login</a>
    )
}

const SignupButton = () => {
    return (
        <a className='button-3 logoutbtn' role='button-3' href='/signup'>Signup</a>
    )
}

function Home() {
    return (
        <div>
            <div className="logo">
                <h1>Frigata: simple key-value store api</h1>
                <img src="https://loco.rs/icon.svg" alt="Loco logo" />
            </div>
            <div className='main'>
                <LoginButton/>
                <SignupButton/>
            </div>
        </div>
        
    )
}

export default Home;