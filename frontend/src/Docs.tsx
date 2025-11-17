import "./Docs.css";

export const Docs = () => {
    return (
        <div className="main-container">
            <h1>
                Documentation
            </h1>
            <p>Both the REST API and the Socket.IO websockets give access to the same functions.</p>
            <p>Available at <code>/api/messages</code></p>

            <h2>
                REST API
            </h2>

            
            <code className="endpoint-name">/get/&#123;device_name&#125;</code>
            <br></br>
            Get all messages for a given device
            <div className="endpoint-description">
                <table className="endpoint-table">
                    <th>Parameter</th>
                    <th>Description</th>
                    <th>Type</th>
                    <th>Optional</th>
                    <tbody>
                        <tr>
                            <td>device_name</td>
                            <td>Name of device, primary identifier</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>api_key</td>
                            <td>User API key, will return both public and private messages if used</td>
                            <td>String</td>
                            <td>Yes</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <code className="endpoint-name">/get-latest/&#123;device_name&#125;</code>
            <br></br>
            Get latest message for a given device
            <div className="endpoint-description">
                <table className="endpoint-table">
                    <th>Parameter</th>
                    <th>Description</th>
                    <th>Type</th>
                    <th>Optional</th>
                    <tbody>
                        <tr>
                            <td>device_name</td>
                            <td>Name of device, primary identifier</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>api_key</td>
                            <td>User API key, will return most recent private message if used</td>
                            <td>String</td>
                            <td>Yes</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <code className="endpoint-name">/set/&#123;device_name&#125;</code>
            <br></br>
            Create or modify message
            <div className="endpoint-description">
                <table className="endpoint-table">
                    <th>Parameter</th>
                    <th>Description</th>
                    <th>Type</th>
                    <th>Optional</th>
                    <tbody>
                        <tr>
                            <td>device_name</td>
                            <td>Name of device, primary identifier</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>key</td>
                            <td>name of message</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>value</td>
                            <td>message information</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>api_key</td>
                            <td>User API key, will return most recent private message if used</td>
                            <td>String</td>
                            <td>Yes</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <h2>SocketIO</h2>
            <code className="endpoint-name">get</code>
            <br></br>
            Get all messages for device
            <div className="endpoint-description">
                <table className="endpoint-table">
                    <th>Parameter</th>
                    <th>Description</th>
                    <th>Type</th>
                    <th>Optional</th>
                    <tbody>
                        <tr>
                            <td>devicename</td>
                            <td>Name of device, primary identifier</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>apikey</td>
                            <td>User API key, will return both public and private messages if used</td>
                            <td>String</td>
                            <td>Yes</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <code className="endpoint-name">set</code>
            <br></br>
            Creates new message if it doesn't exist, updates value if it does
            <div className="endpoint-description">
                <table className="endpoint-table">
                    <th>Parameter</th>
                    <th>Description</th>
                    <th>Type</th>
                    <th>Optional</th>
                    <tbody>
                        <tr>
                            <td>devicename</td>
                            <td>Name of device, primary identifier</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>key</td>
                            <td>name of message</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>value</td>
                            <td>message information</td>
                            <td>String</td>
                            <td>No</td>
                        </tr>
                        <tr>
                            <td>apikey</td>
                            <td>User API key, will return both public and private messages if used</td>
                            <td>String</td>
                            <td>Yes</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <code className="endpoint-name">error</code>
            <br></br>
            <p>Returns any errors and displays a error message</p>
        </div>
        
    )
}