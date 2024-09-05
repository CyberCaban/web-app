import { useState } from "react";
import "./App.css";
import { getData, postData } from "./utils/utils";

function App() {
  const [msg, setMsg] = useState("");
  const [files, setFiles] = useState([]);
  const [imgSrc, setImgSrc] = useState("");

  return (
    <>
      <div className="card">
        <pre style={{ textAlign: "left" }}>{msg}</pre>

        <button
          onClick={() =>
            fetch("/api/get_users")
              .then((res) => res.json())
              .then((data) => {
                console.log(JSON.stringify(data, null, 2));
                setMsg(JSON.stringify(data, null, 2));
              })
          }
        >
          Click to test db!
        </button>

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
          </form>
          <form
            className="flex flex-col border-white border p-5"
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
            <button className="hidden" type="submit">Login</button>
          </form>
        </div>
        <button onClick={() => postData("/api/logout", {})}>Logout</button>

        {files &&
          files.map((file) => (
            <div key={file}>
              <a href={`/api/file/${file}`}>{file}</a>
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
          <input type="submit" value="Submit" />
        </form>
      </div>
    </>
  );
}

export default App;
