import { useEffect, useRef, useState } from "react";
import useUsersWS from "../utils/usersWS";
import useVideoWS from "../utils/videoWS";

const MIME_TYPE_VIDEO_AUDIO = 'video/webm;codecs="vp8,opus"';
const MIME_TYPE_VIDEO_ONLY = 'video/webm; codecs="vp9"';
const MIME_TYPE_AUDIO_ONLY = 'audio/webm; codecs="opus"';

export default function VideoStream() {
  const [isStreaming, setIsStreaming] = useState(false);
  const [isWatching, setIsWatching] = useState(false);

  const mediaRecorder = useRef<MediaRecorder | null>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const mediaSource = useRef<MediaSource | null>(null);
  const videoBuf = useRef<SourceBuffer | null>(null);
  const audioBuf = useRef<SourceBuffer | null>(null);
  const video = useRef<HTMLVideoElement>(null);
  const socketid = useRef((Math.random() * 36).toString(36).substring(2));
  const { users, updateUsers } = useUsersWS(socketid.current);
  const videoWS = useVideoWS(socketid.current);

  useEffect(() => {
    console.log("isStreaming", isStreaming);
  }, [isStreaming]);

  useEffect(() => {
    return () => {};
  }, []);

  useEffect(() => {
    function toggleWatch() {
      console.log("toggleWatch", isWatching);

      const vid = video.current;
      const mediaSourceRef = isWatching
        ? (mediaSource.current = new MediaSource())
        : (mediaSource.current = null);
      if (!mediaSourceRef || !vid) return;
      if (isWatching) {
        mediaSourceRef.onsourceopen = onSourceOpen;
        const objURL = URL.createObjectURL(mediaSourceRef);
        vid.src = objURL;
        videoWS?.send(objURL);
      } else {
        console.log("onSourceClose");
        mediaSourceRef.onsourceopen = null;
        vid.src = "";
      }

      function onSourceOpen() {
        if (!mediaSourceRef) return;
        const mediaSource = mediaSourceRef;
        console.log(
          "isTypeSupported",
          MediaSource.isTypeSupported('video/webm;codecs="vp9"')
        );
        const vidSrcBuffer = (videoBuf.current =
          mediaSource.addSourceBuffer(MIME_TYPE_VIDEO_ONLY));
        // IMPORTANT:
        vidSrcBuffer.mode = "sequence";
        // const audioSrcBuffer = (audioBuf.current =
        //   mediaSource.addSourceBuffer(MIME_TYPE_AUDIO_ONLY));

        // eslint-disable-next-line no-constant-condition
        if (videoWS) {
          videoWS.onmessage = (event) => {
            if (typeof event.data === "string") {
              console.log("event.data", event.data);
              return;
            }

            const arrayU8 = new Uint8Array(event.data);
            console.log(arrayU8.length);
            if (arrayU8.length === 0) {
              vidSrcBuffer.dispatchEvent(new Event("close"));
            }

            if (vidSrcBuffer && videoBuf.current && !vidSrcBuffer.updating) {
              try {
                vidSrcBuffer.appendBuffer(arrayU8);
              } catch (e) {
                console.log("no sourceBuffer", e);
              }
            }
          };
          vidSrcBuffer.onupdateend = (e) => {
            // console.log("updateend", e);
          };
          vidSrcBuffer.onerror = (e) => {
            console.log("error sourceBuffer", mediaSource);
          };
        }
      }
    }
    toggleWatch();
  }, [isWatching]);

  useEffect(() => {
    function getMediaStream() {
      return navigator.mediaDevices.getDisplayMedia({
        video: true,
        // audio: true,
      });
    }
    function toggleStreaming() {
      let mediaRecorderRef = mediaRecorder.current;

      if (isStreaming) {
        getMediaStream()
          .then((mediaStream) => {
            // const audioContext = new AudioContext();
            // const emptyAudioTrack = audioContext
            //   .createMediaStreamDestination()
            //   .stream.getAudioTracks()[0];
            // mediaStream.addTrack(emptyAudioTrack);
            console.log("stream", mediaStream.getAudioTracks());

            // eslint-disable-next-line no-constant-condition, no-empty
            if (mediaStream.getAudioTracks().length === 0 && 0) {
            }
            streamRef.current = mediaStream;
            mediaRecorderRef = new MediaRecorder(mediaStream, {
              mimeType: MIME_TYPE_VIDEO_ONLY,
            });

            mediaRecorderRef.ondataavailable = (event) => {
              if (videoWS) {
                videoWS.send(event.data);
              } else {
                console.log("videoWS.current", videoWS);
              }
            };
            mediaRecorderRef.start(1000);
            mediaRecorderRef.onstop = () => {
              setIsStreaming(false);
            };
          })
          .catch((e) => {
            console.error(e);
            setIsStreaming(false);
          });
      } else {
        if (streamRef.current) {
          streamRef.current.getTracks().forEach((track) => track.stop());
          videoWS?.send(new Uint8Array(0));
        }

        //   if (mediaRecorder.current) {
        //     mediaRecorder.current.ondataavailable = null;
        //     mediaRecorder.current.stop();
        //     mediaRecorder.current = null;
        //   }
      }
    }
    toggleStreaming();
  }, [isStreaming]);

  return (
    <>
      <button onClick={() => setIsStreaming(!isStreaming)}>
        Toggle Streaming
      </button>
      <button onClick={() => setIsWatching(!isWatching)}>Toggle Watch</button>
      <button onClick={updateUsers}>Update Users</button>
      {users.map((u) => (
        <div
          key={u}
          onClick={() => {
            videoWS?.send(u);
            // const s = new WebSocket(`ws://localhost:5000/api/watch/ws/${u}`);
            // s.binaryType = "arraybuffer";
            // s.onmessage = (event) => {
            //   const arrayU8 = new Uint8Array(event.data);
            //   console.log(arrayU8.length);
            //   if (videoBuf.current && !videoBuf.current.updating) {
            //     videoBuf.current.appendBuffer(arrayU8);
            //   }
            // };
            // s.onopen = () => {
            //   s.close()
            // }
          }}
        >
          {u}
        </div>
      ))}
      <video
        autoPlay
        // controls
        muted
        id="video"
        ref={video}
        onClick={() => {
          if (!video.current) return;
          video.current.play();
        }}
      />
    </>
  );
}
