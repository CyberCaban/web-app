use crate::routes::ws_routes::{Message, WSPeers};
use rocket::{futures::{SinkExt, StreamExt}, tokio::select, State};

#[get("/ws/users/<user_id>")]
pub fn test_ws(ws: ws::WebSocket, user_id: String, peers: &State<WSPeers>) -> ws::Channel<'static> {
    let peers = peers.inner().clone();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    msg = stream.next() => match msg {
                        Some(Ok(msg)) => {
                            let users = peers.inner().await.keys().cloned().filter(|u| u != &user_id).collect::<Vec<_>>();
                            let _ = stream.send(Message::Text(format!("{:?}", users))).await;
                            // if let Message::Text(text) = msg {
                            // }
                        },
                        Some(Err(e)) => {
                            eprintln!( "{} error: {}", user_id, e);
                            break;
                        },
                        None => break,
                    }, 
                    else => break,
                }
            }
            Ok(())
        })
    })
}

#[get("/stream/ws/<user_id>")]
pub async fn stream_ws(
    ws: ws::WebSocket,
    user_id: String,
    peers: &State<WSPeers>,
) -> ws::Channel<'static> {
    use rocket::futures::StreamExt;
    use rocket::tokio::sync::mpsc::channel;
    let peers = peers.inner().clone();

    println!("{} connected", user_id);
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let (tx, rx) = channel(1);
            peers.inner().await.insert(user_id.clone(),tx);
            while let Some(msg) = stream.next().await {
                let msg: Message = msg?;
                match msg {
                    Message::Text(text) => {
                        println!("{} sent: {:?}", user_id, text);
                        let _ = stream.send(Message::Text(text)).await;
                    }
                    Message::Binary(bin) => {
                        println!("{:?}", bin.len());
                        println!("{:?}", peers.inner().await.keys());
                        let _ = stream.send(Message::Binary(bin)).await;
                    },
                    _ => {}
                }
            }
            println!("{} disconnected", user_id);
            peers.inner().await.remove(&user_id);
            Ok(())
        })
    })
}

#[get("/watch/ws/<user_id>")]
pub async fn watch_ws(
    ws: ws::WebSocket,
    user_id: String,
    peers: &State<WSPeers>,
) -> ws::Channel<'static> {
    let peers = peers.inner().clone();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            // let mut binding = peers.inner().await;
            // let rx = binding.get_mut(&user_id).unwrap(); // an id to watch from
            // loop {
            //     let msg = rx.recv().await;
            //         match msg {
            //             Some(msg) => {
            //                 println!("receiving: {:?}", msg);
            //                 let _ = stream.send(msg).await;
            //             },
            //             None => break,
            //         }          
            // }
            Ok(())
        })
    })
}