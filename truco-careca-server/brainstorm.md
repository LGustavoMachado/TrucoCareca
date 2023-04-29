# States

## Waiting for players
- Wait for 4 players to connect

## Waiting for ready
- Waits for the players to submit their names

## Pick up seats
- Wait for players to pick up their seats

## Start Game
- Resets the score
- Resets or random the dealer (who is the first player to play) 
- Change state to StartRoundState

## Start round (point cycle)
- Resets the turn
- Resets the table
- Resets players hands
- Resets round_value to 1
- Resets give_up_players
- Create a new deck
- Shuffle
- Cut the deck
- Deal the cards
- Turn the manilha
- If one team has 11 points change to MatchPointState
- If two teams have 11 points sets the blind mode
- Send the state of the game to player

## Match Point
- Show the winning team cards to each other
- Wait for the winning team to chose if they will play the round
- If they decide to give up start a new round
- If they decide to play go to StartTurnState

## Start turn
- Sets the head to the winner of the last turn or the dealer
- Change to PlayerTurnState

## Player turn
- Player can discard a card
- Player can discard a card blind (if it's not the first round)
- Players can truco (change to TrucoState) if the round_value is lower than 12
- Player can give up the round
- if the player calls truco with 11 points ends the game
- if it's the last player change state to FinishRoundState 

## Truco state
- Trucoed team can accept the truco
  - If the round value is 1 sets the round value to 3 else sum 3
  - Change to PlayerTurnState
- Trucoed team can give up
  - Give up adds round value to the winning team
  - Change to StartRoundState
- Trucoed team can re-truco (if the truco is not currently 12)
  - Runs the accept truco fn
  - Change the TrucoedTeam
  
## End turn
- Calculate who won the round (returns the winner player)
- Adds 1 to the winner team round_score (best of 3)
- Do something if draw
- TODO: WRITE RULES TO CHECK IF THE ROUND FINISHED (if so change to FinishRound)

## Finish round
- Calculate who won


## State 1
  ## State 2
    ## State 3

```
{
  game_state: {
    player_hand: [(3,hearts)],
    player_1: {
      cards: N,
      status: in
    }
    table: [(2,clubs), (1,diamonds)],
    player_turn: 1,
    score: [4, 11],
    round_value: 3,
  },
  state: {
    name: "trucoed"
    trucoed_team: 1,
  }
}
```