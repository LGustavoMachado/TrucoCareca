use futures_util::stream::SplitSink;
use tokio::net::{TcpStream};
use tokio_tungstenite::{WebSocketStream};
use tokio_tungstenite::tungstenite::Message;

pub struct Connection {
    pub id: u32,
    pub name: String,
    pub sender: SplitSink<WebSocketStream<TcpStream>, Message>,
}

impl Connection {
  pub fn new(id:u32, sender:SplitSink<WebSocketStream<TcpStream>, Message>) -> Self { 
    Self { id, name: String::from(""), sender }
  }
}