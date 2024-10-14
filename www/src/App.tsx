import { useEffect, useRef, useState } from "react";
import "./App.css";
import { getData, postData } from "./utils/utils";
import VideoStream from "./components/VideoStream";

window.delbnt = false;
const MIME_TYPE = 'video/webm; codecs="vp8, opus"';

function App() {
  const [msg, setMsg] = useState("");
  const [files, setFiles] = useState([]);
  const [imgSrc, setImgSrc] = useState("");
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [isStreaming, setIsStreaming] = useState(false);
  const [isWatching, setIsWatcing] = useState(false);
  const video = useRef<HTMLVideoElement>(null);
  const mediaSrc = useRef<MediaSource | null>(null);
  const mediaRec = useRef<MediaRecorder | null>(null);

  useEffect(() => {}, [window.delbnt]);
  useEffect(() => {
    if (!ws) return;
    // ws.onmessage = (event) => {
    //   // console.log("onmessage", event);
    // };
    ws.onerror = (event) => {
      console.log("onerror", event);
    };
    ws.onclose = (event) => {
      ws.onmessage = null;
      ws.onerror = null;
      ws.onclose = null;
      console.log("onclose", event);
    };

    return () => {
      if (ws) ws.close();
    };
  }, [ws]);

  useEffect(() => {
    if (!isStreaming) return;
    async function streamWS() {
      console.log(window.navigator);

      const isVideoDevices = await navigator.mediaDevices
        .enumerateDevices()
        .then((dev) => dev.some((d) => d.kind === "videoinput"));

      if (!isVideoDevices) {
        console.log("No video devices found");
        return;
      }

      const stream = await navigator.mediaDevices.getDisplayMedia({
        video: true,
        audio: {
          echoCancellation: true,
          noiseSuppression: true,
          sampleRate: 44100,
        },
      });
      const mediaRecorder = (mediaRec.current = new MediaRecorder(stream, {
        mimeType: MIME_TYPE,
      }));
      mediaRecorder.addEventListener("dataavailable", onDataAvailable);
      mediaRecorder.start(2000);
    }
    function onDataAvailable(event: BlobEvent) {
      console.log("onDataAvailable");
      if (event.data && ws && ws.readyState === WebSocket.OPEN) {
        ws.send(event.data);
      }
    }
    streamWS();
    return () => {
      if (mediaRec.current) {
        mediaRec.current.removeEventListener("dataavailable", onDataAvailable);
      }
    };
  }, [isStreaming, ws]);

  useEffect(() => {
    if (!isWatching || !ws || !video.current) return;
    const vid = video.current;
    const mediaSource = (mediaSrc.current = new MediaSource());
    // mediaSource.addEventListener("sourceclose", onSourceClose);
    // mediaSource.addEventListener("error", onSourceError);
    mediaSource.addEventListener("sourceopen", onSourceOpen);
    const objURL = URL.createObjectURL(mediaSource);
    vid.src = objURL;
    vid?.play();

    function onSourceOpen() {
      if (!ws || !mediaSrc.current || !video.current) return;
      const mediaSource = mediaSrc.current;
      console.log("isTypeSupported", MediaSource.isTypeSupported(MIME_TYPE));

      const sourceBuffer = mediaSource.addSourceBuffer(MIME_TYPE);
      ws.addEventListener("message", (event) => {
        onWSMessage(event, sourceBuffer, mediaSource);
      });

      sourceBuffer.addEventListener("updateend", (e) => {
        // console.log("updateend", e);
      });

      sourceBuffer.addEventListener("error", (e) => {
        console.log("error sourceBuffer", e);
      });
    }
    function onSourceClose() {
      console.log("onSourceClose");
    }
    function onSourceError(e) {
      console.log("onSourceError", e);
    }
    function onWSMessage(event, sourceBuf, mediaSource) {
      console.log("onWSMessage", event);
      
      const arrayU8 = new Uint8Array(event.data);
      if (mediaSource.readyState === "open" && !sourceBuf.updating) {
        sourceBuf.appendBuffer(arrayU8);
      } else {
        // if (ws) ws.close();
        console.log("mediaSource.readyState", mediaSource.readyState);
      }
    }
    return () => {
      if (mediaSrc.current) {
        mediaSrc.current.removeEventListener("sourceclose", onSourceClose);
        mediaSrc.current.removeEventListener("error", onSourceError);
        mediaSrc.current.removeEventListener("sourceopen", onSourceOpen);
      }
      if (ws) ws.close();
    };
  }, [isWatching, ws]);

  function startWS() {
    const s = new WebSocket("ws://localhost:5000/api/stream/ws");
    s.binaryType = "arraybuffer";
    setWs(s);
  }

  return (
    <>
      <div className="card">
        <pre style={{ textAlign: "left" }}>{msg}</pre>
        <VideoStream />
        {/* <button onClick={startWS}>Start WS</button>
        <button onClick={() => setIsStreaming(!isStreaming)}>Stream WS</button>
        <button onClick={() => setIsWatcing(!isWatching)}>
          Watch WS Stream
        </button>
        <video src="" id="video" ref={video} autoPlay controls></video>
        <button onClick={() => video.current?.play()}>Play</button> */}

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
