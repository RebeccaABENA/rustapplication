use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use std::sync::{Arc};
use tokio::sync::Mutex;

type Tx = tokio::sync::mpsc::UnboundedSender<tungstenite::Message>;
type PeerMap = Arc<Mutex<Vec<Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!(" Serveur WebSocket en écoute sur ws://127.0.0.1:8080");

    let peers: PeerMap = Arc::new(Mutex::new(Vec::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let peers = peers.clone();
        tokio::spawn(async move {
            handle_connection(stream, peers).await;
        });
    }
}

async fn handle_connection(stream: TcpStream, peers: PeerMap) {
    let ws_stream = accept_async(stream).await.unwrap();
    println!(" Nouvelle connexion ");

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<tungstenite::Message>();

    peers.lock().await.push(tx.clone());

    let peers_clone = peers.clone();
    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if write.send(msg).await.is_err() {
                break;
            }
        }
    });

    while let Some(msg) = read.next().await {
        let msg = msg.unwrap();
        println!(" Message reçu : {:?}", msg);

        // Diffuse à tous les clients
        let peers_locked = peers_clone.lock().await;
        for peer in peers_locked.iter() {
            let _ = peer.send(msg.clone());
        }
    }

    write_task.abort();
}