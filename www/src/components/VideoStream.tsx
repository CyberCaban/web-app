import { useEffect, useRef, useState } from "react";

const MIME_TYPE_VIDEO_AUDIO = 'video/webm;codecs="vp8,opus"';
const MIME_TYPE_VIDEO_ONLY = 'video/webm; codecs="vp8"';

export default function VideoStream() {
  const [isStreaming, setIsStreaming] = useState(false);
  const [isWatching, setIsWatching] = useState(false);

  const mediaRecorder = useRef<MediaRecorder | null>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const mediaSource = useRef<MediaSource | null>(null);
  const srcBuffer = useRef<SourceBuffer | null>(null);
  const video = useRef<HTMLVideoElement>(null);
  const videoWS = useRef<WebSocket | null>(null);
  const socketid = useRef((Math.random() * 36).toString(36).substring(2));

  useEffect(() => {
    console.log("isStreaming", isStreaming);
  }, [isStreaming]);

  useEffect(() => {
    const s = new WebSocket(
      `ws://localhost:5000/api/stream/ws/${socketid.current}`
    );
    s.binaryType = "arraybuffer";
    videoWS.current = s;
    return () => {
      if (s) s.close();
    };
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
        const sourceBuffer = (srcBuffer.current = mediaSource.addSourceBuffer(
          MIME_TYPE_VIDEO_AUDIO
        ));
        // IMPORTANT:
        sourceBuffer.mode = "sequence";

        if (videoWS.current) {
          videoWS.current.onmessage = (event) => {
            const arrayU8 = new Uint8Array(event.data);
            console.log(arrayU8.length);
            if (arrayU8.length === 0) {
              sourceBuffer.dispatchEvent(new Event("close"));
            }

            if (
              srcBuffer.current &&
              mediaSource.readyState === "open" &&
              !sourceBuffer.updating
            ) {
              try {
                srcBuffer.current.appendBuffer(arrayU8);
              } catch (e) {
                console.log("no sourceBuffer", e);
              }
            }
          };
          sourceBuffer.onupdateend = (e) => {
            //   console.log("updateend", e);
          };
          sourceBuffer.onerror = (e) => {
            console.log("error sourceBuffer", mediaSource);
          };
        }
      }
    }
    toggleWatch();
  }, [isWatching]);

  useEffect(() => {
    function toggleStreaming() {
      let mediaRecorderRef = mediaRecorder.current;

      if (isStreaming) {
        const supported = navigator.mediaDevices.getSupportedConstraints();
        navigator.mediaDevices
          .getDisplayMedia({
            video: {
              aspectRatio: 16 / 9,
            },
            audio: {
              echoCancellation: supported.echoCancellation ? true : false,
              noiseSuppression: supported.noiseSuppression ? true : false,
              sampleRate: 48000,
            },
          })
          .then((stream) => {
            // eslint-disable-next-line no-constant-condition
            if (stream.getAudioTracks().length === 0 && 0) {
              const audioContext = new AudioContext();
              const emptyAudioTrack = audioContext
                .createMediaStreamDestination()
                .stream.getAudioTracks()[0];
              stream.addTrack(emptyAudioTrack);
            }
            streamRef.current = stream;
            mediaRecorderRef = new MediaRecorder(stream, {
              mimeType: MIME_TYPE_VIDEO_AUDIO,
            });
            
            mediaRecorderRef.ondataavailable = (event) => {
              if (videoWS.current) {
                videoWS.current.send(event.data);
              } else {
                console.log("videoWS.current", videoWS.current);
                videoWS.current = null;
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
          videoWS.current?.send(new Uint8Array(0));
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
      <video
        autoPlay
        id="video"
        ref={video}
        controls
        onClick={() => {
          if (!video.current) return;
          video.current.play();
        }}
      />
    </>
  );
}
