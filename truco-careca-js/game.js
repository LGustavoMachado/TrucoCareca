import { GameState } from "./types.js";
import WaitingForReadyState from "./states/waiting_for_ready.js";
import PickUpSeatsState from "./states/pick_up_seats.js";
import WaitingForPlayersState from "./states/waiting_for_players.js";

export default class Game {

  constructor(conn, container) {
    this.state = new WaitingForPlayersState();
    this.conn = conn;
    this.container = container;
    this.gameContainer = container.getElementsByClassName("game")[0];
    this.playerConsole = container.getElementsByClassName("console")[0];
    this.seats = [];
    this.data = {}
  }

  onMessage(message){
    try {
      let data = JSON.parse(message.data);

      if(data.state) {
        // handle state
        // {
        //   state: {
        //     name: "state-name",
        //     body: {
        //       player_ready: true,
        //       ...
        //     }
        //   }
        // }
        this.handleState(data);
      }

      if(data.message) {
        // handle messages
        // {
        //   message: {
        //     type: "test",
        //     body: {
        //       ...
        //     }
        //   }
        // }
        this.handleMessage(data.message);
      }
    } catch(e) {
      return;
    }
  }

  handleState(data) {
    if(this.state.name !== data.state.name) {
      this.log('Changing state to ' + data.state.name);
      switch(data.state.name) {
        case "waiting-for-ready":
          this.state = new WaitingForReadyState(this);
          break;
        case "pick-up-seats":
          this.state = new PickUpSeatsState(this);
          break;
        case "game-started":
          // this.state = new GameStartedState(this);
          break;
        default:
          console.log("Unknown state");
          return;
      }
      this.state.init();
    }
    this.data = data?.state?.body ?? {};
  }

  handleMessage(message) {

  }

  log(message) {
    const newMessage = document.createElement("p");
    newMessage.innerHTML = message;
    this.playerConsole.appendChild(newMessage);
  }

  clearGame() {
    this.gameContainer.innerHTML = "";
  }
  
  update(time) {
    this.state.update(this.data, time);
  }
}
