use super::*;
use gstd::prelude::*;
use gstd::msg;

#[test]
fn test_game_flow() {
    let mut game = GameState {
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 15,
        difficulty: DifficultyLevel::Easy,
        first_player: Player::User,
        winner: None,
    };

    // User takes 2 pebbles
    assert!(game.take_turn(2).is_ok());
    assert_eq!(game.pebbles_remaining, 13);

    // Program takes 3 pebbles (simulated)
    assert!(game.take_turn(3).is_ok());
    assert_eq!(game.pebbles_remaining, 10);
}

#[test]
fn test_invalid_move() {
    let mut game = GameState {
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 15,
        difficulty: DifficultyLevel::Easy,
        first_player: Player::User,
        winner: None,
    };

    // User tries to take 4 pebbles, which is invalid
    assert!(game.take_turn(4).is_err());
    assert_eq!(game.pebbles_remaining, 15);
}
