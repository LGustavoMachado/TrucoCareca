use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::StreamExt;
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use std::collections::HashMap;

pub enum Event {
  Join(Connection),
  Quit(u32),
  StateOut(String),
}

pub struct Connection {
  pub id: u32,
  pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl Connection {
  pub fn new(id:u32, sender:SplitSink<WebSocketStream<TcpStream>, Message>) -> Self { 
    Self { id, sender }
  }
}

#[tokio::main]
async fn main() {
    
  let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
  let listener = TcpListener::bind(addr).await.unwrap();
  println!("Server listening on {}", addr);

  let (sender, receiver) = mpsc::unbounded_channel::<Event>();
  tokio::spawn(broadcast(receiver));

  let mut id = 0;

  loop {
    let (stream, _) = listener.accept().await.unwrap();
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
                          
    id += 1;
    tokio::spawn(listen(ws_stream, sender.clone(), id));
  }
    
}

async fn listen(ws_stream: WebSocketStream<TcpStream>, unbounded_sender: UnboundedSender<Event>, id: u32) {
  println!("WebSocket connection established");

  let (sender, mut receiver) = ws_stream.split();
  let conn = Connection::new(id, sender);
  unbounded_sender.send(Event::Join(conn));
  while let Some(Ok(msg)) = receiver.next().await {

    if msg.is_text() {
      println!("Received message: {}", msg.to_text().unwrap());

      let message = String::from(msg.to_text().unwrap());
      unbounded_sender.send(Event::StateOut(message));
    }
  }
}

async fn broadcast(mut rx: UnboundedReceiver<Event>) {

  let mut connections: HashMap<u32, Connection> = HashMap::new();

  while let Some(event) = rx.recv().await {
    match event {
      Event::Join(conn) => {
        connections.insert(conn.id, conn);
      }
      Event::Quit(id) => {
        connections.remove(&id);
      }
      Event::StateOut(message) => {
        for(id, conn) in connections.iter_mut() {
          println!("{}", id);
          let _ = conn.sender.send(Message::Text(message.clone())).await.expect("Error sending message");
          println!("Mensagem enviada");
        }
      }
    }
    
  }

}

