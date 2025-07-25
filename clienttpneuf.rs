use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Échec de connexion");
    println!(" Connecté au serveur WebSocket");

    let (mut write, mut read) = ws_stream.split();

    // Lire depuis stdin et envoyer
    let stdin = BufReader::new(stdin());
    let mut lines = stdin.lines();

    let write_task = tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            if write.send(tokio_tungstenite::tungstenite::Message::Text(line)).await.is_err() {
                break;
            }
        }
    });

    // Lire depuis le serveur
    let read_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(tokio_tungstenite::tungstenite::Message::Text(txt)) => {
                    println!(" Message du serveur : {}", txt);
                }
                Ok(tokio_tungstenite::tungstenite::Message::Binary(bin)) => {
                    println!(" Message binaire : {:?}", bin);
                }
                _ => {}
            }
        }
    });

    let _ = tokio::join!(write_task, read_task);
}