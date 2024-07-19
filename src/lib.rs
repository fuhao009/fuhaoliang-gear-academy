#![no_std]

use gstd::{msg, prelude::*};
use pebbles_game_io::*;
use exec::random;
use gstd::exec;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    let init_message: PebblesInit = msg::load().expect("Unable to load init message");
    let subject: [u8; 32] = array::from_fn(|i| i as u8 + 1);
    // 玩家和机器人随机首发
    let first_player = match random(subject) {
        Ok((_, num)) => {
            if num % 2 == 0 {
                Player::User
            } else {
                Player::Program
            }
        }
        Err(_) => {
            // 处理错误，例如默认选择某个玩家
            Player::Program
        }
    };
    let game_state = GameState {
        pebbles_count: init_message.pebbles_count,
        max_pebbles_per_turn: init_message.max_pebbles_per_turn,
        pebbles_remaining: init_message.pebbles_count,
        // 游戏困难度
        difficulty: init_message.difficulty,
        first_player,
        winner: None,
    };

    unsafe { PEBBLES_GAME = Some(game_state) };
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to load message");

    fn get_random_u32(min_value: Option<u32>, max_value: Option<u32>, game_state: &GameState) -> u32 {
        let salt = msg::id();
        let (hash, _num) = exec::random(salt.into()).expect("get_random_u32_in_range(): random call failed");
        let random_number = u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]);

        let base_random = match (min_value, max_value) {
            (Some(min), Some(max)) => (random_number % (max - min + 1)) + min,
            (Some(min), None) => random_number.max(min),
            (None, Some(max)) => random_number % (max + 1),
            (None, None) => random_number,
        };

        match game_state.difficulty {
            DifficultyLevel::Hard => {
                let max_pebbles_plus_one = game_state.max_pebbles_per_turn + 1;
                let candidate_value = ((base_random / max_pebbles_plus_one) + 1) * max_pebbles_plus_one;

                // Ensure candidate_value is within the provided range
                if let (Some(min), Some(max)) = (min_value, max_value) {
                    if candidate_value >= min && candidate_value <= max {
                        return candidate_value;
                    }
                } else if let Some(min) = min_value {
                    if candidate_value >= min {
                        return candidate_value;
                    }
                } else if let Some(max) = max_value {
                    if candidate_value <= max {
                        return candidate_value;
                    }
                }
                base_random
            },
            DifficultyLevel::Easy => base_random,
        }
    }


    let game_state = unsafe { PEBBLES_GAME.as_mut().expect("Game is not initialized") };

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
                return;
            }

            // Simulate program's turn
            let program_pebbles = match game_state.difficulty {
                DifficultyLevel::Easy => get_random_u32(Some(1), Some(game_state.max_pebbles_per_turn),game_state),
                DifficultyLevel::Hard => {
                    // let k = game_state.max_pebbles_per_turn; // Example value for K, this should be adjusted based on game design
                    // let mut program_pebbles = game_state.pebbles_remaining % (k + 1);

                    let mut program_pebbles = get_random_u32(Some(1), Some(game_state.max_pebbles_per_turn),game_state);

                    // Ensure that the program_pebbles is within the valid range and reduces the pebbles_remaining
                    if program_pebbles > game_state.pebbles_remaining {
                        program_pebbles = game_state.pebbles_remaining
                    }
                    program_pebbles
                }
            };
            game_state.pebbles_remaining -= program_pebbles;

            msg::reply(PebblesEvent::CounterTurn(program_pebbles), 0).expect("Unable to reply");

            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::Program);
                msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to reply");
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
            let subject: [u8; 32] = array::from_fn(|i| i as u8 + 1);
            game_state.first_player = match random(subject) {
                Ok((_, num)) => {
                    if num % 2 == 0 {
                        Player::User
                    } else {
                        Player::Program
                    }
                }
                Err(_) => {
                    // 处理错误，例如默认选择某个玩家
                    Player::Program
                }
            };
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


