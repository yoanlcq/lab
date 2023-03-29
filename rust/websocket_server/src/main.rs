use std::collections::HashMap;
use std::sync::Arc;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc as tokio_mpsc;
use tokio::sync::Mutex as TokioMutex;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::Filter;
use warp::ws as wws;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: tokio_mpsc::UnboundedSender<Result<wws::Message, warp::Error>>,
}

type Clients = Arc<TokioMutex<HashMap<String, Client>>>;

fn main() {
    let clients: Clients = Arc::new(TokioMutex::new(HashMap::new()));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || clients.clone()))
        .and_then(ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    let server = warp::serve(routes);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(server.run(([127, 0, 0, 1], 8000)));
}

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl warp::Reply, warp::Rejection> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| client_connection(socket, clients)))
}

pub async fn client_connection(ws: wws::WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = tokio_mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: client_sender,
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(&uuid, msg, &clients).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(client_id: &str, msg: wws::Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        let locked = clients.lock().await;
        for (it_client_id, client) in locked.iter() {
            println!("sending pong");
            let _ = client.sender.send(Ok(wws::Message::text(format!("pong, your client ID is {}, we received a message from {}", it_client_id, client_id))));
        }
    }
}
