export default class WaitingForPlayersState {

  constructor(game) {
    this.name = "waiting-for-players";
    this.game = game;
  }

  init() {
    const waitingMessage = document.createElement("p");
    waitingMessage.innerHTML = "Waiting for other players...";
    this.gameContainer.appendChild(waitingMessage);
  }

  update(time) {

  }
}