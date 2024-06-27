```shell
cargo new --lib pebbles-game 

cargo new --lib io
```

游戏规则：

1. **玩家**：游戏有两个玩家：
    - **用户**：即你，真人玩家。
    - **程序**：电脑对手。

2. **初始设置**：
    - 游戏开始时有 \( N \) 个石子。示例中，\( N \) 为 15。

3. **游戏玩法**：
    - 第一个玩家随机选择。
    - 在每个玩家的回合，他们必须从总数中移除 1 到 \( K \) 个石子。例如，如果 \( K \) 为 2，则玩家每次可以移除 1 或 2 个石子。

4. **获胜条件**：
    - 拿到最后一个（或多个）石子的玩家获胜。

参考文档
https://docs.gear.rs/gstd/index.html
https://wiki.gear-tech.io/docs/examples/Gaming/rock-paper-scissors

## Project Structure

pebbles-game
    ├── io
    │   ├── src
    │   │   └── lib.rs
    │   └── Cargo.toml
    ├── src
    │   └── lib.rs
    ├── tests
    │   └── basic.rs
    ├── Cargo.lock
    ├── Cargo.toml
    └── build.rs

pebbles-game/src/lib.rs
```
#![no_std]

use gstd::{msg, prelude::*};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
}

#[no_mangle]
extern "C" fn handle() {

}

#[no_mangle]
extern "C" fn handle_reply(){
}

#[no_mangle]
extern "C" fn state() {

}
```
pebbles-game/io/src/lib.rs
```
#![no_std]

use gmeta::{In, InOut, Out, Metadata};
use gstd::prelude::*;

// Metadata for the Pebbles game.
pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    // Initialization message type.
    type Init = In<PebblesInit>;
    // Handle message type, which includes both input and output.
    type Handle = InOut<PebblesAction, PebblesEvent>;
    // State message type.
    type State = Out<GameState>;
    // Reply message type.
    type Reply = ();
    // Others message type.
    type Others = ();
    // Signal message type.
    type Signal = ();
}

// Initialization structure for the Pebbles game.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    // Difficulty level of the game.
    pub difficulty: DifficultyLevel,
    // Total number of pebbles in the game.
    pub pebbles_count: u32,
    // Maximum number of pebbles that can be taken per turn.
    pub max_pebbles_per_turn: u32,
}

// Difficulty levels for the Pebbles game.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum DifficultyLevel {
    // Easy difficulty level.
    #[default]
    Easy,
    // Hard difficulty level.
    Hard,
}

// Actions that can be taken in the Pebbles game.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    // Take a turn by removing a specified number of pebbles.
    Turn(u32),
    // Give up the game.
    GiveUp,
    // Restart the game with specified parameters.
    Restart {
        // New difficulty level.
        difficulty: DifficultyLevel,
        // New total number of pebbles.
        pebbles_count: u32,
        // New maximum number of pebbles per turn.
        max_pebbles_per_turn: u32,
    },
}

// Events that can occur in the Pebbles game.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    // Counter's turn with the number of pebbles taken.
    CounterTurn(u32),
    // Game won by a player.
    Won(Player),
}

// Players in the Pebbles game.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum Player {
    // User player.
    #[default]
    User,
    // Program player.
    Program,
}

// State of the Pebbles game.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    // Total number of pebbles in the game.
    pub pebbles_count: u32,
    // Maximum number of pebbles that can be taken per turn.
    pub max_pebbles_per_turn: u32,
    // Number of pebbles remaining in the game.
    pub pebbles_remaining: u32,
    // Current difficulty level of the game.
    pub difficulty: DifficultyLevel,
    // Player who goes first.
    pub first_player: Player,
    // Winner of the game, if any.
    pub winner: Option<Player>,
}
```

pebbles-game/tests/basic.rs
```cookie

```