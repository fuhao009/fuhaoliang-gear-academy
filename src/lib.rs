#![no_std]

use gstd::{msg, prelude::*};
use pebbles_game_io::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    let init_message: PebblesInit = msg::load().expect("Unable to load init message");
    let mut rng = ChaChaRng::seed_from_u64(0);  // Replace 0 with a seed value if needed

    let game_state = GameState {
        pebbles_count: init_message.pebbles_count,
        max_pebbles_per_turn: init_message.max_pebbles_per_turn,
        pebbles_remaining: init_message.pebbles_count,
        difficulty: init_message.difficulty,
        first_player: if rng.gen_bool(0.5) { Player::User } else { Player::Program },
        winner: None,
    };
    unsafe { PEBBLES_GAME = Some(game_state) };
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to load message");

    let game_state = unsafe { PEBBLES_GAME.as_mut().expect("Game is not initialized") };
    let mut rng = ChaChaRng::seed_from_u64(0);  // Replace 0 with a seed value if needed

    match action {
        PebblesAction::Turn(pebbles) => {
            if pebbles < 1 || pebbles > game_state.max_pebbles_per_turn {
                msg::reply(PebblesEvent::InvalidTurn, 0).expect("Unable to reply");
                return;
            }

            if pebbles > game_state.pebbles_remaining {
                msg::reply(PebblesEvent::InvalidTurn, 0).expect("Unable to reply");
                return;
            }

            game_state.pebbles_remaining -= pebbles;

            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::User);  // Assume current turn is user's
                msg::reply(PebblesEvent::Won(Player::User), 0).expect("Unable to reply");
            } else {
                // Simulate program's turn
                let program_pebbles = rng.gen_range(1..=game_state.max_pebbles_per_turn).min(game_state.pebbles_remaining);
                game_state.pebbles_remaining -= program_pebbles;

                msg::reply(PebblesEvent::CounterTurn(program_pebbles), 0).expect("Unable to reply");

                if game_state.pebbles_remaining == 0 {
                    game_state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
                }
            }
        }
        PebblesAction::GiveUp => {
            game_state.winner = Some(Player::Program);
            msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            game_state.pebbles_count = pebbles_count;
            game_state.max_pebbles_per_turn = max_pebbles_per_turn;
            game_state.pebbles_remaining = pebbles_count;
            game_state.difficulty = difficulty;
            game_state.first_player = if rng.gen_bool(0.5) { Player::User } else { Player::Program };
            game_state.winner = None;
        }
    }
}

#[no_mangle]
extern "C" fn handle_reply() {
    // Handle replies if necessary
}

#[no_mangle]
extern "C" fn state() {
    let game_state = unsafe { PEBBLES_GAME.as_ref().expect("Game is not initialized") };
    msg::reply(game_state, 0).expect("Unable to reply");
}
