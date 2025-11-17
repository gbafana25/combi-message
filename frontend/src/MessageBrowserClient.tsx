import { useEffect, useState } from "react";
import { io } from "socket.io-client";
import Cookies from 'js-cookie';
import "./MessageBrowserClient.css";
import { Snackbar } from "@material-ui/core";

const socket = io("http://localhost:5150")

export const MessageBrowserClient = () => {
    const [devicename, setDeviceName] = useState('');
    const [key, setKey] = useState('');
    const [value, setValue] = useState('');
    const [selectedOperation, setSelectedOperation] = useState('');
    const [responseText, setResponse] = useState([]);
    const [privateCheck, setCheck] = useState(false);
    const [snackbarOpen, setOpen] = useState(false);

    const handleCheckbox = () => {
        setCheck(!privateCheck);
    }

    useEffect(() => {
        setSelectedOperation('get');
    
    socket.on("connected", (d) => {
      console.log("Connected to socketio server "+d);
    });

    socket.on("set-return", (data) => {
      console.log("return received "+data.key+" "+data.value);
    })

    socket.on("get-return", (data) => {
        console.log("return received "+data.key+" "+data.value);
        setResponse(data);
    })

    socket.on("error", (error) => {
        console.log("Error: "+error);
        // open snackbar
        setOpen(true);
    })


  }, [])

  const handleClose = () => {
    setOpen(false);
  }

  const sendRequest = () => {
    if(selectedOperation == "get") {
        if(privateCheck) {
            //console.log(Cookies.get("apikey"))
            socket.emit(selectedOperation, { devicename: devicename, apikey: Cookies.get("apikey")})
        } else {
            socket.emit(selectedOperation, { devicename: devicename, apikey: ""})
        }
    } else {
        if(privateCheck) {
            socket.emit(selectedOperation, { devicename: devicename, key: key, value: value, apikey: Cookies.get("apikey")})
        }
        socket.emit(selectedOperation, { devicename: devicename, key: key, value: value, apikey: ""})
    }
    
  }

  return (
    
        <body>
            <div className="box">
                <span className="text-center">Add/Retrieve messages</span>
                <div className="container">
                    <div className="select">
                        <select className="custom-select-sources" name="operation_type" defaultValue={"get"} onChange={e => setSelectedOperation(e.target.value)}>
                            <option value={"set"}>Set</option>
                            <option value={"get"}>Get</option>
                        </select>
                    </div>
                    <div>
                        <label htmlFor="private-check">Private</label>
                        <input id="private-check" type="checkbox" checked={privateCheck} onChange={handleCheckbox}/>
                    </div>
                    
                </div>
                <div className="input-container">
                    <label htmlFor="devicename">Device Name</label>
                    <input
                        value={devicename}
                        onChange={(e) => setDeviceName(e.target.value)}
                        required
                        name="devicename"
                        type="text"
                    />
                </div>
                
                <div hidden={selectedOperation=="get"} className="input-container">
                    <label htmlFor="value">Key</label>
                    <input
                        value={key}
                        onChange={(e) => setKey(e.target.value)}
                        name="key"
                        type="text"
                        required
                    />
                </div>

                <div hidden={selectedOperation=="get"} className="input-container">
                    <label htmlFor="value">Value</label>
                    <input
                        value={value}
                        onChange={(e) => setValue(e.target.value)}
                        name="value"
                        type="text"
                        required
                    />
                </div>
        
                
                <button className="btn" onClick={sendRequest}>Send</button>
            </div>
                <div className="outputbox">
                    <ul>
                        {responseText.map((e:any) => (
                            <li key={e.key}>{e.key}: {e.value}</li>
                        ))}
                    </ul>
                </div>
                
            <div>

            </div>
            <Snackbar
                open={snackbarOpen}
                autoHideDuration={5000}
                onClose={handleClose}
                message="Error processing request"
            />
        </body>
        
  )
}
