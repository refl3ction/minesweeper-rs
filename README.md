# Minesweeper

## Getting Started

```bash
cargo run
```

## Notes for myself

### Entities

#### Game

- board: Board
- game_difficult: GameDifficult
- lost: bool
- open_field(pos: Position)
- create_board()
- populate_mines()

#### Board

- new(width, height, mine_count)
- open_fields_pos
- mine_fields_pos
- flag_fields_pos
- get_neighbors() -> Vec<Position>
- count_neighboring_mines()
- is_open()
- is_mine()
- is_flag()

### Use Cases

#### open field

- The field can't be flagged
- if not mine:
  - Count all the mines surrounding the field
  - If count is 0, count all the mines surrounding the neighbors
- else
  - Game is lost

## References

- <https://www.geeksforgeeks.org/cpp-implementation-minesweeper-game/>
