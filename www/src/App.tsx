import { useState } from "react";
import "./App.css";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p
          onClick={() =>
            fetch("/api/hello")
              .then((res) => {
                return res.json();
              })
              .then((data) => {
                console.log(data);
              })
          }
        >
          Click to test api!
        </p>
      </div>
    </>
  );
}

export default App;
