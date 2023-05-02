
export default class WaitingForReadyState {

  constructor(game){
    this.name = "waiting-for-ready";
    this.game = game;
    this.container = game.gameContainer;
    this.conn = game.conn;
    this.player_ready = false;
  }

  init() {
    this.game.clearGame();

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

    form.appendChild(input);
    form.appendChild(button);
    this.container.appendChild(form);
  }

  update(data, time){
    if(data.player_ready && !this.player_ready) {
      this.player_ready = true;
      this.game.clearGame();
      let message = document.createElement("p");
      message.innerHTML = "Waiting for other players...";
      this.container.appendChild(message);

      // Log to console
      this.game.log("Player ready!");
    }
  }
}