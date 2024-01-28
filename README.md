# yew-nim-game

A fundamental web assembly app using rust and yew.rs implementing minimax algorithm for counter moves.

## Game Logic

- Upon starting the game, you'll be presented with a row of Ferris crabs whose count varies between 15-21.
- The game is played between user, who represents the player, and computer acting as the opponent.
- User starts off by removing one or two crabs from the row as a beginning.
- Afterward, computer counters the player's move using the minimax algorithm.
- The game keeps continuing until a player removes the last crab(s), and the one who accomplishes this is declared the winner.

## Screenshots

![screenshot_1](https://github.com/DrShahinstein/yew-nim-game/assets/81323808/76ea425e-adb2-4d8e-ba24-49589b72580d)
![screenshot_2](https://github.com/DrShahinstein/yew-nim-game/assets/81323808/ab9df26e-b051-4ad7-b3ad-28c861b12a57)
![screenshot_3](https://github.com/DrShahinstein/yew-nim-game/assets/81323808/efb43292-904b-4e23-8015-4ee912d8dd85)
![screenshot_4](https://github.com/DrShahinstein/yew-nim-game/assets/81323808/e749add7-ce30-4444-b159-f7cf2b9dc494)

## Installation

### Required

- [trunk](https://trunkrs.dev/)

1. `$ git clone https://github.com/DrShahinstein/yew-nim-game.git`
2. `[yew-nim-game]$ trunk serve`
