import { eq } from "../utils.js";

export default class PickingUpSeatsState {

  constructor(game){
    this.name = "pick-up-seats";
    this.game = game;
    this.container = game.gameContainer;
    this.conn = game.conn;
    this.player_ready = false;
  }

  init() {
    this.game.clearGame();
  }

  update(data, time){

    if(data.seats && eq(this.game.seats, data.seats)) {
      return;
    }

    this.game.clearGame();
    this.game.seats = data.seats;

    this.drawSeats();
    this.drawButtons();
  }

  drawSeats() {
    for(let i=0; i<this.game.seats.length; i++) {
      const seat = document.createElement("div");
      seat.classList.add("seat");

      const player = this.game.seats[i];

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

      this.container.appendChild(seat);
    }
  }

  drawButtons() {

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

    this.container.appendChild(startGameButton);
    this.container.appendChild(leaveSeatButton);
  }
}