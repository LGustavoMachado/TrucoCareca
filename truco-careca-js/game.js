import { GameState } from "./types.js";

export default class Game {

  constructor(conn, container) {
    this.conn = conn;
    this.container = container;
    this.gameContainer = container.getElementsByClassName("game")[0];
    this.playerConsole = container.getElementsByClassName("console")[0];
    this.seats = [];
  }

  showReadyForm() {
    const form = document.createElement("form");
    const button = document.createElement("button");
    button.innerText = "Submit";

    const input = document.createElement("input");
    input.type = "text";
    input.placeholder = "enter your name";

    form.addEventListener('submit', (event) => {
      event.preventDefault();
      const data = {
        name: "player-ready",
        body: {
          name: input.value
        }
      };
      this.conn.send(JSON.stringify(data));

    });

    this.clearGame();
    form.appendChild(input);
    form.appendChild(button);
    this.gameContainer.appendChild(form);
  }

  showWaitingForOtherPlayers() {
    const waitingMessage = document.createElement("p");
    waitingMessage.innerHTML = "Waiting for other players...";
    this.gameContainer.appendChild(waitingMessage);
  }

  showPickupSeats() {
    const startGameButton = document.createElement("button");
    const leaveSeatButton = document.createElement("button");
    startGameButton.innerText = "Start Game";
    startGameButton.classList.add("start-game");
    leaveSeatButton.innerText = "Leave Seat";
    leaveSeatButton.classList.add("leave-seat");

    startGameButton.addEventListener('click', () => {
      const data = {
        name: "start-the-game",
        body: {}
      };
      this.conn.send(JSON.stringify(data));
    });

    leaveSeatButton.addEventListener('click', () => {
      const data = {
        name: "leave-seat",
        body: {}
      };
      this.conn.send(JSON.stringify(data));
    });

    for(let i=0; i<this.seats.length; i++) {
      const seat = document.createElement("div");
      seat.classList.add("seat");

      const player = this.seats[i];

      if(player) {
        seat.classList.add("player-seated");
        seat.innerText = player.name;
      }

      seat.addEventListener('click', () => {
        const data = {
          name: "pick-up-seat",
          body: {
            seat: i
          }
        };
        this.conn.send(JSON.stringify(data));
      });

      this.gameContainer.appendChild(seat);
      this.gameContainer.appendChild(startGameButton);
      this.gameContainer.appendChild(leaveSeatButton);
    }
  }

  onMessage(message){
    if (message.data === "WAITING FOR READY STATE") {
      this.log(message.data);
      this.showReadyForm();
      return;
    }

    if(message.data === "Ready!") {
      this.log(message.data);
      this.clearGame();
      this.showWaitingForOtherPlayers();
      return;
    }

    if(message.data === "PICK UP SEATS STATE") {
      this.log(message.data);
      this.state = GameState.PICKUP_SEATS;
      return;
    }
    
    if(message.data === "START GAME STATE") {
      this.log(message.data);
      this.state = GameState.GAME_STARTED;
      this.clearGame();
      return;
    }


    if(this.state === GameState.PICKUP_SEATS) {
      try {
        let data = JSON.parse(message.data);
        if(data.seats) {
          if (eq(this.seats, data.seats)) return;
          this.seats = data.seats;
          this.clearGame();
          this.showPickupSeats();
        }
      } finally {
        return;
      }
    }
  }

  log(message) {
    const newMessage = document.createElement("p");
    newMessage.innerHTML = message;
    this.playerConsole.appendChild(newMessage);
  }

  clearGame() {
    this.gameContainer.innerHTML = "";
  }
}

function eq(a, b) {
  return Object.entries(a).toString() === Object.entries(b).toString();
}
