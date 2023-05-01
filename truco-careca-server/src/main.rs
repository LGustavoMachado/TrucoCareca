use std::net::SocketAddr;
use std::{thread, time};
use std::io::{Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::unconstrained;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt, FutureExt};
use queues::IsQueue;

mod game;
use game::game_event::{get_event_from_command};
use game::connection::Connection;
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

    let sixteen_ms = time::Duration::from_millis(100);
    let mut delta_time: f32 = 0.0;

    // Game loop
    loop {
        let start_time = time::Instant::now();

        print!(".");
        std::io::stdout().flush().unwrap();

        // Capture all new events
        while let Some(event) = unconstrained(game_receiver.recv()).now_or_never() {
            match event {
                Some(Event::Join(id, connection)) => {
                    println!("PLAYER {} connected!", id);
                    game_instance.add_player(id, connection).unwrap();
                }
                Some(Event::Input(id, command)) => {
                    print!("USER INPUT RECEIVED {:#?}", command);
                    let game_event = get_event_from_command(id, command).unwrap();
                    game_instance.input(game_event);
                }
                _ => {}
            }
        }

        // Game update
        state_machine.update(&mut game_instance, delta_time);

        // Game output
        while let Ok((id, message)) = game_instance.output_mut().remove() {
            let (connection, _) = game_instance.get_player_mut(id).unwrap();
            match connection.sender.send(Message::Text(message.clone())).await {
                Ok(_) => { println!("Message sent successfully {}", message) }
                Err(_) => {}
            }
        }

        // Game state
        let ids: Vec<u32> = game_instance.get_players().keys().cloned().collect();
        for id in ids {
            let message = state_machine.state_out(&game_instance);
            let (connection, _) = game_instance.get_player_mut(id).unwrap();
            match connection.sender.send(Message::Text(message)).await {
                Ok(_) => { }
                Err(_) => {}
            }
        }

        // sleep n
        thread::sleep(sixteen_ms);
        delta_time = start_time.elapsed().as_secs_f32();
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

    unbounded_sender.send(Event::Join(id, conn)).unwrap();

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
