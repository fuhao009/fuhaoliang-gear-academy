use gstd::{msg, prelude::*};
use gtest::{Program, System};
use pebbles_game_io::{DifficultyLevel, GameState, PebblesAction, PebblesInit};

#[test]
fn test_pebbles_game() {
    let system = System::new();
    let program = Program::current(&system);

    // Initialize the game with 15 pebbles, max 3 pebbles per turn, and easy difficulty
    program.send(
        1,
        PebblesInit {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 15,
            max_pebbles_per_turn: 3,
        },
    );
}
