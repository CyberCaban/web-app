import { useEffect, useState } from "react";
import "./App.css";
import { getData, postData } from "./utils/utils";

window.delbnt = false;

function App() {
  const [msg, setMsg] = useState("");
  const [files, setFiles] = useState([]);
  const [imgSrc, setImgSrc] = useState("");
  const [ws, setWs] = useState<WebSocket | null>(null);

  useEffect(() => {
    if (!ws) return;
    ws.onmessage = (event) => {
      console.log("onmessage", event);
      setMsg(event.data);
    };
    ws.onerror = (event) => {
      console.log("onerror", event);
    };
    ws.onclose = (event) => {
      console.log("onclose", event);
    };

    return () => {
      if (ws) ws.close();
    };
  }, [ws]);

  useEffect(() => {}, [window.delbnt]);

  function startWS() {
    setWs(new WebSocket("ws://localhost:5000/api/ws"));
  }
  function testWS(msg: string) {
    if (ws) {
      let i = 0;
      const interval = setInterval(() => {
        ws.send(JSON.stringify({ msg: `${i}: ${msg}` }));
        i++;
        if (i > 5) {
          clearInterval(interval);
        }
      }, 1000);
    }
  }

  return (
    <>
      <div className="card">
        <pre style={{ textAlign: "left" }}>{msg}</pre>
        <button onClick={startWS}>Start WS</button>
        <button onClick={() => testWS("darowa")}>Test WS</button>

        <div className="flex flex-row gap-2">
          <form
            className="create-user-form"
            onSubmit={(e) => {
              e.preventDefault();
              postData("/api/register", {
                username: e.target.username.value,
                password: e.target.password.value,
              }).then((data) => {
                console.log(data);
              });
            }}
          >
            <h1>Register</h1>
            <label htmlFor="username">Username</label>
            <input type="text" name="username" id="username" />
            <label htmlFor="password">Password</label>
            <input type="password" name="password" id="password" />
            <button type="submit">Register</button>
          </form>
          <form
            className="flex flex-col p-5"
            onSubmit={(e) => {
              e.preventDefault();
              postData("/api/login", {
                username: e.target.login_username.value,
                password: e.target.login_password.value,
              }).then((data) => {
                console.log(data);
              });
            }}
          >
            <h1>Login</h1>
            <label htmlFor="login_username">Username</label>
            <input type="text" name="login_username" id="login_username" />
            <label htmlFor="login_password">Password</label>
            <input type="password" name="login_password" id="login_password" />
            <button type="submit">Login</button>
          </form>
        </div>
        <button onClick={() => postData("/api/logout", {})}>Logout</button>

        {files &&
          files.map((file) => (
            <div key={file}>
              <a href={`/api/file/${file}`}>{file}</a>
              {window.delbnt && (
                <button
                  className="ml-2 px-2 py-1 bg-red-500 text-white rounded-md"
                  onClick={() => {
                    fetch(`/api/file/${file}`, { method: "DELETE" });
                    getData("/api/files")
                      .then((res) => {
                        if (res.error_msg) {
                          throw new Error(res.error_msg);
                        }
                        console.log(res);
                        setMsg(JSON.stringify(res, null, 2));
                        setFiles(res);
                      })
                      .catch((err) => console.error(err));
                  }}
                >
                  delete
                </button>
              )}
            </div>
          ))}
        <button
          onClick={() =>
            getData("/api/files")
              .then((res) => {
                if (res.error_msg) {
                  throw new Error(res.error_msg);
                }
                console.log(res);
                setMsg(JSON.stringify(res, null, 2));
                setFiles(res);
              })
              .catch((err) => console.error(err))
          }
        >
          Get all files
        </button>

        <form
          className="upload-form"
          encType="multipart/form-data"
          onSubmit={(e) => {
            e.preventDefault();
            const target = e.target as HTMLFormElement;
            const formData = new FormData(target);
            formData.append("file", target.file.files[0]);
            formData.append("filename", target.filename.value);
            formData.append("is_private", target.is_private.checked);
            fetch("/api/file/create", {
              method: "POST",
              body: formData,
            })
              .then((res) => res.json())
              .then((res) => console.log(res));
          }}
        >
          <label htmlFor="file">File</label>
          <input type="file" name="file" id="file" />
          <label htmlFor="filename">Filename</label>
          <input type="text" name="filename" id="filename" />
          <label htmlFor="is_private">Private</label>
          <input
            type="checkbox"
            name="is_private"
            id="is_private"
            defaultChecked={true}
          />
          <input type="submit" value="Submit" />
        </form>
      </div>
    </>
  );
}

export default App;
