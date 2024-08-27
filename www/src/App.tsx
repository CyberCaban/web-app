import { useState } from "react";
import "./App.css";

function App() {
  const [msg, setMsg] = useState("");
  const [name, setName] = useState("");

  return (
    <>
      <div className="card">
        <h1>{msg}</h1>
        <p
          onClick={() =>
            fetch("/api/hello")
              .then((res) => {
                return res.json();
              })
              .then((data) => {
                console.log(data);
                setMsg(data);
              })
          }
        >
          Click to test GET api!
        </p>
        <input
          type="text"
          name="name"
          id="name"
          onChange={(e) => setName(e.target.value)}
        />
        <p
          onClick={() =>
            fetch("/api/hello", {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({ msg: name }),
            })
              .then((res) => res.json())
              .then((data) => {
                console.log(data);
                setMsg(data.msg);
              })
          }
        >
          Click to test POST api!{" "}
        </p>
      </div>
    </>
  );
}

export default App;
