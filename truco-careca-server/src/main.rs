use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use futures_util::StreamExt;

mod game;

use game::Game;
use game::connection::Connection;

pub enum Event {
  Join(Connection),
  Quit(u32),
  StateOut(String),
  ChangePlayerName(u32, String),
  SendChatMessage(u32, String),
}

#[tokio::main]
async fn main() {

  // start server, listen to port 8080
  let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
  let listener = TcpListener::bind(addr).await.unwrap();
  println!("Server listening on {}", addr);

  let (game_sender, game_receiver) = mpsc::unbounded_channel::<Event>();
  tokio::spawn(run(game_receiver));

  let mut id = 1;

  loop {
    // Wait for client connection
    let (stream, _) = listener.accept().await.unwrap();

    // Try accepting the connection
    let ws_stream = accept_async(stream).await.expect("Failed to accept");

    // Player connection
    tokio::spawn(connect_player(id, ws_stream, game_sender.clone()));

    id += 1;
  }
}

pub async fn run(mut game_receiver: UnboundedReceiver<Event>) {
  let mut game_instance = Game::new();

  while let Some(event) = game_receiver.recv().await {
    match event {
      Event::Join(conn) => {
        match game_instance.add_player(conn){
          Ok(()) => {},
          Err(error) => { println!("Error: {}", error) }
        }
      }
      Event::ChangePlayerName(id, name) => {
        game_instance.change_player_name(id, name);
      }
      Event::SendChatMessage(id, message) => {
        game_instance.send_message(id, message).await;
      }
      Event::Quit(_) => {}
      Event::StateOut(_) => {}
    }   
  }
}

async fn connect_player(id: u32, ws_stream: WebSocketStream<TcpStream>, unbounded_sender: UnboundedSender<Event>) {
  let (sender, mut receiver) = ws_stream.split();
  let mut step = 0;
  let conn = Connection::new(id, sender);
  match unbounded_sender.send(Event::Join(conn)) {
    Ok(()) => {},
    Err(_) => {}
  }

  while let Some(Ok(message)) = receiver.next().await {
    if message.is_text() {
    let message_str = message.to_text().unwrap();

      if step == 0 {
        let name = String::from(message_str);
        match unbounded_sender.send(Event::ChangePlayerName(id, name)) {
          Ok(()) => {},
          Err(_) => {}
        }
      } else {
        match unbounded_sender.send(Event::SendChatMessage(id, String::from(message_str))) {
          Ok(()) => {},
          Err(_) => {}
        }
      }
      
      step += 1;
    }
  }
}

// async fn listen(ws_stream: WebSocketStream<TcpStream>, unbounded_sender: UnboundedSender<Event>, id: u32) {
//   println!("WebSocket connection established");

//   let (sender, mut receiver) = ws_stream.split();
//   let conn = Connection::new(id, sender);
//   unbounded_sender.send(Event::Join(conn));
//   while let Some(Ok(msg)) = receiver.next().await {

//     if msg.is_text() {
//       println!("Received message: {}", msg.to_text().unwrap());

//       let message = String::from(msg.to_text().unwrap());
//       unbounded_sender.send(Event::StateOut(message));
//     }
//   }
// }

// async fn broadcast(mut rx: UnboundedReceiver<Event>) {

//   let mut connections: HashMap<u32, Connection> = HashMap::new();

//   while let Some(event) = rx.recv().await {
//     match event {
//       Event::Join(conn) => {
//         connections.insert(conn.id, conn);
//       }
//       Event::Quit(id) => {
//         connections.remove(&id);
//       }
//       Event::StateOut(message) => {
//         for(id, conn) in connections.iter_mut() {
//           println!("{}", id);
//           let _ = conn.sender.send(Message::Text(message.clone())).await.expect("Error sending message");
//           println!("Mensagem enviada");
//         }
//       }
//     }   
//   }
// }

