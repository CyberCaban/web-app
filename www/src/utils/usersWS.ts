import { useEffect, useRef, useState } from "react";

export default function useUsersWS(id: string) {
  const [users, setUsers] = useState([]);
  const uWs = useRef<WebSocket | null>(null);

  function updateUsers() {
    if (uWs.current) {
      uWs.current.send(id);
    }
  }

  useEffect(() => {
    const ws = new WebSocket(`ws://localhost:5000/api/ws/users/${id}`);
    ws.onmessage = (event) => {
      setUsers(JSON.parse(event.data));
    };
    uWs.current = ws;

    return () => {
      if (ws) ws.close();
    };
  }, [id]);

  return { users, updateUsers };
}
