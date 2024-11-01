import { useEffect, useRef } from "react";

export default function useVideoWS(id: string) {
  const videoWS = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket(`ws://localhost:5000/api/stream/ws/${id}`);
    ws.binaryType = "arraybuffer";
    videoWS.current = ws;
  }, [id]);

  return videoWS.current;
}
