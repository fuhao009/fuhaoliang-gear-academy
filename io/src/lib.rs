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
