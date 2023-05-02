"use strict";

import Game from './game.js';

window.addEventListener("load", function () {

    let connections = [
        new WebSocket("ws://127.0.0.1:8080"),
        new WebSocket("ws://127.0.0.1:8080"),
        new WebSocket("ws://127.0.0.1:8080"),
        new WebSocket("ws://127.0.0.1:8080")
    ];

    const players = document.getElementsByClassName("player");

    for(let i=0; i<connections.length; i++) {
        const conn = connections[i];
        const game = new Game(conn, players[i]);

        conn.addEventListener('open', () => {
            game.log('Connection established');
        });

        conn.addEventListener('message', (event) => {
            game.onMessage(event);
        });

        const gameLoop = (tFrame) => {
            game.update(tFrame);
            window.requestAnimationFrame(gameLoop);
        };
    
        gameLoop(); // Start the cycle
    }
});