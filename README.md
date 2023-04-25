# TrucoCareca

TrucoCareca is a truco game made from 3 bald guys that wanted to learn Rust programming language, websockets and game development at the same time.

# Prerequisites

As a prerequisite to build and run this project you will need to install some components. Above we have the recomended ones but feel free to choose any other IDE that meets the requirements to work with a Rust project.

## Rust

### Installing with rustup

This project is being build using Rust lang so you will need to install rustc bin. 

To do it you can follow this [instructions](https://rustup.rs/).

### Installing with asdf (version manager)

Download asdf
```bash
cd ~/
git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.8.1`
```

Set up the rust plugin for asdf
```bash
asdf plugin-add rust https://github.com/asdf-community/asdf-rust.git
asdf install rust <version>
asdf global rust <version>
```


# Optional steps

## VSCode

VSCode is a editor that allow us to install pretty nice extensions to boost our coding. You can install VSCode downloading it [here](https://code.visualstudio.com/)

## VSCode extensions

### Rust-analyzer

This extension provides support for the Rust programming language in vscode and you can install directly from vscode extensions.

### Rusty sintax

This extension provides a TextMate grammar for Rust and is pretty nice to keep the project well frmatted. It can be found and install directly from vscode extensions.


# Websocket

To connect to the project and test it without a frontend you can use postman websockets (beta). You can found postman to install [here](https://www.postman.com/downloads/).

To create a new websocket you can tap new and select create a new websocket request, as the images above:


<img width="1511" alt="Screenshot 2023-04-25 at 12 14 44" src="https://user-images.githubusercontent.com/8165641/234323698-8375198f-8f3e-4c42-9837-15f1c46f4bd9.png">

<img width="1510" alt="Screenshot 2023-04-25 at 12 17 32" src="https://user-images.githubusercontent.com/8165641/234323989-4c5866d1-4cdb-4852-8de1-5d99901cd0be.png">


# Build and run

To build and run the project you just need to run the command `cargo run` on terminal (you can use the vscode terminal by ctrl+\`).

To run the project tests you can run the command `cargo test` on terminal.
