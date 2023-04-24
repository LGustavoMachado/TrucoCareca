use futures_util::stream::SplitSink;
use tokio::net::{TcpStream};
use tokio_tungstenite::{WebSocketStream};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug)]
pub struct Connection {
  pub sender: SplitSink<WebSocketStream<TcpStream>, Message>
}

impl Connection {
  pub fn new(sender:SplitSink<WebSocketStream<TcpStream>, Message>) -> Self { 
    Self { sender }
  }
}