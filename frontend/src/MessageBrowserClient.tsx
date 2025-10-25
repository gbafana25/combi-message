import { useEffect, useState } from "react";
import { io } from "socket.io-client";
import "./MessageBrowserClient.css";

const socket = io("http://localhost:5150")

export const MessageBrowserClient = () => {
    const [devicename, setDeviceName] = useState('');
    const [key, setKey] = useState('');
    const [value, setValue] = useState('');
    const [selectedOperation, setSelectedOperation] = useState('');
    const [responseText, setResponse] = useState([]);

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


  }, [])

  const sendRequest = () => {
    if(selectedOperation == "get") {
        socket.emit(selectedOperation, { devicename: devicename})
    } else {
        socket.emit(selectedOperation, { devicename: devicename, key: key, value: value})
    }
    
  }

  return (
    
        <body>
            <div className="box">
                <span className="text-center">Add/Retrieve messages</span>
                <div className="input-container">
                    <select name="operation_type" defaultValue={"get"} onChange={e => setSelectedOperation(e.target.value)}>
                        <option value={"set"}>Set</option>
                        <option value={"get"}>Get</option>
                    </select>
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
                <ul>
                    {responseText.map((e:any) => (
                        <li key={e.key}>{e.key} - {e.value}</li>
                    ))}
                </ul>
            <div>

            </div>
        </body>
        
  )
}
