use futures_util::StreamExt;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

mod game;

use game::game_event::get_event_from_command;
use game::connection::Connection;
use game::game_event::GameEvent;
use game::state_machine::StateMachine;
use game::states::waiting_for_player_state::WaitingForPlayersState;
use game::Game;

#[derive(Debug)]
pub enum Event {
    Join(u32, Connection),
    Input(u32, TrucoCommand),
    Quit(u32),
    ChangePlayerName(u32, String),
    SendChatMessage(u32, String),
    ReadyToPlay(u32),
}

#[tokio::main]
async fn main() {
    // start server, listen to port 8080
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on {}", addr);

    let (game_sender, game_receiver) = mpsc::unbounded_channel::<Event>();

    // Spawn a new game
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
    let mut state_machine = StateMachine::new(Box::new(WaitingForPlayersState::new()));

    while let Some(event) = game_receiver.recv().await {
        match event {
            Event::Join(id, conn) => {
                state_machine.update(&mut game_instance, GameEvent::PlayerJoined(id, conn));
            }
            Event::Input(id, command) => {
                let game_event = get_event_from_command(id, command).unwrap();
                state_machine.update(&mut game_instance, game_event);
            }
            Event::Quit(_) => {}
            _ => {}
        }

        game_instance.output_mut().reverse();

        while let Some((id, message)) = game_instance.output_mut().pop() {
            let (connection, _) = game_instance.get_player_mut(id).unwrap();
            match connection.sender.send(Message::Text(message.clone())).await {
                Ok(_) => { println!("Message sent successfully {}", message) }
                Err(_) => {}
            }
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct TrucoCommand {
    name: String,
    body: serde_json::Value,
}

async fn connect_player(
    id: u32,
    ws_stream: WebSocketStream<TcpStream>,
    unbounded_sender: UnboundedSender<Event>,
) {
    let (sender, mut receiver) = ws_stream.split();
    let conn = Connection::new(sender);

    match unbounded_sender.send(Event::Join(id, conn)) {
        Ok(()) => {}
        Err(_) => {}
    }

    while let Some(Ok(message)) = receiver.next().await {
        if message.is_text() {
            let message_str = message.to_text().unwrap();
            if let Ok(command) = serde_json::from_str::<TrucoCommand>(message_str) {
                unbounded_sender
                    .send(Event::Input(id, command))
                    .ok();
            }
        }
    }
}
